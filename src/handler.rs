use crate::errors::WinThingError;
use crate::mft::EntryListener;
use crate::usn::listener::UsnListenerConfig;
use crate::winevt::callback::CallbackContext;
use crate::winevt::callback::OutputFormat;
use crate::winevt::{EvtHandle, EvtSubscriptionHandle};
use crossbeam::channel::{self, Receiver, Select, Sender};
use serde_json::Value;
use std::collections::HashMap;
use std::thread;

use winapi::um::winevt::{EvtSubscribeStartAtOldestRecord, EvtSubscribeToFutureEvents};

pub fn launch_elistener(
    tx: Sender<Value>,
    historical_flag: bool,
    _format_enum: OutputFormat,
    channel_list: Vec<String>,
) {
    // Historical flag
    let _flags = match historical_flag {
        true => Some(EvtSubscribeStartAtOldestRecord),
        false => Some(EvtSubscribeToFutureEvents),
    };

    let mut senders = HashMap::new();
    let mut receivers = Vec::new();
    let mut handles = vec![];

    for chan in channel_list.iter() {
        let (s, r) = channel::unbounded();
        senders.insert(chan, CallbackContext::new(s));
        receivers.push(r);
    }

    for (chan, ctx) in senders.iter_mut() {
        let handle =
            EvtSubscriptionHandle::register(&None, chan, None, None, ctx).expect("Create channel");

        handles.push(handle);
    }

    let mut sel = Select::new();

    for r in receivers.iter() {
        sel.recv(r);
    }

    loop {
        // Wait until a receive operation becomes ready and try executing it.
        let index = sel.ready();
        let value = match receivers[index].try_recv() {
            Err(e) => {
                eprintln!("Error recieving: {:?}", e);
                continue;
            }
            Ok(v) => v,
        };

        match tx.send(value) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("error sending value: {:?}", error);
            }
        }
    }
}

pub struct WindowsHandler {}

impl WindowsHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// Listen to a file's MFT changes. Get the reciever.
    pub fn listen_mft(&self, file_path: &str) -> Result<Receiver<Value>, WinThingError> {
        let listener = EntryListener::new(file_path)?;
        listener.listen_to_file()
    }

    /// Listen to a volume's USN changes. Get the reciever.
    pub fn listen_usn(
        &self,
        volume: &str,
        config: Option<UsnListenerConfig>,
    ) -> Result<Receiver<Value>, WinThingError> {
        let config = match config {
            Some(c) => c,
            None => UsnListenerConfig::default(),
        };

        let listener = config.get_listener(volume);

        listener.listen_to_volume()
    }

    /// Listen to Windows events
    pub fn listen_events(
        &self,
        session: Option<EvtHandle>,
        historical_flag: bool,
        _format_enum: OutputFormat,
        channel_list: Vec<String>,
    ) -> Result<Receiver<Value>, WinThingError> {
        let (tx, rx): (Sender<Value>, Receiver<Value>) = channel::unbounded();

        let _thread = thread::spawn(move || {
            launch_elistener(tx.clone(), historical_flag, _format_enum, channel_list)
        });

        Ok(rx)
    }
}
