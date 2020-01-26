use crate::errors::WinThingError;
use crate::file::FileHandle;
use std::fs::File;
use std::os::windows::io::FromRawHandle;
use std::ptr::null_mut;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::namedpipeapi::CreateNamedPipeW;
use winapi::um::winbase::{
    PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
};

use winapi::um::fileapi::CreateFileW;
use winapi::um::fileapi::OPEN_EXISTING;
use winapi::um::winnt::GENERIC_WRITE;

pub fn create_pipe(pipe_name: &str) -> Result<File, WinThingError> {
    let mut path_u16: Vec<u16> = pipe_name.to_string().encode_utf16().collect();
    path_u16.resize(path_u16.len() + 1, 0);

    let handle = unsafe {
        CreateFileW(
            path_u16.as_ptr(),
            GENERIC_WRITE,
            0,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(WinThingError::from_windows_last_error());
    }

    let file = unsafe { File::from_raw_handle(handle) };

    Ok(file)
}
