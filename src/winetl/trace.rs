use crate::errors::format_win_error;
use crate::errors::WinThingError;
use std::mem;
use winapi::shared::evntrace;
use winapi::shared::evntrace::StartTraceW;
use winapi::shared::evntrace::StopTraceW;
use winapi::shared::evntrace::EVENT_TRACE_PROPERTIES;
use winapi::shared::evntrace::PEVENT_TRACE_PROPERTIES;
use winapi::shared::evntrace::TRACEHANDLE;
use winapi::shared::evntrace::*;
use winapi::shared::winerror;
use winapi::shared::wmistr::WNODE_FLAG_TRACED_GUID;

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

        let result =
            unsafe { StartTraceW(&mut session_handle, session_name_u16.as_ptr(), etrace_prop) };

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
