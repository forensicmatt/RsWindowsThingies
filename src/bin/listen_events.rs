extern crate chrono;
extern crate clap;
extern crate log;
extern crate serde_json;
use clap::{App, Arg};
use rswinthings::handler::WindowsHandler;
use rswinthings::utils::cli::{add_session_options_to_app, get_session_from_matches};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winevt::callback::OutputFormat;
use rswinthings::winevt::channels::get_channel_name_list;
use rswinthings::winevt::channels::ChannelConfig;
use rswinthings::winevt::EvtHandle;
use std::process::exit;

static VERSION: &'static str = "0.3.0";
static DESCRIPTION: &'static str = r"
Event listener written in Rust. Output is JSONL.

This tool queries the available list of channels then creates a XPath
query and uses the Windows API to monitor for events on the applicable 
channels. Use the print_channels tool to list available channels and
their configurations.
";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let channel = Arg::with_name("channel")
        .short("-c")
        .long("channel")
        .value_name("CHANNEL")
        .multiple(true)
        .takes_value(true)
        .help("Specific Channel to listen to.");

    let format = Arg::with_name("format")
        .short("-f")
        .long("format")
        .value_name("FORMAT")
        .takes_value(true)
        .possible_values(&["xml", "jsonl"])
        .help("Output format to use. [defaults to jsonl]");

    let historical = Arg::with_name("historical")
        .short("p")
        .long("historical")
        .help("List historical records along with listening to new changes.");

    let named_pipe = Arg::with_name("named_pipe")
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

    let app = App::new("listen_events")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about(DESCRIPTION)
        .arg(channel)
        .arg(format)
        .arg(historical)
        .arg(named_pipe)
        .arg(debug);

    // Add session arguments to app
    add_session_options_to_app(app)
}

fn get_list_from_system(session: &Option<EvtHandle>) -> Vec<String> {
    let mut channels = Vec::new();

    // Get a list off all the channels
    let channel_list = get_channel_name_list(&session).expect("Error getting channel list");

    // Iterate each channel in our available channels
    for channel in channel_list {
        // Get the config for this channel
        let channel_config = match ChannelConfig::new(channel.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error getting ChannelConfig for {}: {:?}", channel, e);
                continue;
            }
        };

        // Cutting out config types of 2 or more seems to resolve
        // observed Subscription issues
        if !channel_config.can_subscribe() {
            continue;
        }

        // We can only monitor channels that are enabled and are classic event log channels
        if !channel_config.is_enabled() {
            continue;
        }

        eprintln!("Adding {} to listener", channel);
        channels.push(channel);
    }

    channels
}

fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => set_debug_level("Error").expect("Error setting debug level"),
    }

    // Get Session
    let session: Option<EvtHandle> = get_session_from_matches(&options)
        .expect("Error getting session from options")
        .map(|sess| sess.into_handle());

    let format_enum = match options.value_of("format") {
        Some(f) => match f {
            "xml" => OutputFormat::XmlFormat,
            "jsonl" => OutputFormat::JsonFormat,
            other => {
                eprintln!("Unkown format: {}", other);
                exit(-1);
            }
        },
        None => OutputFormat::JsonFormat,
    };

    // Historical flag
    let historical_flag = options.is_present("historical");

    let channel_list = match options.values_of("channel") {
        Some(ch_str_list) => {
            let mut list = Vec::new();
            for ch_str in ch_str_list {
                list.push(ch_str.to_string())
            }
            list
        }
        None => get_list_from_system(&session),
    };

    let handler = WindowsHandler::new();
    let reciever = handler
        .listen_events(session, historical_flag, format_enum, channel_list)
        .expect("Error creating listener");

    loop {
        for event in reciever.recv() {
            println!("{}", event.to_string());
        }
    }
}
