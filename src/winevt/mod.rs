pub mod callback;
pub mod channels;
pub mod session;
pub mod subscription;
pub mod variant;
pub mod wevtapi;
use crate::errors::WinThingError;
use winapi::um::winevt::EvtClose;
use winapi::um::winevt::EVT_HANDLE;

#[derive(Debug)]
pub struct EvtHandle(pub EVT_HANDLE);
impl EvtHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl Drop for EvtHandle {
    fn drop(&mut self) {
        let result = unsafe { EvtClose(self.0) };

        if result == 0 {
            let error = WinThingError::from_windows_last_error();
            eprintln!("Error calling EvtClose on EVT_HANDLE: {}", error.message);
        }
    }
}
