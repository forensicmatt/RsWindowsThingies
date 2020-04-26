use std::mem;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::evntrace::EVENT_TRACE_LOGFILEW;

use winapi::um::{
    evntcons::{
        PEVENT_RECORD,
        PROCESS_TRACE_MODE_EVENT_RECORD, 
        PROCESS_TRACE_MODE_REAL_TIME
    }
};
use crate::errors::WinThingError;
use crate::winetl::{
    record::EventRecord,
    evntrace::{
        open_trace, 
        process_trace,
        get_event_information
    }
};


unsafe extern "system" fn process_event(
    p_event: PEVENT_RECORD
) {
    let (buf_size, e_info) = get_event_information(
        p_event
    ).expect("Error getting event info");

    if (*e_info).DecodingSource != 0 {
        return;
    }

    let record = EventRecord::new(
        p_event as _,
        e_info
    );

    let record_value = record.get_value();

    println!("{}", record_value.to_string());
}


/// Represent an easy way to handle EVENT_TRACE_LOGFILEW
/// 
pub struct EventTraceLogFile{
    inner_source: OsString,
    inner: EVENT_TRACE_LOGFILEW
}
impl EventTraceLogFile {
    /// Create a EventTraceLogFile from a logfile name
    /// 
    pub fn from_logfile(
        logfile_name: &str
    ) -> Self {
        let logfile_name: OsString = OsString::from(
            logfile_name
        );

        // LogFileName buffer
        let mut logfile_name_u16: Vec<u16> = logfile_name
            .encode_wide()
            //.chain(Some(0).into_iter())
            .collect();

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

        Self { 
            inner_source: logfile_name,
            inner 
        }
    }

    /// Create a EventTraceLogFile from a logger name
    /// 
    pub fn from_logger(
        logger_name: &str
    ) -> Self {
        let logger_name: OsString = OsString::from(
            logger_name
        );

        // Logger buffer
        let mut logger_name_u16: Vec<u16> = logger_name
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let mut inner: EVENT_TRACE_LOGFILEW = unsafe {
            mem::zeroed()
        };

        // Set logger name
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

        Self { 
            inner_source: logger_name,
            inner 
        }
    }

    /// Set kernel trace flag
    ///
    pub fn is_kernel_trace(mut self, flag: u32) -> Self {
        self.inner.IsKernelTrace = flag;
        self
    }

    /// Get the consumer for this EventTraceLogFile
    pub fn get_consumer(self) -> TraceConsumer {
        TraceConsumer::new(self)
    }
}
impl Into<EVENT_TRACE_LOGFILEW> for EventTraceLogFile {
    fn into(self) -> EVENT_TRACE_LOGFILEW {
        self.inner
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
