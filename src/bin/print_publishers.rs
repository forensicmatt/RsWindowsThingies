#[macro_use] extern crate serde_json;
use clap::{App, Arg};
use std::process::exit;
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winetl::publisher::PublisherEnumerator;

static VERSION: &'static str = "0.1.0";


fn make_app<'a, 'b>() -> App<'a, 'b> {
    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("print_publishers")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("Print Publisher Propperties.")
        .arg(debug)
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect(
            "Error setting debug level"
        ),
        None => {}
    }

    let enumerator = PublisherEnumerator::new(None)
        .expect("Error creating PublisherEnumerator");

    for publisher_meta in enumerator {
        let meta_value = publisher_meta.to_json_value().unwrap();
        println!("Publisher: {}", publisher_meta.name);
        println!("{}", meta_value.to_string());
    }
}