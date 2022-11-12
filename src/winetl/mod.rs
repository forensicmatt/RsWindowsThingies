pub mod consumer;
pub mod evntrace;
pub mod publisher;
pub mod trace;
use crate::errors::WinThingError;
use winapi::shared::evntrace::CloseTrace;
use winapi::shared::evntrace::TRACEHANDLE;

#[derive(Debug)]
pub struct TraceHandle(pub TRACEHANDLE);
impl TraceHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl Drop for TraceHandle {
    fn drop(&mut self) {
        let result = unsafe { CloseTrace(self.0) };

        if result == 0 {
            let error = WinThingError::from_windows_last_error();
            eprintln!("Error calling CloseTrace on TRACEHANDLE: {}", error.message);
        }
    }
}
