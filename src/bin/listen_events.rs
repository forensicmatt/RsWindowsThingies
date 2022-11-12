extern crate chrono;
extern crate clap;
extern crate log;
extern crate serde_json;
use clap::{App, Arg};
use rswinthings::utils::cli::{add_session_options_to_app, get_session_from_matches};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winevt::callback::CallbackContext;
use rswinthings::winevt::callback::OutputFormat;
use rswinthings::winevt::channels::get_channel_name_list;
use rswinthings::winevt::channels::ChannelConfig;
use rswinthings::winevt::subscription::ChannelSubscription;
use rswinthings::winevt::EvtHandle;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winevt::{EvtSubscribeStartAtOldestRecord, EvtSubscribeToFutureEvents};

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
        .arg(debug);

    // Add session arguments to app
    add_session_options_to_app(app)
}

fn get_query_list_from_system(
    session: &Option<EvtHandle>,
    context: &CallbackContext,
    flags: Option<u32>,
) -> Vec<ChannelSubscription> {
    let mut subscriptions: Vec<ChannelSubscription> = Vec::new();
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

        eprintln!("listening to channel: {}", channel);

        // Create subscription
        let subscription =
            match ChannelSubscription::new(session, channel.to_string(), None, flags, &context) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error creating subscription for {}: {:?}", channel, e);
                    continue;
                }
            };

        subscriptions.push(subscription);
    }

    subscriptions
}

fn get_query_list_from_str_list<'a>(
    session: &Option<EvtHandle>,
    context: &CallbackContext,
    flags: Option<u32>,
    channel_list: Vec<&'a str>,
) -> Vec<ChannelSubscription> {
    let mut subscriptions: Vec<ChannelSubscription> = Vec::new();

    for channel in channel_list {
        // Create subscription
        let subscription =
            match ChannelSubscription::new(session, channel.to_string(), None, flags, &context) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error creating subscription for {}: {:?}", channel, e);
                    continue;
                }
            };

        subscriptions.push(subscription);
    }

    subscriptions
}

fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.value_of("debug") {
        Some(d) => set_debug_level(d).expect("Error setting debug level"),
        None => set_debug_level("Error").expect("Error setting debug level"),
    }

    // Get Session
    let session: Option<EvtHandle> =
        match get_session_from_matches(&options).expect("Error getting session from options") {
            Some(s) => Some(s.0),
            None => None,
        };

    let format_enum = match options.value_of("format") {
        Some(f) => match f {
            "xml" => OutputFormat::XmlFormat,
            "jsonl" => OutputFormat::JsonlFormat,
            other => {
                eprintln!("Unkown format: {}", other);
                exit(-1);
            }
        },
        None => OutputFormat::JsonlFormat,
    };

    // Historical flag
    let flags = match options.is_present("historical") {
        true => Some(EvtSubscribeStartAtOldestRecord),
        false => Some(EvtSubscribeToFutureEvents),
    };

    // Create context
    let context = CallbackContext::new().with_format(format_enum);

    let _subscritions = match options.values_of("channel") {
        Some(v_list) => get_query_list_from_str_list(&session, &context, flags, v_list.collect()),
        None => get_query_list_from_system(&session, &context, flags),
    };

    eprintln!("Listening to events...");
    loop {
        sleep(Duration::from_millis(200));
    }
}
