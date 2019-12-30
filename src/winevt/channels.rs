use std::ptr::null_mut;
use std::ffi::OsString;
use std::os::windows::prelude::*;
use serde_json::Value;
use winapi::um::winevt::*;
use winapi::um::winevt::EVT_HANDLE;
use winapi::um::winevt::EvtOpenChannelEnum;
use winapi::um::winevt::EvtNextChannelPath;
use winapi::um::winevt::EvtOpenChannelConfig;
use winapi::um::winevt::EvtGetChannelConfigProperty;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER;
use crate::winevt::EvtHandle;
use crate::errors::WinThingError;
use crate::winevt::variant::EvtVariant;
use crate::winevt::variant::VariantValue;

const CHANNEL_PROPERTIES: [(&str, u32); 21] = [
    ("EvtChannelConfigEnabled", EvtChannelConfigEnabled),
    ("EvtChannelConfigIsolation", EvtChannelConfigIsolation),
    ("EvtChannelConfigType", EvtChannelConfigType),
    ("EvtChannelConfigOwningPublisher", EvtChannelConfigOwningPublisher),
    ("EvtChannelConfigClassicEventlog", EvtChannelConfigClassicEventlog),
    ("EvtChannelConfigAccess", EvtChannelConfigAccess),
    ("EvtChannelLoggingConfigRetention", EvtChannelLoggingConfigRetention),
    ("EvtChannelLoggingConfigAutoBackup", EvtChannelLoggingConfigAutoBackup),
    ("EvtChannelLoggingConfigMaxSize", EvtChannelLoggingConfigMaxSize),
    ("EvtChannelLoggingConfigLogFilePath", EvtChannelLoggingConfigLogFilePath),
    ("EvtChannelPublishingConfigLevel", EvtChannelPublishingConfigLevel),
    ("EvtChannelPublishingConfigKeywords", EvtChannelPublishingConfigKeywords),
    ("EvtChannelPublishingConfigControlGuid", EvtChannelPublishingConfigControlGuid),
    ("EvtChannelPublishingConfigBufferSize", EvtChannelPublishingConfigBufferSize),
    ("EvtChannelPublishingConfigMinBuffers", EvtChannelPublishingConfigMinBuffers),
    ("EvtChannelPublishingConfigMaxBuffers", EvtChannelPublishingConfigMaxBuffers),
    ("EvtChannelPublishingConfigLatency", EvtChannelPublishingConfigLatency),
    ("EvtChannelPublishingConfigClockType", EvtChannelPublishingConfigClockType),
    ("EvtChannelPublishingConfigSidType", EvtChannelPublishingConfigSidType),
    ("EvtChannelPublisherList", EvtChannelPublisherList),
    ("EvtChannelPublishingConfigFileMax", EvtChannelPublishingConfigFileMax)
];


#[allow(dead_code)]
pub struct ChannelConfig {
    name: String,
    handle: EvtHandle
}
impl ChannelConfig {
    pub fn new(channel: String) -> Result<Self, WinThingError> {
        let handle = evt_open_channel_config(
            &channel
        )?;

        Ok(
            Self {
                name: channel,
                handle: handle
            }
        )
    }

    /// Check if this channel can be subscribed to
    pub fn can_subscribe(&self) -> bool {
        match self.get_config_type() {
            Some(i) => {
                if i as u32 == EvtChannelTypeOperational || i as u32 == EvtChannelTypeAdmin {
                    true
                }
                else {
                    false
                }
            },
            None => false
        }
    }

