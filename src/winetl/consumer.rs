use crate::errors::WinThingError;
use crate::winetl::evntrace::{open_trace, process_trace};
use crate::winetl::TraceHandle;
use std::mem;
use winapi::shared::evntrace::EVENT_TRACE_LOGFILEW;
use winapi::um::evntcons::{
    PEVENT_RECORD, PROCESS_TRACE_MODE_EVENT_RECORD, PROCESS_TRACE_MODE_REAL_TIME,
};

unsafe extern "system" fn process_event(_p_event: PEVENT_RECORD) {
    println!("process_event");
}

pub struct TraceConsumer {
    trace_handle: TraceHandle,
}
impl TraceConsumer {
    pub fn new(logger_name: String) -> Result<Self, WinThingError> {
        // logger_name buffer
        let mut logger_name_u16: Vec<u16> = logger_name.encode_utf16().collect();
        logger_name_u16.resize(logger_name.len() + 1, 0);

        let mut event_trace_logfile: EVENT_TRACE_LOGFILEW = unsafe { mem::zeroed() };

        // Set logger name
        event_trace_logfile.LoggerName = logger_name_u16.as_mut_ptr();
        // Set mode
        unsafe {
            let mut _mode = event_trace_logfile.u1.ProcessTraceMode_mut();
            *_mode = PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
        }
        // Set callback
        unsafe {
            let mut _callback = event_trace_logfile.u2.EventRecordCallback_mut();
            *_callback = Some(process_event);
        }

        // Get trace handle
        let mut handle = open_trace(event_trace_logfile)?;

        // Process trace
        process_trace(&mut handle)?;

        Ok(Self {
            trace_handle: handle,
        })
    }

    pub fn start(&mut self) -> Result<(), WinThingError> {
        // Process trace
        process_trace(&mut self.trace_handle)?;

        Ok(())
    }
}
