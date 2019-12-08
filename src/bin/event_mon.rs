#[macro_use]
extern crate log;
extern crate clap;
extern crate chrono;
extern crate serde_json;
extern crate win_event_log;
use log::LevelFilter;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use win_event_log::prelude::*;
use clap::{App, Arg, ArgMatches};
use rswinthings::utils::xmltojson::xml_string_to_json;
use rswinthings::winevt::channels::get_channel_name_list;
use rswinthings::winevt::channels::ChannelConfig;

static VERSION: &'static str = "0.0.1";


fn make_app<'a, 'b>() -> App<'a, 'b> {
    let debug = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("event_mon")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("Event Monitor written in Rust. Output is JSONL.")
        .arg(debug)
}


fn set_debug_level(matches: &ArgMatches){
    // Get the possible logging level supplied by the user
    let message_level = match matches.is_present("debug") {
        true => {
            match matches.value_of("debug") {
                Some("Off") => LevelFilter::Off,
                Some("Error") => LevelFilter::Error,
                Some("Warn") => LevelFilter::Warn,
                Some("Info") => LevelFilter::Info,
                Some("Debug") => LevelFilter::Debug,
                Some("Trace") => LevelFilter::Trace,
                Some(unknown) => {
                    eprintln!("Unknown debug level [{}]", unknown);
                    exit(-1);
                },
                None => {
                    LevelFilter::Off
                }
            }
        },
        false => LevelFilter::Off
    };

    // Create logging with debug level that prints to stderr
    let result = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(message_level)
        .chain(std::io::stderr())
        .apply();
    
    // Ensure that logger was dispatched
    match result {
        Ok(_) => trace!("Logging as been initialized!"),
        Err(error) => {
            eprintln!("Error initializing fern logging: {}", error);
            exit(-1);
        }
    }
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    set_debug_level(&options);

    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::GreaterThanOrEqual))
    ];

    let channel_list = get_channel_name_list();
    let mut query_list = QueryList::new();
    for channel in channel_list {
        eprintln!("channel: {}", channel);

        // Get the config for this channel
        let channel_config = match ChannelConfig::new(channel.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error getting ChannelConfig for {}: {:?}", channel, e);
                continue;
            }
        };

        // We can only monitor channels that are enabled, and are classic event log channels
        if !channel_config.is_enabled() || !channel_config.is_classic_event_log() {
            continue;
        }

        let mut channel_query = Query::new();

        let query_item = QueryItem::selector(channel)
            .system_conditions(Condition::or(conditions.clone()))
            .build();

        channel_query.item(query_item);

        query_list.with_query(
            channel_query
        );
    }

    let query_list_build = query_list.build();
    debug!("XPath query: {}", query_list_build);
    match WinEventsSubscriber::get(query_list_build) {
        Ok(mut events) => {
            println!("Ctrl+C to quit!");

            while let Some(_event) = events.next() {
                // catch up to present
            }

            println!("Waiting for new events...");
            loop {
                while let Some(event) = events.next() {
                    let xml_string = event.to_string();
                    let value = xml_string_to_json(xml_string);
                    println!("{}", &value.to_string());
                }
                sleep(Duration::from_millis(200));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
