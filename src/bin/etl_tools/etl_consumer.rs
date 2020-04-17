extern crate chrono;
extern crate clap;
extern crate log;
extern crate serde_json;
use clap::{App, Arg};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winetl::consumer::{
    EventTraceLogFile
};
use std::process::exit;

static VERSION: &'static str = "0.1.0";
static DESCRIPTION: &'static str = r"
Consume Events from a ETL source.
";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let logfile_arg = Arg::with_name("logfile")
        .long("logfile")
        .value_name("LOGFILE")
        .takes_value(true)
        .required_unless("logger")
        .help("A logfile to consume");

    let logger_arg = Arg::with_name("logger")
        .long("logger")
        .value_name("LOGGER")
        .takes_value(true)
        .help("A real-time logger to consume");

    let debug_arg = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("etl_consumer")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about(DESCRIPTION)
        .arg(logfile_arg)
        .arg(logger_arg)
        .arg(debug_arg)
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => set_debug_level("Error").expect("Error setting debug level"),
    }

    let event_trace_log = match options.value_of("logfile") {
        Some(logfile) => {
            EventTraceLogFile::from_logfile(logfile)
        },
        None => {
            match options.value_of("logger") {
                Some(logger) => {
                    EventTraceLogFile::from_logger(logger)
                        .is_kernel_trace(0)
                },
                None => {
                    eprintln!("No logfile or logger was specified");
                    exit(-1);
                }
            }
        }
    };
    let consumer = event_trace_log.get_consumer();
    consumer.process_trace().expect("Error processing trace");
}
