pub mod fileapi;
pub mod helper;
use crate::file::fileapi::close_handle;
use winapi::um::winnt::HANDLE;

#[derive(Debug)]
pub struct FileHandle(pub HANDLE);
impl FileHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
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
