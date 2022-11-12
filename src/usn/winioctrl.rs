use crate::errors::WinThingError;
use crate::usn::structs as usnstruct;
use std::mem;
use std::ptr;
use winapi::ctypes::c_void;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::{FSCTL_QUERY_USN_JOURNAL, FSCTL_READ_USN_JOURNAL};
use winapi::um::winnt::HANDLE;

/// BOOL WINAPI DeviceIoControl(
/// 	(HANDLE) Device,                       // handle to volume
/// 	(DWORD)        FSCTL_QUERY_USN_JOURNAL,// dwIoControlCode
/// 	(LPVOID)       NULL,                   // lpInBuffer
/// 	(DWORD)        0,                      // nInBufferSize
/// 	(LPVOID)       lpOutBuffer,            // output buffer
/// 	(DWORD)        nOutBufferSize,         // size of output buffer
/// 	(LPDWORD)      lpBytesReturned,        // number of bytes returned
/// 	(LPOVERLAPPED) lpOverlapped            // OVERLAPPED structure
/// );
pub fn query_usn_journal(
    volume_handle: HANDLE,
) -> Result<usnstruct::UsnJournalData, WinThingError> {
    let mut output_buffer = [0u8; 80];
    let mut bytes_read = 0;

    let result = unsafe {
        DeviceIoControl(
            volume_handle,
            FSCTL_QUERY_USN_JOURNAL,
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

    usnstruct::UsnJournalData::new(&output_buffer[..bytes_read as usize])
}

/// BOOL WINAPI DeviceIoControl(
/// 	(HANDLE)       hDevice,         // handle to volume
/// 	(DWORD) FSCTL_READ_USN_JOURNAL, // dwIoControlCode
/// 	(LPVOID)       lpInBuffer,      // input buffer
/// 	(DWORD)        nInBufferSize,   // size of input buffer
/// 	(LPVOID)       lpOutBuffer,     // output buffer
/// 	(DWORD)        nOutBufferSize,  // size of output buffer
/// 	(LPDWORD)      lpBytesReturned, // number of bytes returned
/// 	(LPOVERLAPPED) lpOverlapped     // OVERLAPPED structure
/// );
pub fn read_usn_journal(
    volume_handle: HANDLE,
    read_jounral_data: usnstruct::ReadUsnJournalData,
) -> Result<Vec<u8>, WinThingError> {
    let mut bytes_read: u32 = 0;
    let mut record_buffer = vec![0u8; 4096];

    let result = match read_jounral_data {
        usnstruct::ReadUsnJournalData::V0(mut read_data_v0) => unsafe {
            DeviceIoControl(
                volume_handle,
                FSCTL_READ_USN_JOURNAL,
                &mut read_data_v0 as *mut _ as *mut c_void,
                mem::size_of::<usnstruct::ReadUsnJournalDataV0>() as u32,
                record_buffer.as_mut_ptr() as *mut _,
                record_buffer.len() as u32,
                &mut bytes_read,
                ptr::null_mut(),
            )
        },
        usnstruct::ReadUsnJournalData::V1(mut read_data_v1) => unsafe {
            DeviceIoControl(
                volume_handle,
                FSCTL_READ_USN_JOURNAL,
                &mut read_data_v1 as *mut _ as *mut c_void,
                mem::size_of::<usnstruct::ReadUsnJournalDataV1>() as u32,
                record_buffer.as_mut_ptr() as *mut _,
                record_buffer.len() as u32,
                &mut bytes_read,
                ptr::null_mut(),
            )
        },
    };

    if result == 0 {
        return Err(WinThingError::from_windows_last_error());
    }

    record_buffer.truncate(bytes_read as usize);

    Ok(record_buffer)
}
