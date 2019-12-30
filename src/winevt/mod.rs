pub mod channels;
pub mod variant;
pub mod wevtapi;
pub mod callback;
pub mod subscription;
pub mod session;
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
        unsafe {
            EvtClose(
                self.0
            );
        }
    }
}