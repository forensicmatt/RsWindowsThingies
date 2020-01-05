extern crate serde_json;
use std::io::stdin;
use std::io::BufRead;
use clap::{App, Arg};
use std::process::exit;
use rswinthings::utils::debug::set_debug_level;
use rswinthings::mft::MftDifferencer;

static VERSION: &'static str = "0.2.0";


fn make_app<'a, 'b>() -> App<'a, 'b> {
    let format = Arg::with_name("file")
        .short("-f")
        .long("file")
        .value_name("FILE")
        .takes_value(true)
        .help("The file to difference.");

    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("difference_mft")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("See the differences in MFT attirbues.")
        .arg(format)
        .arg(debug)
}


fn run(mut differencer: MftDifferencer) {
    eprintln!("Hit enter to print snapshot.");

    loop {
        let mut line = String::new();
        let stdin_io = stdin();
        
        stdin_io.lock().read_line(
            &mut line
        ).expect("Could not read line");

        let value = differencer.get_current_value().expect("Unable to get current mft entry value");
        println!("{}", value.to_string());
    }
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    // Set debug
    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect(
            "Error setting debug level"
        ),
        None => {}
    }

    let file_path = match options.value_of("file") {
        Some(p) => p,
        None => {
            eprintln!("file parameter was expected.");
            exit(-1);
        }
    };

    let differencer = MftDifferencer::new(
        file_path
    ).expect("Error creating MftDifferencer");

    run(differencer);
}