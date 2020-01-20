use std::fs::File;
use mft::MftEntry;
use serde_json::Value;
use std::sync::mpsc::Receiver;
use std::os::windows::io::AsRawHandle;
use winapi::um::winioctl::NTFS_VOLUME_DATA_BUFFER;
use crate::errors::WinThingError;
use crate::mft::MftOutputBuffer;
use crate::file::helper::query_file_record;
use crate::devio::volume::get_ntfs_volume_data;


pub struct WindowsHandler {}

impl WindowsHandler {
    /// Listen to a file's MFT changes. Get the reciever.
    pub fn listen_mft(&self, _file_path: &str) -> Result<Receiver<Value>, WinThingError> {
        Err(
            WinThingError::unhandled(
                "listen_usn unimplemented.".to_string()
            )
        )
    }

    /// Listen to a volume's USN changes. Get the reciever.
    pub fn listen_usn(&self, _volume_path: &str) -> Result<Receiver<Value>, WinThingError> {
        Err(
            WinThingError::unhandled(
                "listen_usn unimplemented.".to_string()
            )
        )
    }
}


/// Struct for interacting with a live NTFS volume via Windows API
///
pub struct WindowsLiveNtfs {
    volume_path: String,
    volume_handle: File,
    ntfs_volume_data: NTFS_VOLUME_DATA_BUFFER
}
impl WindowsLiveNtfs {
    /// Create a WindowsLiveNtfs from the volume path
    /// 
    pub fn from_volume_path(volume_path: &str) -> Result<Self, WinThingError> {
        let file_handle = File::open(
            &volume_path
        )?;
        
        let ntfs_volume_data = get_ntfs_volume_data(
            file_handle.as_raw_handle()
        )?;

        Ok(
            WindowsLiveNtfs {
                volume_path: volume_path.to_string(),
                volume_handle: file_handle,
                ntfs_volume_data: ntfs_volume_data
            }
        )
    }

    /// Get the handle reference to this volume
    /// 
    pub fn get_handle(&self) -> &File {
        &self.volume_handle
    }

    /// Get an MftEntry for a given entry number
    /// 
    pub fn get_mft_entry(&mut self, entry: i64) -> Result<MftEntry, WinThingError> {
        let raw_buffer = query_file_record(
            self.volume_handle.as_raw_handle(),
            entry,
            self.ntfs_volume_data.BytesPerFileRecordSegment
        )?;

        let mft_buffer = MftOutputBuffer::from_buffer(
            &raw_buffer[..]
        )?;

        mft_buffer.as_entry()
    }
}

impl Clone for WindowsLiveNtfs {
    fn clone(&self) -> Self {
        Self::from_volume_path(
            &self.volume_path
        ).expect(
            &format!(
                "Unable to create WindowsLiveNtfs from {}", 
                self.volume_path
            )
        )
    }
}