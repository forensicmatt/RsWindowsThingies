use std::thread;
use std::fs::File;
use serde_json::Value;
use std::time::Duration;
use std::os::windows::io::AsRawHandle;
use std::sync::mpsc::{
    channel,
    Sender, 
    Receiver
};
use byteorder::{ByteOrder, LittleEndian};
use rusty_usn::record::{EntryMeta, UsnEntry};
use rusty_usn::usn::IterRecordsByIndex;
use crate::errors::WinThingError;
use crate::volume::liventfs::WindowsLiveNtfs;
use crate::usn::structs::ReadUsnJournalData;
use crate::devio::volume::query_usn_journal;
use crate::devio::volume::read_usn_journal;


fn listen_usn(
    listener: UsnVolumeListener,
    tx: Sender<Value>
) -> Result<(), WinThingError> {
    let live_volume = WindowsLiveNtfs::from_volume_path(
        &listener.source
    )?;

    let file_handle = File::open(
        listener.source.clone()
    )?;

    let usn_journal_data = query_usn_journal(
        live_volume.get_handle().as_raw_handle()
    )?;

    let mut next_start_usn: u64 = usn_journal_data.get_next_usn();

    if listener.config.historical_flag {
        next_start_usn = 0;
    }

    loop {
        let read_data = ReadUsnJournalData::from_usn_journal_data(
            usn_journal_data.clone()
        ).with_start_usn(next_start_usn)
            .with_reason_mask(listener.config.mask);

        let buffer = match read_usn_journal(
            file_handle.as_raw_handle(),
            read_data
        ){
            Ok(b) => b,
            Err(e) => {
                eprintln!("{:#?}", e);
                break;
            }
        };

        // The first 8 bytes are the usn of the next record NOT in the buffer,
        // use this value as the next_start_usn
        next_start_usn = LittleEndian::read_u64(
            &buffer[0..8]
        );

        let entry_meta = EntryMeta::new(
            &listener.source,
            0
        );

        let record_iterator = IterRecordsByIndex::new(
            entry_meta,
            buffer[8..].to_vec()
        );
        
        let mut record_count: u64 = 0;
        for usn_entry in record_iterator {
            let entry_value = usn_entry.to_json_value()?;

            match tx.send(entry_value) {
                Ok(_) => {
                    record_count += 1;
                },
                Err(error) => {
                    eprintln!("error sending usn entry: {:?}", error);
                }
            }
        }

        // need to sleep to minimize resources
        if record_count == 0 {
            thread::sleep(
                Duration::from_millis(
                    100
                )
            );
        }
    }

    Ok(())
}


#[derive(Clone)]
pub struct UsnListenerConfig {
    pub historical_flag: bool,
    pub enumerate_paths: bool,
    pub mask: u32
}
impl UsnListenerConfig {
    pub fn new() -> Self {
        UsnListenerConfig::default()
    }

    pub fn mask(mut self, mask: u32) -> Self {
        self.mask = mask;
        self
    }

    pub fn historic(mut self, historical_flag: bool) -> Self {
        self.historical_flag = historical_flag;
        self
    }

    pub fn enumerate_paths(mut self, enumerate_paths: bool) -> Self {
        self.enumerate_paths = enumerate_paths;
        self
    }

    pub fn get_listener(self, volume: &str) -> UsnVolumeListener {
        UsnVolumeListener::new(
            volume.to_string(),
            self
        )
    }
}
impl Default for UsnListenerConfig {
    fn default() -> Self { 
        Self {
            historical_flag: false,
            enumerate_paths: true,
            mask: 0xffffffff
        }
    }
}


pub struct UsnVolumeListener {
    source: String,
    config: UsnListenerConfig
}
impl UsnVolumeListener {
    pub fn new(
        source: String,
        config: UsnListenerConfig
    ) -> Self {
        Self {
            source,
            config
        }
    }

    pub fn listen_to_volume(
        self
    ) -> Result<Receiver<Value>, WinThingError> {
        let (tx, rx): (Sender<Value>, Receiver<Value>) = channel();

        let _thread = thread::spawn(move || {
            match listen_usn(
                self,
                tx
            ) {
                Ok(_) => println!("thread terminated"),
                Err(e) => eprintln!("Error listening: {:?}", e)
            }
        });

        Ok(rx)
    }
}