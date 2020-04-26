pub mod fileapi;
pub mod helper;
pub mod pipe;

use crate::file::fileapi::close_handle;
use std::fs::File;
use std::os::windows::io::FromRawHandle;
use winapi::um::winnt::HANDLE;

#[derive(Debug)]
pub struct FileHandle(pub HANDLE);
impl FileHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn get_file(&self) -> File {
        unsafe { File::from_raw_handle(self.0 as _) }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        match close_handle(self.0) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error calling FileHandle on HANDLE: {}", e.message);
            }
        }
    }
}
