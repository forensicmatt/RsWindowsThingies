#[macro_use] extern crate serde_json;
use rswinthings::winevt::channels::ChannelConfig;
use rswinthings::winevt::channels::get_channel_name_list;


fn main() {
    let channels = get_channel_name_list();
    for channel in channels {
        let channel_config = match ChannelConfig::new(channel.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error creating ChannelConfig for: {:?}", e);
                continue;
            }
        };

        let mut channel_config = match channel_config.to_json_value() {
            Ok(p) => p,
            Err(e) => continue
        };

        channel_config["ChannelName"] = json!(channel.to_owned());

        println!("{}", channel_config.to_string());
    }
}