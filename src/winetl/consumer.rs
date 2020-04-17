use crate::errors::WinThingError;
use crate::winetl::evntrace::{
    open_trace, 
    process_trace
};
use crate::winetl::TraceHandle;
use std::mem;
use winapi::shared::evntrace::EVENT_TRACE_LOGFILEW;
use winapi::um::evntcons::{
    PEVENT_RECORD, 
    PROCESS_TRACE_MODE_EVENT_RECORD, 
    PROCESS_TRACE_MODE_REAL_TIME,
};

unsafe extern "system" fn process_event(
    _p_event: PEVENT_RECORD
) {
    println!("process_event");
}


/// Represent an easy way to handle EVENT_TRACE_LOGFILEW
/// 
pub struct EventTraceLogFile(EVENT_TRACE_LOGFILEW);
impl EventTraceLogFile {
    /// Create a EventTraceLogFile from a logfile name
    /// 
    pub fn from_logfile(
        logfile_name: &str
    ) -> Self {
        // LogFileName buffer
        let mut logfile_name_u16: Vec<u16> = logfile_name.encode_utf16().collect();
        // Add null terminater
        logfile_name_u16.resize(logfile_name.len() + 1, 0);

        let mut inner: EVENT_TRACE_LOGFILEW = unsafe {
            mem::zeroed()
        };

        // Set logfile name
        inner.LogFileName = logfile_name_u16.as_mut_ptr();

        // Set mode
        unsafe {
            let process_trace_mode = inner.u1.ProcessTraceMode_mut();
            *process_trace_mode = PROCESS_TRACE_MODE_EVENT_RECORD;
        }

        // Set callback
        unsafe {
            let callback = inner.u2.EventRecordCallback_mut();
            *callback = Some(process_event);
        }

        Self( inner )
    }

    /// Create a EventTraceLogFile from a logger name
    /// 
    pub fn from_logger(
        logger_name: &str
    ) -> Self {
        // LoggerName buffer
        let mut logger_name_u16: Vec<u16> = logger_name.encode_utf16().collect();
        // Add null terminater
        logger_name_u16.resize(logger_name.len() + 1, 0);

        let mut inner: EVENT_TRACE_LOGFILEW = unsafe {
            mem::zeroed()
        };

        // Set logfile name
        inner.LoggerName = logger_name_u16.as_mut_ptr();

        // Set mode
        unsafe {
            let process_trace_mode = inner.u1.ProcessTraceMode_mut();
            *process_trace_mode =  PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
        }

        // Set callback
        unsafe {
            let callback = inner.u2.EventRecordCallback_mut();
            *callback = Some(process_event);
        }

        Self ( inner )
    }

    /// Set kernel trace flag
    ///
    pub fn is_kernel_trace(mut self, flag: u32) -> Self {
        self.0.IsKernelTrace = flag;
        self
    }

    /// Get the consumer for this EventTraceLogFile
    pub fn get_consumer(self) -> TraceConsumer {
        TraceConsumer::new(self)
    }
}
impl Into<EVENT_TRACE_LOGFILEW> for EventTraceLogFile {
    fn into(self) -> EVENT_TRACE_LOGFILEW {
        self.0
    }
}


/// Represent an easy way to handle Trace functionality
/// 
pub struct TraceConsumer {
    event_trace_logfile: EventTraceLogFile
}
impl TraceConsumer {
    /// Create a TraceConsumer from a given EventTraceLogFile. I think that in the future
    /// this could be an array of EventTraceLogFile because you can process multiple trace
    /// handles at once (only one realtime).
    pub fn new(
        event_trace_logfile: EventTraceLogFile
    ) -> Self {
        Self {
            event_trace_logfile
        }
    }

    pub fn process_trace(self) -> Result<(), WinThingError> {
        // Get trace handle
        let mut handle = open_trace(
            self.event_trace_logfile.into()
        )?;

        // Process trace
        process_trace(
            &mut handle
        )?;

        Ok(())
    }
}
