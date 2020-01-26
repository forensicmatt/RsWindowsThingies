use std::ffi::OsString;
use std::ptr::null_mut;
use winapi::um::winevt::*;
use winapi::ctypes::c_void;
use winapi::um::winevt::EvtClose;
use std::os::windows::prelude::*;
use winapi::shared::minwindef::{DWORD};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::ERROR_NO_MORE_ITEMS;
use winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER;
use crate::winevt::EvtHandle;
use crate::errors::WinThingError;
use crate::winevt::variant::EvtVariant;
use crate::winevt::callback::CallbackContext;


/// BOOL EvtRender(
///   EVT_HANDLE Context,
///   EVT_HANDLE Fragment,
///   DWORD      Flags,
///   DWORD      BufferSize,
///   PVOID      Buffer,
///   PDWORD     BufferUsed,
///   PDWORD     PropertyCount
/// );
pub fn evt_render(
    event_handle: EVT_HANDLE
) -> Result<String, WinThingError> {
    let mut buffer_used: DWORD = 0;
    let mut property_count: DWORD = 0;

    let context = null_mut();
    let flags = EvtRenderEventXml;

    let result = unsafe {
        EvtRender(
            context,
            event_handle as _,
            flags,
            0,
            null_mut(),
            &mut buffer_used,
            &mut property_count
        )
    };

    // We expect this to fail but return the buffer size needed.
    if result == 0 {
        let last_error: DWORD = unsafe {
            GetLastError()
        };

        if last_error == ERROR_INSUFFICIENT_BUFFER {
            let buffer: Vec<u16> = vec![0; buffer_used as usize];

            let result = unsafe {
                EvtRender(
                    context,
                    event_handle as _,
                    flags,
                    buffer.len() as _,
                    buffer.as_ptr() as _,
                    &mut buffer_used,
                    &mut property_count
                )
            };

            if result != 0 {
                let mut index = buffer_used as usize - 1;

                // Buffers can be null padded. We want to trim the null chars.
                match buffer.iter().position(|&x| x == 0) {
                    Some(i) => {
                        index = i;
                    },
                    None => {}
                }

                let xml_string = OsString::from_wide(
                    &buffer[..index]
                ).to_string_lossy().to_string();

                return Ok(xml_string);
            } else {
                let last_error: DWORD = unsafe {
                    GetLastError()
                };

                return Err(WinThingError::os_error(
                    last_error as i32
                ));
            }
        } else {
            return Err(WinThingError::os_error(
                last_error as i32
            ));
        }
    } else {
        Err(
            WinThingError::unhandled(
                "Expected Error on first EvtRender call.".to_owned()
            )
        )
    }
}


/// DWORD EvtSubscribeCallback(
///   EVT_SUBSCRIBE_NOTIFY_ACTION Action,
///   PVOID UserContext,
///   EVT_HANDLE Event
/// )
pub extern "system" fn evt_subscribe_callback(
    action: EVT_SUBSCRIBE_NOTIFY_ACTION,
    user_context: *mut c_void,
    event_handle: EVT_HANDLE
) -> u32 {
    if action != EvtSubscribeActionDeliver {
        error!("Expected EvtSubscribeActionDeliver for evt_subscribe_callback but found {:?}", action);
        return 0;
    }

    let user_context: *mut CallbackContext = user_context as *mut _;

    match evt_render(event_handle) {
        Ok(xml_event) => {
            unsafe {
                user_context.as_ref().unwrap().handle_record(
                    xml_event
                );
            }
        },
        Err(e) => {
            error!("Error calling evt_render(): {:?}", e);
        }
    }

    // Close the EVT_HANDLE
    unsafe {
        EvtClose(event_handle);
    }

    return 0;
}


