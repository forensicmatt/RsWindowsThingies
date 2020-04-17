extern crate chrono;
extern crate clap;
extern crate log;
extern crate serde_json;
use clap::{App, Arg};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winetl::trace::TraceSession;
use std::process::exit;

static VERSION: &'static str = "0.1.0";
static DESCRIPTION: &'static str = r"

";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let session = Arg::with_name("session")
        .short("-s")
        .long("session")
        .value_name("SESSION")
        .takes_value(true)
        .required(true)
        .help("The name to call this session. (Use 'NT Kernel Logger' for kernel logger)");

    let provider = Arg::with_name("provider")
        .short("-p")
        .long("provider")
        .value_name("PROVIDER")
        .multiple(true)
        .takes_value(true)
        .required(true)
        .help("Specific provider to listen to.");

    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("listen_events")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about(DESCRIPTION)
        .arg(session)
        .arg(provider)
        .arg(debug)
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => set_debug_level("Error").expect("Error setting debug level"),
    }

    let providers = options.values_of("provider")
        .expect("No provider/s provided")
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let session_name = options.value_of("session")
        .expect("No session provided")
        .to_string();

    let trace_session = TraceSession::new(
        session_name,
        providers
    ).expect("Error creating TraceSession");
}
