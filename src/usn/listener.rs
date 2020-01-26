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
use rusty_usn::flags;
use rusty_usn::record::EntryMeta;
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

    let mut mapping = None;

    // Path enumeration is optional
    if listener.config.enumerate_paths {
        debug!("creating folder mapping");
        mapping = Some(
            live_volume.get_folder_mapping()
        );
        debug!("finished folder mapping");
    }

    let file_handle = File::open(
        listener.source.clone()
    )?;

    let usn_journal_data = query_usn_journal(
        live_volume.get_handle().as_raw_handle()
    )?;

    let mut next_start_usn: u64 = usn_journal_data.get_next_usn();
    let catch_up_usn = next_start_usn;

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
            let mut entry_value = usn_entry.to_json_value()?;

            match mapping {
                None => {},
                Some(ref mut mapping) => {
                    let entry_usn = usn_entry.record.get_usn();
                    let file_name = usn_entry.record.get_file_name();
                    let file_ref = usn_entry.record.get_file_reference();
                    let reason_code = usn_entry.record.get_reason_code();
                    let parent_ref = usn_entry.record.get_parent_reference();
                    let file_attributes = usn_entry.record.get_file_attributes();

                    if file_attributes.contains(flags::FileAttributes::FILE_ATTRIBUTE_DIRECTORY){
                        if reason_code.contains(flags::Reason::USN_REASON_RENAME_OLD_NAME) {
                            // We can remove old names from the mapping because we no longer need these.
                            // On new names, we add the name to the mapping.
                            mapping.remove_mapping(
                                file_ref
                            );
                        }
                        else if reason_code.contains(flags::Reason::USN_REASON_FILE_DELETE) {
                            // If we are starting from historical entries, we need to add deleted
                            // entries to the map until we catch up to the current system, then we can 
                            // start removing deleted entries. This is because our mapping cannot
                            // get unallocated entries from the MFT via the Windows API.
                            if listener.config.historical_flag && entry_usn < catch_up_usn {
                                mapping.add_mapping(
                                    file_ref, 
                                    file_name.clone(), 
                                    parent_ref
                                )
                            } else {
                                mapping.remove_mapping(
                                    file_ref
                                );
                            }
                        } else if reason_code.contains(flags::Reason::USN_REASON_RENAME_NEW_NAME) ||
                            reason_code.contains(flags::Reason::USN_REASON_FILE_CREATE) {
                            // If its a new name or creation, we need to updated the mapping
                            mapping.add_mapping(
                                file_ref, 
                                file_name.clone(), 
                                parent_ref
                            )
                        }
                    }

                    // Enumerate the path of this record from the FolderMapping
                    let full_path = match mapping.enumerate_path(
                        parent_ref.entry,
                        parent_ref.sequence
                    ){
                        Some(path) => path,
                        None => "[<unknown>]".to_string()
                    };

                    let full_file_name = format!("{}/{}", &full_path, &file_name);
                    entry_value["full_path"] = Value::String(full_file_name);
                }
            }

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