/// EVT_HANDLE EvtSubscribe(
///   EVT_HANDLE             Session,
///   HANDLE                 SignalEvent,
///   LPCWSTR                ChannelPath,
///   LPCWSTR                Query,
///   EVT_HANDLE             Bookmark,
///   PVOID                  Context,
///   EVT_SUBSCRIBE_CALLBACK Callback,
///   DWORD                  Flags
/// );
pub fn register_event_callback(
        session: &Option<EvtHandle>,
        channel_path: &String, 
        query: Option<String>,
        flags: Option<u32>,
        context: &mut CallbackContext
) -> Result<EvtHandle, WinThingError> {
    let session = match session {
        Some(s) => s.0,
        None => null_mut()
    };
    
    // This is null becuase we are using a callback
    let signal_event = null_mut();

    // Create the wide string buffer
    let mut channel_path_u16 : Vec<u16> = channel_path.encode_utf16().collect();
    channel_path_u16.resize(channel_path.len() + 1, 0);

    // Get the query string, or if None was passed, make it *
    let query_str = match query {
        Some(q) => q,
        None => "*".to_owned()
    };
    let mut query_str_u16 : Vec<u16> = query_str.encode_utf16().collect();
    query_str_u16.resize(query_str.len() + 1, 0);

    // Bookmarks are not currently implemented
    let bookmark = null_mut();

    let flags = match flags {
        Some(f) => f,
        None => EvtSubscribeToFutureEvents
    };

    // This handle will need to be closed when the subscription is done...
    let subscription_handle = unsafe {
        EvtSubscribe(
            session,
            signal_event,
            channel_path_u16.as_ptr(),
            query_str_u16.as_ptr(),
            bookmark,
            context as *mut _ as *mut c_void,
            Some(evt_subscribe_callback),
            flags
        )
    };

    if subscription_handle.is_null() {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(
        EvtHandle(
            subscription_handle
        )
    )
}


/// EVT_HANDLE EvtOpenPublisherEnum(
///   EVT_HANDLE Session,
///   DWORD      Flags
/// );
pub fn evt_open_publisher_enum(
    session: &Option<EvtHandle>
) -> Result<EvtHandle, WinThingError> {
    let session = match session {
        Some(s) => s.0,
        None => null_mut()
    };

    let enum_handle = unsafe {
        EvtOpenPublisherEnum(
            session,
            0
        )
    };

    if enum_handle.is_null() {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(
        EvtHandle(enum_handle)
    )
}


/// BOOL EvtNextPublisherId(
///   EVT_HANDLE PublisherEnum,
///   DWORD      PublisherIdBufferSize,
///   LPWSTR     PublisherIdBuffer,
///   PDWORD     PublisherIdBufferUsed
/// );
pub fn evt_next_publisher_id(
    publisher_enum: &EvtHandle
) -> Result<Option<String>, WinThingError> {
    let mut buffer_used: DWORD = 0;

    let result = unsafe {
        EvtNextPublisherId(
            publisher_enum.0,
            0,
            null_mut(),
            &mut buffer_used
        )
    };

    if result == 0 {
        let last_error: DWORD = unsafe {
            GetLastError()
        };

        if last_error == ERROR_INSUFFICIENT_BUFFER {
            let buffer: Vec<u16> = vec![0; buffer_used as usize];

            let result = unsafe {
                EvtNextPublisherId(
                    publisher_enum.0,
                    buffer.len() as _,
                    buffer.as_ptr() as _,
                    &mut buffer_used
                )
            };

            if result != 0 {
                let provider_name = OsString::from_wide(
                    &buffer[..buffer_used as usize - 1]
                ).to_string_lossy().to_string();

                return Ok(Some(provider_name));
            }
            else {
                return Err(
                    WinThingError::from_windows_last_error()
                );
            }
        }
        else if last_error == ERROR_NO_MORE_ITEMS {
            return Ok(None);
        }
        else {
            return Err(
                WinThingError::from_windows_error_code(
                    last_error
                )
            );
        }
    } 
    else {
        Err(
            WinThingError::unhandled(
                "Expected Error on first EvtRender call.".to_owned()
            )
        )
    }
}


/// EVT_HANDLE EvtOpenPublisherMetadata(
///   EVT_HANDLE Session,
///   LPCWSTR    PublisherId,
///   LPCWSTR    LogFilePath,
///   LCID       Locale,
///   DWORD      Flags
/// );
pub fn evt_open_publisher_metadata(
    session: &Option<EvtHandle>,
    publisher_id: Option<String>,
    logfile_path: Option<String>
) -> Result<EvtHandle, WinThingError> {
    let session = match session {
        Some(s) => s.0,
        None => null_mut()
    };

    let mut string_u16: Vec<u16>;
    let publisher_id = match publisher_id {
        Some(s) => {
            string_u16 = s.encode_utf16().collect();
            // Needs to be null terminated
            string_u16.resize(s.len() + 1, 0);
            string_u16.as_ptr()
        },
        None => null_mut()
    };

    let logfile_path = match logfile_path {
        Some(s) => {
            let mut string_u16 : Vec<u16> = s.encode_utf16().collect();
            string_u16.resize(s.len() + 1, 0);
            string_u16.as_ptr()
        },
        None => null_mut()
    };

    let meta_handle = unsafe {
        EvtOpenPublisherMetadata(
            session,
            publisher_id as _,
            logfile_path as _,
            0,
            0
        )
    };

    if meta_handle.is_null() {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(
        EvtHandle(meta_handle)
    )
}


/// BOOL EvtGetPublisherMetadataProperty(
///   EVT_HANDLE                         PublisherMetadata,
///   EVT_PUBLISHER_METADATA_PROPERTY_ID PropertyId,
///   DWORD                              Flags,
///   DWORD                              PublisherMetadataPropertyBufferSize,
///   PEVT_VARIANT                       PublisherMetadataPropertyBuffer,
///   PDWORD                             PublisherMetadataPropertyBufferUsed
/// );
/// https://docs.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtgetpublishermetadataproperty
pub fn evt_get_publisher_metadata_property(
    publisher_metadata: &EvtHandle,
    property_id: EVT_PUBLISHER_METADATA_PROPERTY_ID,
) -> Result<EvtVariant, WinThingError> {
    let mut buffer_used: DWORD = 0;

    let result = unsafe {
        EvtGetPublisherMetadataProperty(
            publisher_metadata.0,
            property_id,
            0,
            0,
            null_mut(),
            &mut buffer_used
        )
    };

    // We expect this to fail but return the buffer size needed.
    if result == 0 {
        let last_error: DWORD = unsafe {
            GetLastError()
        };

        if last_error == ERROR_INSUFFICIENT_BUFFER {
            let mut buffer: Vec<u8> = vec![0; buffer_used as usize];

            let result = unsafe {
                EvtGetPublisherMetadataProperty(
                    publisher_metadata.0,
                    property_id,
                    0,
                    buffer.len() as _,
                    buffer.as_mut_ptr() as *mut EVT_VARIANT,
                    &mut buffer_used
                )
            };

            if result != 0 {
                let variant: EVT_VARIANT = unsafe {
                    std::ptr::read(
                        buffer.as_ptr() as *const _
                    ) 
                };

                return Ok(
                    EvtVariant(variant)
                );
            }
            else {
                return Err(
                    WinThingError::from_windows_last_error()
                );
            }
        }
        else {
            return Err(
                WinThingError::from_windows_error_code(
                    last_error
                )
            );
        }
    }
    else {
        Err(
            WinThingError::unhandled(
                "Expected Error on first EvtGetPublisherMetadataProperty call.".to_owned()
            )
        )
    }
}


/// BOOL EvtGetObjectArraySize(
///   EVT_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray,
///   PDWORD                           ObjectArraySize
/// );
pub fn evt_get_object_array_size(
    object_array: &EvtHandle
) -> Result<u32, WinThingError> {
    let mut object_array_size: DWORD = 0;

    let result = unsafe {
        EvtGetObjectArraySize(
            object_array.0,
            &mut object_array_size
        )
    };

    if result == 0 {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(object_array_size)
}


/// BOOL EvtGetObjectArrayProperty(
///   EVT_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray,
///   DWORD                            PropertyId,
///   DWORD                            ArrayIndex,
///   DWORD                            Flags,
///   DWORD                            PropertyValueBufferSize,
///   PEVT_VARIANT                     PropertyValueBuffer,
///   PDWORD                           PropertyValueBufferUsed
/// );
pub fn evt_get_object_array_property(
    object_array: &EvtHandle,
    index: DWORD,
    property_id: EVT_PUBLISHER_METADATA_PROPERTY_ID
) -> Result<EvtVariant, WinThingError> {
    let mut buffer_used: DWORD = 0;

    let result = unsafe {
        EvtGetObjectArrayProperty(
            object_array.0,
            property_id,
            index,
            0,
            0,
            null_mut(),
            &mut buffer_used
        )
    };

    // We expect this to fail but return the buffer size needed.
    if result == 0 {
        let last_error: DWORD = unsafe {
            GetLastError()
        };

        if last_error == ERROR_INSUFFICIENT_BUFFER {
            let mut buffer: Vec<u8> = vec![0; buffer_used as usize];

            let result = unsafe {
                EvtGetObjectArrayProperty(
                    object_array.0,
                    property_id,
                    index,
                    0,
                    buffer.len() as _,
                    buffer.as_mut_ptr() as *mut EVT_VARIANT,
                    &mut buffer_used
                )
            };

            if result != 0 {
                let variant: EVT_VARIANT = unsafe {
                    std::ptr::read(
                        buffer.as_ptr() as *const _
                    ) 
                };

                return Ok(
                    EvtVariant(
                        variant
                    )
                );
            }
            else {
                return Err(
                    WinThingError::from_windows_last_error()
                );
            }
        }
        else {
            return Err(
                WinThingError::from_windows_error_code(
                    last_error
                )
            );
        }
    }
    else {
        Err(
            WinThingError::unhandled(
                "Expected Error on first EvtGetObjectArrayProperty call.".to_owned()
            )
        )
    }
}


/// BOOL EvtFormatMessage(
///   EVT_HANDLE   PublisherMetadata,
///   EVT_HANDLE   Event,
///   DWORD        MessageId,
///   DWORD        ValueCount,
///   PEVT_VARIANT Values,
///   DWORD        Flags,
///   DWORD        BufferSize,
///   LPWSTR       Buffer,
///   PDWORD       BufferUsed
/// );
pub fn evt_format_message(
    publisher_metadata: Option<&EvtHandle>,
    event: Option<&EvtHandle>,
    message_id: DWORD
) -> Result<String, WinThingError> {
    let mut buffer_used: DWORD = 0;

    let publisher_metadata = match publisher_metadata {
        Some(h) => h.0,
        None => null_mut(),
    };

    let event = match event {
        Some(h) => h.0,
        None => null_mut(),
    };

    let flags = EvtFormatMessageId;
    let result = unsafe {
        EvtFormatMessage(
            publisher_metadata,
            event,
            message_id,
            0,
            null_mut(),
            flags,
            0,
            null_mut(),
            &mut buffer_used
        )
    };

    // We expect this to fail but return the buffer size needed.
    if result == 0 {
        let last_error: DWORD = unsafe {
            GetLastError()
        };

        if last_error == ERROR_INSUFFICIENT_BUFFER {
            let buffer: Vec<u16> = vec![0; buffer_used as usize];

            let result = unsafe {
                EvtFormatMessage(
                    publisher_metadata,
                    event,
                    message_id,
                    0,
                    null_mut(),
                    flags,
                    buffer.len() as _,
                    buffer.as_ptr() as _,
                    &mut buffer_used
                )
            };

            if result != 0 {
                // Remove terminating null
                let message_string = OsString::from_wide(
                    &buffer[..buffer_used as usize - 1]
                ).to_string_lossy().to_string();

                return Ok(message_string);
            }
            else {
                return Err(
                    WinThingError::from_windows_last_error()
                );
            }
        }
        else {
            return Err(
                WinThingError::from_windows_error_code(
                    last_error
                )
            );
        }
    }
    else {
        Err(
            WinThingError::unhandled(
                "Expected Error on first EvtFormatMessage call.".to_owned()
            )
        )
    }
}