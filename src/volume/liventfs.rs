use crate::errors::WinThingError;
use crate::file::helper::query_file_record;
use crate::usn::structs::UsnJournalData;
use crate::usn::winioctrl::query_usn_journal;
use byteorder::{LittleEndian, ReadBytesExt};
use mft::MftEntry;
use std::fs::File;
use std::io::Read;
use std::os::windows::io::AsRawHandle;
use std::ptr;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::FSCTL_GET_NTFS_VOLUME_DATA;
use winapi::um::winioctl::NTFS_VOLUME_DATA_BUFFER;
use winapi::um::winnt::HANDLE;

/// Query FSCTL_GET_NTFS_VOLUME_DATA to get the NTFS volume data.
/// https://docs.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_get_ntfs_volume_data
///
pub fn get_ntfs_volume_data(
    volume_handle: HANDLE,
) -> Result<NTFS_VOLUME_DATA_BUFFER, WinThingError> {
    let mut bytes_read = 0;
    let mut output_buffer = vec![0u8; 128];

    let result = unsafe {
        DeviceIoControl(
            volume_handle,
            FSCTL_GET_NTFS_VOLUME_DATA,
            ptr::null_mut(),
            0,
            output_buffer.as_mut_ptr() as *mut _,
            output_buffer.len() as u32,
            &mut bytes_read,
            ptr::null_mut(),
        )
    };

    if result == 0 {
        return Err(WinThingError::from_windows_last_error());
    }

    let volume_data_buffer: NTFS_VOLUME_DATA_BUFFER =
        unsafe { std::ptr::read(output_buffer.as_ptr() as *const _) };

    Ok(volume_data_buffer)
}

#[derive(Debug)]
pub struct MftOutputBuffer {
    file_reference_number: u64,
    file_record_length: u32,
    file_record_buffer: Vec<u8>,
}

impl MftOutputBuffer {
    pub fn from_buffer<T: Read>(mut raw_buffer: T) -> Result<Self, WinThingError> {
        let file_reference_number = raw_buffer.read_u64::<LittleEndian>()?;
        let file_record_length = raw_buffer.read_u32::<LittleEndian>()?;
        let mut file_record_buffer = vec![0; file_record_length as usize];

        raw_buffer.read_exact(&mut file_record_buffer)?;

        Ok(MftOutputBuffer {
            file_reference_number,
            file_record_length,
            file_record_buffer,
        })
    }

    pub fn buffer_as_hex(&self) -> String {
        hex::encode(&self.file_record_buffer)
    }

    pub fn as_entry(&self) -> Result<MftEntry, WinThingError> {
        Ok(MftEntry::from_buffer_skip_fixup(
            self.file_record_buffer.clone(),
            self.file_reference_number,
        )?)
    }
}

/// Struct for interacting with a live NTFS volume via Windows API
///
pub struct WindowsLiveNtfs {
    _volume_path: String,
    volume_handle: File,
    ntfs_volume_data: NTFS_VOLUME_DATA_BUFFER,
}
impl WindowsLiveNtfs {
    pub fn from_volume_path(volume_path: &str) -> Result<Self, WinThingError> {
        let file_handle = File::open(&volume_path)?;

        let ntfs_volume_data = get_ntfs_volume_data(file_handle.as_raw_handle())?;

        Ok(WindowsLiveNtfs {
            _volume_path: volume_path.to_string(),
            volume_handle: file_handle,
            ntfs_volume_data: ntfs_volume_data,
        })
    }

    pub fn query_usn_journal(&mut self) -> Result<UsnJournalData, WinThingError> {
        query_usn_journal(self.volume_handle.as_raw_handle())
    }

    fn get_entry_buffer(&mut self, entry: i64) -> Result<MftOutputBuffer, WinThingError> {
        let raw_buffer = query_file_record(
            self.volume_handle.as_raw_handle(),
            entry,
            self.ntfs_volume_data.BytesPerFileRecordSegment,
        )?;

        MftOutputBuffer::from_buffer(&raw_buffer[..])
    }

    pub fn get_entry(&mut self, entry: i64) -> Result<MftEntry, WinThingError> {
        let mft_buffer = self.get_entry_buffer(entry)?;
        mft_buffer.as_entry()
    }
}
