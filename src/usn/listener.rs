use crate::errors::WinThingError;
use crate::usn::structs::ReadUsnJournalData;
use crate::usn::winioctrl::read_usn_journal;
use crate::volume::liventfs::WindowsLiveNtfs;
use byteorder::{ByteOrder, LittleEndian};
use rusty_usn::record::{EntryMeta, UsnEntry};
use rusty_usn::usn::IterRecordsByIndex;
use std::fs::File;
use std::os::windows::io::AsRawHandle;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub struct UsnVolumeListener {
    source: String,
    historical_flag: bool,
    sender: Sender<UsnEntry>,
}

impl UsnVolumeListener {
    pub fn new(source: String, historical_flag: bool, sender: Sender<UsnEntry>) -> Self {
        Self {
            source,
            historical_flag,
            sender,
        }
    }

    pub fn listen_to_volume(self, reason_mask: Option<u32>) -> Result<(), WinThingError> {
        let mut live_volume = WindowsLiveNtfs::from_volume_path(&self.source)?;

        let file_handle = File::open(self.source.clone())?;

        let reason_mask = match reason_mask {
            Some(r) => r,
            None => 0xffffffff,
        };

        let usn_journal_data = live_volume.query_usn_journal()?;

        let mut next_start_usn: u64 = usn_journal_data.get_next_usn();

        if self.historical_flag {
            next_start_usn = 0;
        }

        loop {
            let read_data = ReadUsnJournalData::from_usn_journal_data(usn_journal_data.clone())
                .with_start_usn(next_start_usn)
                .with_reason_mask(reason_mask);

            let buffer = match read_usn_journal(file_handle.as_raw_handle(), read_data) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{:#?}", e);
                    break;
                }
            };

            // The first 8 bytes are the usn of the next record NOT in the buffer,
            // use this value as the next_start_usn
            next_start_usn = LittleEndian::read_u64(&buffer[0..8]);

            let entry_meta = EntryMeta::new(&self.source, 0);

            let record_iterator = IterRecordsByIndex::new(entry_meta, buffer[8..].to_vec());

            let mut record_count: u64 = 0;
            for usn_entry in record_iterator {
                match self.sender.send(usn_entry) {
                    Ok(_) => {
                        record_count += 1;
                    }
                    Err(error) => {
                        eprintln!("error sending usn entry: {:?}", error);
                    }
                }
            }

            // need to sleep to minimize resources
            if record_count == 0 {
                thread::sleep(Duration::from_millis(100));
            }
        }

        Ok(())
    }
}
