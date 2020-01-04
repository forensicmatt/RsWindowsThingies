pub mod consumer;
pub mod evntrace;
pub mod trace;
pub mod publisher;
use winapi::shared::evntrace::TRACEHANDLE;
use winapi::shared::evntrace::CloseTrace;
use crate::errors::WinThingError;


#[derive(Debug)]
pub struct TraceHandle(pub TRACEHANDLE);
impl TraceHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl Drop for TraceHandle {
    fn drop(&mut self) {
        let result = unsafe {
            CloseTrace(
                self.0
            )
        };

        if result == 0 {
            let error = WinThingError::from_windows_last_error();
            eprintln!("Error calling CloseTrace on TRACEHANDLE: {}", error.message);
        }
    }
}