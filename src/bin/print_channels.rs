#[macro_use]
extern crate serde_json;
use clap::{App, Arg};
use rswinthings::utils::cli::{add_session_options_to_app, get_session_from_matches};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winevt::channels::get_channel_name_list;
use rswinthings::winevt::channels::ChannelConfig;
use std::process::exit;

static VERSION: &'static str = "0.2.0";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let format = Arg::with_name("format")
        .short("-f")
        .long("format")
        .value_name("FORMAT")
        .takes_value(true)
        .possible_values(&["text", "jsonl"])
        .help("Output format. (defaults to text)");

    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    let app = App::new("print_channels")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("Print Channel Propperties.")
        .arg(format)
        .arg(debug);

    // Add session arguments to app
    add_session_options_to_app(app)
}

fn print_text_value(name: &str, config_value: serde_json::Value) {
    let config_map = config_value
        .as_object()
        .expect("config_value should be a mapping.");

    println!("========================================================");
    println!("Channel: {}", name);
    println!("========================================================");
    for (key, value) in config_map {
        println!("{}: {}", key, value);
    }
    println!("");
}

fn print_jsonl_value(config_value: serde_json::Value) {
    println!("{}", config_value.to_string());
}

fn main() {
    let app = make_app();
    let options = app.get_matches();

    // Set debug
    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => {}
    }

    let out_format = match options.value_of("format") {
        Some(f) => f,
        None => "text",
    };

    let session =
        match get_session_from_matches(&options).expect("Error getting session from options") {
            Some(s) => Some(s.0),
            None => None,
        };

    // Get list of channel names
    let channels = get_channel_name_list(&session).expect("Could not get channel name list");

    for channel in channels {
        // Get the channel config for this channel name
        let channel_config = match ChannelConfig::new(channel.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error creating ChannelConfig for: {:?}", e);
                continue;
            }
        };

        let mut channel_config_value = match channel_config.to_json_value() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error getting channel config. {:?}", e);
                continue;
            }
        };

        match out_format {
            "text" => print_text_value(&channel, channel_config_value),
            "jsonl" => {
                channel_config_value["ChannelName"] = json!(channel.to_owned());
                print_jsonl_value(channel_config_value);
            }
            other => {
                eprintln!("Unhandled output format: {}", other);
                exit(-1);
            }
        }
    }
}