    pub fn get_config_isolation(&self) -> Option<u64> {
        match evt_get_channel_config_property(
            &self.handle, EvtChannelConfigIsolation
        ) {
            Some(v) => {
                match v.get_variant_value() {
                    Ok(variant_value) => {
                        match variant_value {
                            VariantValue::UInt(i) => return Some(i),
                            other => {
                                error!("Not expecting {:?}", other);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error getting variant value for EvtChannelConfigIsolation: {:?}", e);
                    }
                }
            },
            None => {}
        }

        None
    }

    pub fn get_log_file_path(&self) -> Option<String> {
        match evt_get_channel_config_property(
            &self.handle, EvtChannelLoggingConfigLogFilePath
        ) {
            Some(v) => {
                match v.get_variant_value() {
                    Ok(variant_value) => {
                        match variant_value {
                            VariantValue::String(s) => return Some(s),
                            other => {
                                error!("Not expecting {:?}", other);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error getting variant value for EvtChannelConfigClassicEventlog: {:?}", e);
                    }
                }
            },
            None => {}
        }

        None
    }

    pub fn get_config_type(&self) -> Option<u64> {
        match evt_get_channel_config_property(
            &self.handle, EvtChannelConfigType
        ) {
            Some(v) => {
                match v.get_variant_value() {
                    Ok(variant_value) => {
                        match variant_value {
                            VariantValue::UInt(i) => return Some(i),
                            other => {
                                error!("Not expecting {:?}", other);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error getting variant value for EvtChannelConfigType: {:?}", e);
                    }
                }
            },
            None => {}
        }

        None
    }

    pub fn is_classic_event_log(&self) -> bool {
        match evt_get_channel_config_property(
            &self.handle, EvtChannelConfigClassicEventlog
        ) {
            Some(v) => {
                match v.get_variant_value() {
                    Ok(variant_value) => {
                        match variant_value {
                            VariantValue::Boolean(b) => b,
                            other => {
                                error!("Not expecting {:?}", other);
                                false
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error getting variant value for EvtChannelConfigClassicEventlog: {:?}", e);
                        false
                    }
                }
            },
            None => false
        }
    }

    pub fn is_enabled(&self) -> bool {
        match evt_get_channel_config_property(
            &self.handle, EvtChannelConfigEnabled
        ) {
            Some(v) => {
                match v.get_variant_value() {
                    Ok(variant_value) => {
                        match variant_value {
                            VariantValue::Boolean(b) => b,
                            other => {
                                error!("Not expecting {:?}", other);
                                false
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error getting variant value for EvtChannelConfigEnabled: {:?}", e);
                        false
                    }
                }
            },
            None => false
        }
    }

    pub fn to_json_value(&self) -> Result<Value, WinThingError> {
        let mut mapping = json!({});

        for (key, id) in &CHANNEL_PROPERTIES {
            let variant = match evt_get_channel_config_property(
                &self.handle, *id
            ) {
                Some(v) => v,
                None => continue
            };

            match variant.get_json_value() {
                Ok(v) => {
                    mapping[key] = v;
                },
                Err(e) => {
                    error!("Error getting variant value: {:?}", e);
                }
            }
        }

        Ok(mapping)
    }
}


pub fn get_channel_name_list(
    session: &Option<EvtHandle>
) -> Result<Vec<String>, WinThingError> {
    let mut channel_name_list: Vec<String> = Vec::new();

    let channel_enum_handle = evt_open_channel_enum(
        session
    )?;

    loop {
        match evt_next_channel_id(channel_enum_handle.0) {
            None => break,
            Some(ps) => channel_name_list.push(ps)
        }
    }

    Ok(channel_name_list)
}


/// EVT_HANDLE EvtOpenChannelEnum(
///   EVT_HANDLE Session,
///   DWORD      Flags
/// );
pub fn evt_open_channel_enum(
    session: &Option<EvtHandle>
) -> Result<EvtHandle, WinThingError> {
    let session = match session {
        Some(s) => s.0,
        None => null_mut()
    };

    let enum_handle = unsafe {
        EvtOpenChannelEnum(
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


/// BOOL EvtGetChannelConfigProperty(
///   EVT_HANDLE                     ChannelConfig,
///   EVT_CHANNEL_CONFIG_PROPERTY_ID PropertyId,
///   DWORD                          Flags,
///   DWORD                          PropertyValueBufferSize,
///   PEVT_VARIANT                   PropertyValueBuffer,
///   PDWORD                         PropertyValueBufferUsed
/// );
fn evt_get_channel_config_property(evt_handle: &EvtHandle, property_id: EVT_CHANNEL_CONFIG_PROPERTY_ID) -> Option<EvtVariant> {
    let mut buffer_used: DWORD = 0;

    let result = unsafe {
        EvtGetChannelConfigProperty(
            evt_handle.0,
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

            match unsafe {
                EvtGetChannelConfigProperty(
                    evt_handle.0,
                    property_id,
                    0,
                    buffer_used,
                    buffer.as_mut_ptr() as *mut EVT_VARIANT,
                    &mut buffer_used
                )
            } {
                0 => {
                    // TODO: This function should error here because we expected this
                    // to work. For now, we do nothing...
                },
                _ => {
                    let variant : EVT_VARIANT = unsafe {
                        std::ptr::read(
                            buffer.as_ptr() as *const _
                        ) 
                    };

                    return Some(
                        EvtVariant(variant)
                    );
                }
            }
        }
    }

    None
}


/// EVT_HANDLE EvtOpenChannelConfig(
///   EVT_HANDLE Session,
///   LPCWSTR    ChannelPath,
///   DWORD      Flags
/// );
fn evt_open_channel_config(channel_path: &String) -> Result<EvtHandle, WinThingError> {
    // Create the wide string buffer
    let mut channel_path_u16 : Vec<u16> = channel_path.encode_utf16().collect();

    // Append a null wchar
    channel_path_u16.resize(channel_path.len() + 1, 0);

    let result = unsafe {
        EvtOpenChannelConfig(
            null_mut(), 
            channel_path_u16.as_ptr(), 
            0
        )
    };
    if result.is_null() {
        let last_error = unsafe {
            GetLastError()
        };

        let message = format!(
            "EvtOpenChannelConfig('{}') failed with code {}", 
            channel_path, 
            last_error
        );

        Err(
            WinThingError::winapi_error(message)
        )
    } else {
        Ok(
            EvtHandle(result)
        )
    }
}


/// BOOL EvtNextChannelPath(
///   EVT_HANDLE ChannelEnum,
///   DWORD      ChannelPathBufferSize,
///   LPWSTR     ChannelPathBuffer,
///   PDWORD     ChannelPathBufferUsed
/// );
fn evt_next_channel_id(channel_enum_handle: EVT_HANDLE) -> Option<String> {
    let mut buffer_used: DWORD = 0;

    let result = unsafe {
        EvtNextChannelPath(
            channel_enum_handle,
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
            let mut buffer: Vec<u16> = vec![0; buffer_used as usize];

            match unsafe {
                EvtNextChannelPath(
                    channel_enum_handle,
                    buffer.len() as _,
                    buffer.as_mut_ptr() as _,
                    &mut buffer_used
                )
            } {
                0 => {
                    // TODO: This function should error here because we expected this
                    // to work. For now, we do nothing...
                },
                _ => {
                    let channel_string = OsString::from_wide(
                        &buffer[..(buffer.len()-1)]
                    ).to_string_lossy().to_string();

                    return Some(channel_string);
                }
            }
        }
    }

    None
}