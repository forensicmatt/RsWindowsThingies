use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::DWORD;
use winapi::um::fileapi::GetVolumePathNameW;
use winapi::um::fileapi::GetFileInformationByHandle;
use winapi::um::fileapi::BY_HANDLE_FILE_INFORMATION;
use crate::errors::WinThingError;

/// BOOL GetVolumePathNameW(
///   LPCWSTR lpszFileName,
///   LPWSTR  lpszVolumePathName,
///   DWORD   cchBufferLength
/// );
pub fn get_volume_path_name(
    file_name: &str
) -> Result<String, WinThingError> {
    let mut file_name_u16: Vec<u16> = file_name.to_string().encode_utf16().collect();
    file_name_u16.resize(file_name_u16.len() + 1, 0);

    let mut buffer: Vec<u16> = vec![0; 255 as usize];

    let result = unsafe {
        GetVolumePathNameW(
            file_name_u16.as_ptr(),
            buffer.as_mut_ptr(),
            255 as _
        )
    };

    if result == 0 {
        return Err(
            WinThingError::from_windows_last_error()
        )
    }

    let mut index = buffer.len() - 1;
    // Buffers can be null padded. We want to trim the null chars.
    match buffer.iter().position(|&x| x == 0) {
        Some(i) => {
            index = i;
        },
        None => {}
    }

    let volume = OsString::from_wide(
        &buffer[..index]
    ).to_string_lossy().to_string();

    Ok(
        volume
    )
}


/// BOOL GetFileInformationByHandle(
///   HANDLE                       hFile,
///   LPBY_HANDLE_FILE_INFORMATION lpFileInformation
/// );
pub fn get_file_information_by_handle(
    handle: HANDLE
) -> Result<BY_HANDLE_FILE_INFORMATION, WinThingError> {
    let mut handle_file_info = BY_HANDLE_FILE_INFORMATION::default();

    let result = unsafe {
        GetFileInformationByHandle(
            handle,
            &mut handle_file_info
        )
    };

    if result == 0 {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(handle_file_info)
}