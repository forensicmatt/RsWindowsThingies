pub mod callback;
pub mod channels;
pub mod session;
pub mod variant;
pub mod wevtapi;

use std::ptr;
use widestring::WideCString;

use crate::errors::WinThingError;
use crate::winevt::callback::CallbackContext;
use crate::winevt::wevtapi::evt_render;
use winapi::ctypes::c_void;

use winapi::um::winevt::*;

#[derive(Debug)]
pub struct EvtHandle(pub EVT_HANDLE);

#[derive(Debug)]
/// A handle which ensures that the callback will not be dropped while the handle is still active and subscribed.
pub struct EvtSubscriptionHandle<'a> {
    inner: EvtHandle,
    ctx: &'a mut CallbackContext,
}

impl<'a> EvtSubscriptionHandle<'a> {
    // Safety: We need to ensure that `CallbackContext` is alive and exclusive when registering a subscription.
    pub fn register(
        session: &Option<EvtHandle>,
        channel_path: &str,
        query: Option<&str>,
        flags: Option<u32>,
        ctx: &'a mut CallbackContext,
    ) -> Result<Self, WinThingError> {
        let session = match session {
            Some(s) => s.0,
            None => ptr::null_mut(),
        };

        // This is null becuase we are using a callback
        let signal_event = ptr::null_mut();

        // Create the wide string buffer
        let channel_path_u16 = WideCString::from_str(channel_path).expect("channel path string");
        let query_str_u16 = WideCString::from_str(query.unwrap_or("*")).expect("query string");

        // Bookmarks are not currently implemented
        let bookmark = ptr::null_mut();
        let flags = flags.unwrap_or(EvtSubscribeToFutureEvents);

        // EVT_HANDLE EvtSubscribe(
        //   EVT_HANDLE             Session,
        //   HANDLE                 SignalEvent,
        //   LPCWSTR                ChannelPath,
        //   LPCWSTR                Query,
        //   EVT_HANDLE             Bookmark,
        //   PVOID                  Context,
        //   EVT_SUBSCRIBE_CALLBACK Callback,
        //   DWORD                  Flags
        // );
        // Safety - We are expected to be the sole owners of `CallbackContext`.
        //          Otherwise Bad Things Could Happen.
        let subscription_handle = unsafe {
            EvtSubscribe(
                session,
                signal_event,
                channel_path_u16.as_ptr(),
                query_str_u16.as_ptr(),
                bookmark,
                ctx as *mut _ as *mut c_void,
                Some(evt_subscribe_callback),
                flags,
            )
        };

        if subscription_handle.is_null() {
            return Err(WinThingError::from_windows_last_error());
        }

        Ok(Self {
            inner: EvtHandle(subscription_handle),
            ctx,
        })
    }
}

/// DWORD EvtSubscribeCallback(
///   EVT_SUBSCRIBE_NOTIFY_ACTION Action,
///   PVOID UserContext,
///   EVT_HANDLE Event
/// )
pub extern "system" fn evt_subscribe_callback(
    action: EVT_SUBSCRIBE_NOTIFY_ACTION,
    // This is assumed to be populated with a live "CallbackContext".
    user_context: *mut c_void,
    // As per https://docs.microsoft.com/en-us/windows/win32/api/winevt/nc-winevt-evt_subscribe_callback
    // We are expected NOT to call EvtClose on these handles.
    event_handle: EVT_HANDLE,
) -> u32 {
    if action != EvtSubscribeActionDeliver {
        error!(
            "Expected EvtSubscribeActionDeliver for evt_subscribe_callback but found {:?}",
            action
        );
        return 0;
    }

    // We cast the context back to a ptr.
    let user_context: *mut CallbackContext = user_context as *mut _;

    match evt_render(event_handle) {
        Ok(xml_event) => unsafe {
            user_context
                .as_ref()
                .expect("CallbackContext cannot be null")
                .handle_record(xml_event);
        },
        Err(e) => {
            error!("Error calling evt_render(): {:?}", e);
        }
    }

    return 0;
}

impl EvtHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl Drop for EvtHandle {
    fn drop(&mut self) {
        let result = unsafe { EvtClose(self.0) };

        if result == 0 {
            let error = WinThingError::from_windows_last_error();
            eprintln!("Error calling EvtClose on EVT_HANDLE: {}", error.message);
        }
    }
}
