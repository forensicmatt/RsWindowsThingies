pub mod channels;
pub mod variant;
pub mod wevtapi;
pub mod callback;
pub mod subscription;
use winapi::um::winevt::EvtClose;
use winapi::um::winevt::EVT_HANDLE;


#[derive(Debug)]
pub struct EvtHandle(EVT_HANDLE);
impl Drop for EvtHandle {
    fn drop(&mut self) {
        unsafe {
            EvtClose(
                self.0
            );
        }
    }
}