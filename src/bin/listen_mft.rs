extern crate serde_json;
use clap::{App, Arg};
use rswinthings::file::pipe::create_pipe;
use rswinthings::handler::WindowsHandler;
use rswinthings::mft::EntryListener;
use rswinthings::usn::listener::UsnListenerConfig;
use rswinthings::utils::debug::set_debug_level;
use rswinthings::utils::json::get_difference_value;
use std::fs::File;
use std::io::Write;
use std::process::exit;

static VERSION: &'static str = "0.2.0";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let file_arg = Arg::with_name("file")
        .short("-f")
        .long("file")
        .value_name("FILE")
        .takes_value(true)
        .help("The file to difference.");

    let pretty_arg = Arg::with_name("pretty")
        .short("p")
        .long("pretty")
        .help("Use pretty json output.");

    let namedpipe_arg = Arg::with_name("named_pipe")
        .long("named_pipe")
        .value_name("NAMEDPIPE")
        .takes_value(true)
        .help("The named pipe to write out to.");

    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("listen_mft")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("See the differences in MFT attirbues.")
        .arg(file_arg)
        .arg(pretty_arg)
        .arg(namedpipe_arg)
        .arg(debug)
}

fn run(mut listener: EntryListener, mut named_pipe_opt: Option<File>) {
    let mut previous_value = listener
        .get_current_value()
        .expect("Unable to get current mft entry value");
    match named_pipe_opt {
        Some(ref mut fh) => {
            fh.write(&previous_value.to_string().into_bytes())
                .expect("Error writing value");
        }
        None => {
            println!("{}", previous_value.to_string());
        }
    }

    let volume_str = listener
        .get_volume_string()
        .expect("Error getting volume path.");

    let usn_config = UsnListenerConfig::new().enumerate_paths(false);
    let usn_listener = usn_config.get_listener(&volume_str);
    let usn_rx = usn_listener
        .listen_to_volume()
        .expect("Unable to listen to USN");

    loop {
        let usn_entry_value = match usn_rx.recv() {
            Ok(e) => e,
            Err(_) => panic!("Disconnected!"),
        };

        let entry = &usn_entry_value["file_reference"]["entry"];

        if entry != listener.entry_to_monitor {
            continue;
        }

        let current_value = listener
            .get_current_value()
            .expect("Unable to get current mft entry value");

        let difference_value = get_difference_value(&previous_value, &current_value);

        match difference_value.as_object() {
            None => continue,
            Some(o) => {
                if o.is_empty() {
                    continue;
                }

                let value_str = serde_json::to_string_pretty(&difference_value)
                    .expect("Unable to format Value");

                match named_pipe_opt {
                    Some(ref mut fh) => {
                        fh.write(&format!("{}", value_str).into_bytes())
                            .expect("Unable to write value");
                    }
                    None => {
                        println!("{}", value_str);
                    }
                }

                previous_value = current_value.to_owned();
            }
        }
    }
}

fn main() {
    let app = make_app();
    let options = app.get_matches();

    // Set debug
    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => {}
    }

    let file_path = match options.value_of("file") {
        Some(p) => p,
        None => {
            eprintln!("file parameter was expected.");
            exit(-1);
        }
    };

    let _named_pipe = match options.value_of("named_pipe") {
        Some(p) => Some(create_pipe(p).expect("blahh")),
        None => None,
    };

    let pretty_flag = options.is_present("pretty");

    let handler = WindowsHandler::new();
    let reciever = handler
        .listen_mft(file_path)
        .expect("Error creating listener");

    loop {
        for value in reciever.recv() {
            let value_str = match pretty_flag {
                false => match serde_json::to_string(&value) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error creating string from value: {:?}", e);
                        continue;
                    }
                },
                true => match serde_json::to_string_pretty(&value) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error creating pretty string from value: {:?}", e);
                        continue;
                    }
                },
            };

            println!("{}", value_str);
        }
    }
}
