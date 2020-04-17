use crate::errors::format_win_error;
use crate::errors::WinThingError;
use std::mem;
use winapi::shared::evntrace::{
    KERNEL_LOGGER_NAME
};
use winapi::shared::evntrace;
use winapi::shared::minwindef::DWORD;
use winapi::shared::evntrace::StartTraceW;
use winapi::shared::evntrace::StopTraceW;
use winapi::shared::evntrace::EVENT_TRACE_PROPERTIES;
use winapi::shared::evntrace::PEVENT_TRACE_PROPERTIES;
use winapi::shared::evntrace::TRACEHANDLE;
use winapi::shared::evntrace::*;
use winapi::shared::winerror;
use winapi::shared::wmistr::WNODE_FLAG_TRACED_GUID;


/// The default flags to use for kernel trace
pub const DEFAULT_TRACE_FLAGS: DWORD = EVENT_TRACE_FLAG_DISK_FILE_IO | 
    EVENT_TRACE_FLAG_DISK_IO | EVENT_TRACE_FLAG_REGISTRY;


///https://docs.microsoft.com/en-us/windows/win32/api/evntrace/ns-evntrace-event_trace_properties
pub struct TraceProperties {
    pub properties: PEVENT_TRACE_PROPERTIES
}

impl TraceProperties {
    /// Just a note:
    /// You should not use real-time mode because the supported event rate is much lower than reading from 
    /// the log file (events may be dropped). Also, the event order is not guaranteed on computers with 
    /// multiple processors. The real-time mode is more suitable for low-traffic, notification type events.
    pub fn new(
        session_name: &str,
        logfile_name: Option<&str>
    ) -> Self {
        // Size of EVENT_TRACE_PROPERTIES
        let event_trace_size = mem::size_of::<EVENT_TRACE_PROPERTIES>();
        // The session name
        let session_name_u16: Vec<u16> = session_name.encode_utf16().collect();

        // - MSDN -
        // When you allocate the memory for this structure, you must allocate enough memory to include 
        // the session name and log file name following the structure. The session name must come before 
        // the log file name in memory. You must copy the log file name to the offset but you do not copy
        // the session name to the offset—the StartTrace function copies the name for you.
        let mut var_length: usize = (session_name_u16.len() + 1) * 2;
        
        let mut logfile_name_offset = 0;
        if let Some(logfile_name) = logfile_name {
            // The logfile_name_offset will be the size of the event + the size (+null u16) of the
            // session name.
            logfile_name_offset = event_trace_size + var_length;
            // The logfile name
            let logfile_name_u16: Vec<u16> = logfile_name.encode_utf16().collect();
            var_length += (logfile_name_u16.len() + 1) * 2;
        }

        let mut etrace_prop_buf = vec![0u8; event_trace_size + var_length];
        let etrace_prop = etrace_prop_buf.as_mut_ptr() as PEVENT_TRACE_PROPERTIES;

        unsafe {
            // Wnode.BufferSize is the total size of this EVENT_TRACE_PROPERTIES
            (*etrace_prop).Wnode.BufferSize = etrace_prop_buf.len() as u32;
            (*etrace_prop).Wnode.Guid = evntrace::SystemTraceControlGuid;
            // Wnode.ClientContext controls timestamp context. Review docs for information including
            // how to resolve event timestamps
            (*etrace_prop).Wnode.ClientContext = 2;
            (*etrace_prop).Wnode.Flags = WNODE_FLAG_TRACED_GUID;

            (*etrace_prop).EnableFlags = DEFAULT_TRACE_FLAGS;
            (*etrace_prop).MaximumFileSize = 1;
            (*etrace_prop).LogFileMode = evntrace::EVENT_TRACE_REAL_TIME_MODE;
            (*etrace_prop).LoggerNameOffset = logfile_name_offset as u32;
        }

        Self {
            properties: etrace_prop
        }
    }
}


/// TraceSession allows for easy consumer operations.
/// 
pub struct TraceSession {
    session_name: String,
    providers: Vec<String>
}

impl TraceSession {
    /// Create a session.
    pub fn new(
        session_name: String, 
        providers: Vec<String>
    ) -> Result<Self, WinThingError> {
        if providers.len() < 1 {
            return Err(
                WinThingError::general_error(
                    "TraceSession must have at least 1 provider.".to_string()
                )
            );
        }

        Ok(
            Self {
                session_name,
                providers
            }
        )
    }

    /// Check if session is a kernel trace based off the session name.
    /// KERNEL_LOGGER_NAME => "NT Kernel Logger"
    pub fn is_kernel_trace(&self) -> bool {
        if self.session_name == KERNEL_LOGGER_NAME {
            true
        } else {
            false
        }
    }

    /// Start this session.
    pub fn start_session(&self) {

    }
}

pub struct TraceConsumer {
    _session_name: String,
}

/// https://github.com/ROki1988/etw_client/blob/4d3f079468c24ba4b0183a3f4712841cffcc0445/src/main.rs
///
impl TraceConsumer {
    pub fn new(session_name: String) -> Result<Self, WinThingError> {
        // The session name
        let session_name_u16: Vec<u16> = session_name.encode_utf16().collect();
        // let session_name_u16_ptr = session_name_u16.as_ptr() as *mut _;

        let mut etrace_prop_buf =
            vec![0u8; mem::size_of::<EVENT_TRACE_PROPERTIES>() + (session_name_u16.len() + 1) * 2];
        let etrace_prop = etrace_prop_buf.as_mut_ptr() as PEVENT_TRACE_PROPERTIES;

        // let etrace_prop = prop_buf.as_mut_ptr() as PEVENT_TRACE_PROPERTIES;
        // Add session_name size plus 1 null wchar
        // let etrace_prop_size = mem::size_of::<EVENT_TRACE_PROPERTIES>() + session_name_u16.len() * 2;
        // let mut etrace_prop = EVENT_TRACE_PROPERTIES::default();
        unsafe {
            (*etrace_prop).Wnode.BufferSize = etrace_prop_buf.len() as u32;
            (*etrace_prop).Wnode.Guid = evntrace::SystemTraceControlGuid;
            (*etrace_prop).Wnode.ClientContext = 1;
            (*etrace_prop).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
            (*etrace_prop).EnableFlags = EVENT_TRACE_FLAG_REGISTRY;
            (*etrace_prop).MaximumFileSize = 1;
            (*etrace_prop).LogFileMode = evntrace::EVENT_TRACE_REAL_TIME_MODE;
            (*etrace_prop).LoggerNameOffset =
                mem::size_of::<evntrace::EVENT_TRACE_PROPERTIES>() as u32;
        }

        let mut session_handle: TRACEHANDLE = 0 as TRACEHANDLE;

        let result = unsafe { 
            StartTraceW(
                &mut session_handle, 
                session_name_u16.as_ptr(), 
                etrace_prop
            )
        };

        match result {
            winerror::ERROR_SUCCESS | winerror::ERROR_ALREADY_EXISTS => {
                // println!("Success! {}", result);
                let message = format_win_error(Some(result));
                println!("Success: {}", message);
            }
            _ => {
                let message = format_win_error(None);
                println!("Error: {}", message);
                return Err(WinThingError::from_windows_error_code(result));
            }
        }

        let result = unsafe { StopTraceW(session_handle, session_name_u16.as_ptr(), etrace_prop) };

        match result {
            winerror::ERROR_SUCCESS | winerror::ERROR_ALREADY_EXISTS => {
                println!("Success! {}", result);
            }
            _ => {
                return Err(WinThingError::from_windows_error_code(result));
            }
        }

        Ok(Self {
            _session_name: session_name,
        })
    }
}
