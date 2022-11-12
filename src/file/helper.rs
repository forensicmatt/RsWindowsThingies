use crate::errors::WinThingError;
use crate::file::fileapi::{create_file, get_file_information_by_handle, get_volume_path_name};
use regex::Regex;
use std::mem;
use std::ptr;
use winapi::ctypes::c_void;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::FSCTL_GET_NTFS_FILE_RECORD;
use winapi::um::winioctl::NTFS_FILE_RECORD_INPUT_BUFFER;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::LARGE_INTEGER;

/// Get the mft entry number from a given path
///
pub fn get_entry_from_path(path: &str) -> Result<u64, WinThingError> {
    let file = create_file(path)?;
    let file_info = get_file_information_by_handle(file.0)?;

    Ok(file_info.nFileIndexLow as u64)
}

pub fn get_volume_path_from_path(path: &str) -> Result<String, WinThingError> {
    let root = get_volume_path_name(&path)?;

    let re = Regex::new(r"([a-zA-Z]):").unwrap();
    let volume_str = match re.captures(&root) {
        Some(caps) => match caps.get(1) {
            Some(v) => v.as_str(),
            None => {
                return Err(WinThingError::unhandled(format!(
                    "Unable to get volume for {}",
                    path
                )))
            }
        },
        None => {
            return Err(WinThingError::unhandled(format!(
                "Unable to get volume for {}",
                path
            )))
        }
    };

    let volume = format!("\\\\.\\{}:", volume_str);

    Ok(volume)
}

/// BOOL DeviceIoControl(
/// 	(HANDLE) hDevice,              // handle to device
/// 	FSCTL_GET_NTFS_FILE_RECORD,    // dwIoControlCode
/// 	(LPVOID) lpInBuffer,           // input buffer
/// 	(DWORD) nInBufferSize,         // size of input buffer
/// 	(LPVOID) lpOutBuffer,          // output buffer
/// 	(DWORD) nOutBufferSize,        // size of output buffer
/// 	(LPDWORD) lpBytesReturned,     // number of bytes returned
/// 	(LPOVERLAPPED) lpOverlapped    // OVERLAPPED structure
/// );
pub fn query_file_record(
    volume_handle: HANDLE,
    entry: i64,
    entry_size: u32,
) -> Result<Vec<u8>, WinThingError> {
    let mut bytes_read = 0;
    let buffer_size = (entry_size + 12) as usize;
    let mut output_buffer = vec![0u8; buffer_size];

    let result = unsafe {
        let mut entry_reference = mem::zeroed::<LARGE_INTEGER>();
        *entry_reference.QuadPart_mut() = entry;

        // Input buffer
        let mut input_buffer = NTFS_FILE_RECORD_INPUT_BUFFER {
            FileReferenceNumber: entry_reference,
        };

        DeviceIoControl(
            volume_handle,
            FSCTL_GET_NTFS_FILE_RECORD,
            &mut input_buffer as *mut _ as *mut c_void,
            mem::size_of::<NTFS_FILE_RECORD_INPUT_BUFFER>() as u32,
            output_buffer.as_mut_ptr() as *mut _,
            output_buffer.len() as u32,
            &mut bytes_read,
            ptr::null_mut(),
        )
    };

    if result == 0 {
        return Err(WinThingError::from_windows_last_error());
    } else {
        output_buffer.truncate(bytes_read as usize);
    }

    Ok(output_buffer)
}
