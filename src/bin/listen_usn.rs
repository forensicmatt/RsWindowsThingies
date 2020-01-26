use clap::{App, Arg};
use rswinthings::handler::WindowsHandler;
use rswinthings::usn::listener::UsnListenerConfig;
use rswinthings::utils::debug::set_debug_level;
use std::process::exit;

static VERSION: &'static str = "0.2.0";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let source_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .help("The source volume to listen to. (example: '\\\\.\\C:')")
        .required(true)
        .takes_value(true);

    let historical_arg = Arg::with_name("historical")
        .short("p")
        .long("historical")
        .help("List historical records along with listening to new changes.");

    let mask_arg = Arg::with_name("mask")
        .short("m")
        .long("mask")
        .value_name("MASK")
        .help("The USN mask for filtering.")
        .takes_value(true);

    let verbose = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("listen_usn")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyUsn>")
        .about("USN listener written in Rust. Output is JSONL.")
        .arg(source_arg)
        .arg(historical_arg)
        .arg(mask_arg)
        .arg(verbose)
}

fn main() {
    let app = make_app();
    let options = app.get_matches();

    // Set debug
    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => {}
    }

    let source_volume = match options.value_of("source") {
        Some(s) => s,
        None => {
            eprintln!("listen_usn requires a source volume.");
            exit(-1);
        }
    };

    let mask_opt = match options.value_of("mask") {
        Some(m) => {
            if m.starts_with("0x") {
                let without_prefix = m.trim_start_matches("0x");
                Some(u32::from_str_radix(without_prefix, 16).expect("Error converting mask to u32"))
            } else {
                Some(m.parse::<u32>().expect("Error converting mask to u32"))
            }
        }
        None => None,
    };

    let handler = WindowsHandler::new();
    let mut config = UsnListenerConfig::new();
    if options.is_present("historical") {
        config = config.historic(true);
    }
    match mask_opt {
        Some(m) => {
            config = config.mask(m);
        }
        None => {}
    }

    let reciever = handler
        .listen_usn(source_volume, Some(config))
        .expect("Error creating listener");

    loop {
        for event in reciever.recv() {
            println!("{}", event.to_string());
        }
    }
}
