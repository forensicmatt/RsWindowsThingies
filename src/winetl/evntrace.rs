use crate::errors::WinThingError;
use crate::winetl::TraceHandle;
use std::ptr::null_mut;
use winapi::shared::{
    guiddef::GUID,
    ntdef::{
        PVOID,
        ULONG,
        ULONGLONG
    },
    evntrace::{
        OpenTraceW,
        ProcessTrace,
        PTRACE_ENABLE_INFO
    },
    winerror::{
        ERROR_SUCCESS,
        ERROR_INSUFFICIENT_BUFFER
    }
};
use winapi::um::{
    eventtrace,
    evntcons::PEVENT_RECORD
};
use winapi::shared::evntrace::*;


/// TDHSTATUS TdhGetEventInformation(
///   PEVENT_RECORD     Event,
///   ULONG             TdhContextCount,
///   PTDH_CONTEXT      TdhContext,
///   PTRACE_EVENT_INFO Buffer,
///   PULONG            BufferSize
/// );
pub fn get_event_information(
    p_event: PEVENT_RECORD
) -> Result<(u32, eventtrace::PTRACE_EVENT_INFO), WinThingError> {
    let mut buff_size = 0;

    let result = unsafe {
        eventtrace::TdhGetEventInformation(
            p_event as eventtrace::PEVENT_RECORD,
            0,
            null_mut(),
            null_mut(),
            &mut buff_size
        )
    };

    // Get buffer size
    if result != ERROR_INSUFFICIENT_BUFFER {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    let buff = vec![0u8; buff_size as usize];
    let info = buff.as_ptr() as eventtrace::PTRACE_EVENT_INFO;

    // Populate the TRACE_EVENT_INFO struct
    let result = unsafe {
        eventtrace::TdhGetEventInformation(
            p_event as eventtrace::PEVENT_RECORD, 
            0, 
            null_mut(), 
            info, 
            &mut buff_size
        )
    };

    if result != ERROR_SUCCESS {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    return Ok((buff_size, info))
}


/// ETW_APP_DECLSPEC_DEPRECATED TRACEHANDLE WMIAPI OpenTraceW(
///   PEVENT_TRACE_LOGFILEW Logfile
/// );
pub fn open_trace(
    mut logfile: EVENT_TRACE_LOGFILEW
) -> Result<TraceHandle, WinThingError> {
    // Get the trace handle
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
pub fn process_trace(
    handle: &mut TraceHandle
) -> Result<(), WinThingError> {
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
