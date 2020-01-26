use serde_json::Value;
use std::sync::mpsc::Receiver;
use winapi::um::winevt::{
    EvtSubscribeToFutureEvents,
    EvtSubscribeStartAtOldestRecord
};
use crate::winevt::EvtHandle;
use crate::mft::EntryListener;
use crate::errors::WinThingError;
use crate::winevt::callback::OutputFormat;
use crate::usn::listener::UsnListenerConfig;
use crate::winevt::callback::CallbackContext;
use crate::winevt::subscription::ChannelSubscription;


pub struct WindowsHandler {}

impl WindowsHandler {
    pub fn new() -> Self {
        Self{}
    }
    
    /// Listen to a file's MFT changes. Get the reciever.
    pub fn listen_mft(
        &self, 
        file_path: &str
    ) -> Result<Receiver<Value>, WinThingError> {
        let listener = EntryListener::new(file_path)?;
        listener.listen_to_file()
    }

    /// Listen to a volume's USN changes. Get the reciever.
    pub fn listen_usn(
        &self, 
        volume: &str, 
        config: Option<UsnListenerConfig>
    ) -> Result<Receiver<Value>, WinThingError> {
        let config = match config {
            Some(c) => c,
            None => UsnListenerConfig::default()
        };
        
        let listener = config.get_listener(volume);

        listener.listen_to_volume()
    }

    /// Listen to Windows events
    pub fn listen_events(
        &self,
        session: Option<EvtHandle>,
        historical_flag: bool,
        format_enum: OutputFormat,
        channel_list: Vec<String>
    ) -> Result<(Receiver<Value>, Vec<ChannelSubscription>), WinThingError> {
        // Create context
        let (rx, mut context) = CallbackContext::with_reciever();
        context = context.with_format(format_enum);

        let mut subscriptions: Vec<ChannelSubscription> = Vec::new();

        // Historical flag
        let flags = match historical_flag {
            true => Some(EvtSubscribeStartAtOldestRecord),
            false => Some(EvtSubscribeToFutureEvents)
        };

        for channel in channel_list {
            eprintln!("creating {} ChannelSubscription", channel);
            // Create subscription
            let subscription = match ChannelSubscription::new(
                &session,
                channel.to_string(),
                None,
                flags,
                &mut context
            ){
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error creating subscription for {}: {:?}", channel, e);
                    continue;
                }
            };
    
            subscriptions.push(
                subscription
            );
        }
    
        Ok((
            rx, subscriptions
        ))
    }
}