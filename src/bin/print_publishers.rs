#[macro_use]
extern crate log;
use clap::{App, Arg};
use rswinthings::utils::cli::{add_session_options_to_app, get_session_from_matches};
use rswinthings::utils::debug::set_debug_level;
use rswinthings::winetl::publisher::PublisherEnumerator;
use rswinthings::winetl::publisher::PublisherMeta;
use rswinthings::winevt::EvtHandle;
use std::process::exit;
use winapi::um::winevt::*;

static VERSION: &'static str = "0.1.0";

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let provider = Arg::with_name("provider")
        .short("-p")
        .long("provider")
        .value_name("PROVIDER")
        .multiple(true)
        .takes_value(true)
        .help("Specific Provider.");

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

    let app = App::new("print_publishers")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>")
        .about("Print Publisher Propperties.")
        .arg(provider)
        .arg(format)
        .arg(debug);

    // Add session arguments to app
    add_session_options_to_app(app)
}

fn get_message_desc(message: Option<String>) -> String {
    match message {
        Some(s) => format!("[{}]", s),
        None => "".to_string(),
    }
}

fn get_text_block(meta: &PublisherMeta) -> String {
    let mut message: String;
    let mut temp: String;
    message = "----------------------------------------------\n".to_string();
    message.push_str(&format!("Publisher: {}\n", meta.name));
    temp = meta
        .get_property_string(EvtPublisherMetadataPublisherGuid)
        .unwrap_or("".to_string());
    message.push_str(&format!("GUID: {}\n", temp));
    message.push_str(&format!("----------------------------------------------\n"));
    temp = meta
        .get_property_string(EvtPublisherMetadataResourceFilePath)
        .unwrap_or("".to_string());
    message.push_str(&format!("Resource File Path: {}\n", temp));
    temp = meta
        .get_property_string(EvtPublisherMetadataParameterFilePath)
        .unwrap_or("".to_string());
    message.push_str(&format!("Parameter File Path: {}\n", temp));
    temp = meta
        .get_property_string(EvtPublisherMetadataMessageFilePath)
        .unwrap_or("".to_string());
    message.push_str(&format!("Message File Path: {}\n", temp));
    temp = meta
        .get_property_string(EvtPublisherMetadataHelpLink)
        .unwrap_or("".to_string());
    message.push_str(&format!("Help Link: {}\n", temp));
    temp = match meta.get_publisher_message() {
        Ok(m) => m.unwrap_or("".to_string()),
        Err(e) => {
            error!(
                "[{}] Error getting publisher message: {}",
                meta.name, e.message
            );
            "".to_string()
        }
    };
    message.push_str(&format!("Publisher Message: {}\n", temp));

    message.push_str(&format!("--- Channels ---\n"));
    match meta.get_metadata_channels() {
        Ok(metadata) => {
            for meta_item in metadata.0 {
                message.push_str(&format!(
                    "{:016X}: {} {}\n",
                    meta_item
                        .index
                        .to_string()
                        .parse::<u64>()
                        .expect("Error parsing id"),
                    meta_item.path.to_string(),
                    get_message_desc(meta_item.message)
                ));
            }
        }
        Err(e) => {
            error!("[{}] Error getting channels: {}", meta.name, e.message);
        }
    }

    message.push_str(&format!("--- Keywords ---\n"));
    match meta.get_metadata_keywords() {
        Ok(metadata) => {
            for meta_item in metadata.0 {
                message.push_str(&format!(
                    "{:016X}: {} {}\n",
                    meta_item
                        .value
                        .to_string()
                        .parse::<u64>()
                        .expect("Error parsing id"),
                    meta_item.name.to_string(),
                    get_message_desc(meta_item.message)
                ));
            }
        }
        Err(e) => {
            error!("[{}] Error getting keywords: {}", meta.name, e.message);
        }
    }

    message.push_str(&format!("--- Operations ---\n"));
    match meta.get_metadata_opcodes() {
        Ok(metadata) => {
            for meta_item in metadata.0 {
                message.push_str(&format!(
                    "{:016X}: {} {}\n",
                    meta_item
                        .value
                        .to_string()
                        .parse::<u64>()
                        .expect("Error parsing id"),
                    meta_item.name.to_string(),
                    get_message_desc(meta_item.message)
                ));
            }
        }
        Err(e) => {
            error!("[{}] Error getting opcodes: {}", meta.name, e.message);
        }
    }

    message.push_str(&format!("--- Levels ---\n"));
    match meta.get_metadata_levels() {
        Ok(metadata) => {
            for meta_item in metadata.0 {
                message.push_str(&format!(
                    "{:016X}: {} {}\n",
                    meta_item
                        .value
                        .to_string()
                        .parse::<u64>()
                        .expect("Error parsing id"),
                    meta_item.name.to_string(),
                    get_message_desc(meta_item.message)
                ));
            }
        }
        Err(e) => {
            error!("[{}] Error getting levels: {}", meta.name, e.message);
        }
    }

    message.push_str(&format!("--- Tasks ---\n"));
    match meta.get_metadata_tasks() {
        Ok(metadata) => {
            for meta_item in metadata.0 {
                message.push_str(&format!(
                    "{:016X}: {} {}\n",
                    meta_item
                        .value
                        .to_string()
                        .parse::<u64>()
                        .expect("Error parsing id"),
                    meta_item.name.to_string(),
                    get_message_desc(meta_item.message)
                ));
            }
        }
        Err(e) => {
            error!("[{}] Error getting tasks: {}", meta.name, e.message);
        }
    }

    message
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
        get_session_from_matches(&options).expect("Error getting session from options").map(|sess| sess.into_handle());

    let out_format = match options.value_of("format") {
        Some(f) => f,
        None => "text",
    };

    match options.values_of("provider") {
        Some(p_list) => {
            for value in p_list {
                let publisher_meta = PublisherMeta::new(&session, value.to_string())
                    .expect("Error creating PublisherMeta");

                match out_format {
                    "text" => {
                        let out = get_text_block(&publisher_meta);
                        println!("{}", out);
                    }
                    "jsonl" => {
                        let meta_value = match publisher_meta.to_json_value() {
                            Ok(v) => v,
                            Err(e) => {
                                error!("Error serializing value: {:?}", e);
                                continue;
                            }
                        };
                        println!("{}", meta_value.to_string());
                    }
                    other => {
                        eprintln!("Unhandled output format: {}", other);
                        exit(-1);
                    }
                }
            }
        }
        None => {
            let enumerator =
                PublisherEnumerator::new(session).expect("Error creating PublisherEnumerator");

            for publisher_meta in enumerator {
                match out_format {
                    "text" => {
                        let out = get_text_block(&publisher_meta);
                        println!("{}", out);
                    }
                    "jsonl" => {
                        let meta_value = match publisher_meta.to_json_value() {
                            Ok(v) => v,
                            Err(e) => {
                                error!("Error serializing value: {:?}", e);
                                continue;
                            }
                        };
                        println!("{}", meta_value.to_string());
                    }
                    other => {
                        eprintln!("Unhandled output format: {}", other);
                        exit(-1);
                    }
                }
            }
        }
    }
}
