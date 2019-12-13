#[macro_use]
extern crate log;
extern crate clap;
extern crate chrono;
extern crate serde_json;
extern crate win_event_log;
use clap::{App, Arg};
use std::thread::sleep;
use std::time::Duration;
use win_event_log::prelude::*;
use rswinthings::utils::debug::set_debug_level;
use rswinthings::utils::xmltojson::xml_string_to_json;
use rswinthings::winevt::channels::get_channel_name_list;
use rswinthings::winevt::channels::ChannelConfig;

static VERSION: &'static str = "0.0.1";
static DESCRIPTION: &'static str = r"
Event listener written in Rust. Output is JSONL.

This tool queries the available list of channels then creates a XPath
query and uses the Windows API to monitor for events on the applicable 
channels. Currently, all classic eventlog channels are selected for 
monitoring. Use the print_channels tool to list available channels and
their configurations.
";


fn make_app<'a, 'b>() -> App<'a, 'b> {
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

    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::GreaterThanOrEqual))
    ];

    // Get a list off all the channels
    let channel_list = get_channel_name_list();
    // Create our query list for XPath
    let mut query_list = QueryList::new();

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

        if channel.contains("Analytic") || channel.contains("Debug") {
            // We cant monitor Analytic or Debug channels
            // See https://docs.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtsubscribe
            // It wont error, but it wont work either if we include any of these.
            continue;
        }

        // Cutting out config types of 2 or more seems to resolve
        // observed Subscription issues
        match channel_config.get_config_type() {
            Some(i) => {
                if i > 1 {
                    continue;
                }
            },
            None => continue
        };

        // Cutting out config isolations that are not 0 seems to resolve
        // observed Subscription issues
        match channel_config.get_config_isolation() {
            Some(i) => {
                if i != 0 {
                    continue;
                }
            },
            None => continue
        }

        // We can only monitor channels that are enabled and are classic event log channels
        if !channel_config.is_enabled() {
            continue;
        }

        eprintln!("listening to channel: {}", channel);

        // Create this channels XPath query
        let mut channel_query = Query::new();
        let query_item = QueryItem::selector(channel)
            .system_conditions(Condition::or(conditions.clone()))
            .build();

        channel_query.item(query_item);

        query_list.with_query(
            channel_query
        );
    }

    // Build the complete xpath query.
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
