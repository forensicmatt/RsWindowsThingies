use crate::errors::WinThingError;
use crate::winetl::TraceHandle;
use std::ptr::null_mut;
use winapi::shared::evntrace::OpenTraceW;
use winapi::shared::evntrace::ProcessTrace;
use winapi::shared::evntrace::*;
use winapi::shared::winerror::ERROR_SUCCESS;


/// ETW_APP_DECLSPEC_DEPRECATED TRACEHANDLE WMIAPI OpenTraceW(
///   PEVENT_TRACE_LOGFILEW Logfile
/// );
pub fn open_trace(mut logfile: EVENT_TRACE_LOGFILEW) -> Result<TraceHandle, WinThingError> {
    let handle = unsafe {
        OpenTraceW(
            &mut logfile
        )
    };

    if handle == INVALID_PROCESSTRACE_HANDLE {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(
        TraceHandle(handle)
    )
}

/// ULONG ProcessTrace(
///   _In_ PTRACEHANDLE HandleArray,
///   _In_ ULONG        HandleCount,
///   _In_ LPFILETIME   StartTime,
///   _In_ LPFILETIME   EndTime
/// );
pub fn process_trace(handle: &mut TraceHandle) -> Result<(), WinThingError> {
    let result = unsafe {
        ProcessTrace(
            &mut handle.0,
            1,
            null_mut(),
            null_mut()
        )
    };

    if result != ERROR_SUCCESS {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(())
}
