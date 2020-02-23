use crate::errors::WinThingError;
use crate::winevt::variant::VariantValue;
use crate::winevt::wevtapi::evt_format_message;
use crate::winevt::wevtapi::evt_get_object_array_property;
use crate::winevt::wevtapi::evt_get_object_array_size;
use crate::winevt::wevtapi::evt_get_publisher_metadata_property;
use crate::winevt::wevtapi::evt_next_publisher_id;
use crate::winevt::wevtapi::evt_open_publisher_enum;
use crate::winevt::wevtapi::evt_open_publisher_metadata;
use crate::winevt::EvtHandle;
use serde::Serialize;
use serde_json::Value;
use std::path::Path;
use winapi::um::winevt::*;

#[allow(dead_code)]
const PUBLISHER_META_REFERENCES: [(&str, u32); 5] = [
    (
        "EvtPublisherMetadataChannelReferencePath",
        EvtPublisherMetadataChannelReferencePath,
    ),
    (
        "EvtPublisherMetadataChannelReferenceIndex",
        EvtPublisherMetadataChannelReferenceIndex,
    ),
    (
        "EvtPublisherMetadataChannelReferenceID",
        EvtPublisherMetadataChannelReferenceID,
    ),
    (
        "EvtPublisherMetadataChannelReferenceFlags",
        EvtPublisherMetadataChannelReferenceFlags,
    ),
    (
        "EvtPublisherMetadataChannelReferenceMessageID",
        EvtPublisherMetadataChannelReferenceMessageID,
    ),
];

#[allow(dead_code)]
const PUBLISHER_META_TASKS: [(&str, u32); 4] = [
    ("EvtPublisherMetadataTaskName", EvtPublisherMetadataTaskName),
    (
        "EvtPublisherMetadataTaskEventGuid",
        EvtPublisherMetadataTaskEventGuid,
    ),
    (
        "EvtPublisherMetadataTaskValue",
        EvtPublisherMetadataTaskValue,
    ),
    (
        "EvtPublisherMetadataTaskMessageID",
        EvtPublisherMetadataTaskMessageID,
    ),
];

#[allow(dead_code)]
const PUBLISHER_META_LEVELS: [(&str, u32); 3] = [
    (
        "EvtPublisherMetadataLevelName",
        EvtPublisherMetadataLevelName,
    ),
    (
        "EvtPublisherMetadataLevelValue",
        EvtPublisherMetadataLevelValue,
    ),
    (
        "EvtPublisherMetadataLevelMessageID",
        EvtPublisherMetadataLevelMessageID,
    ),
];

#[allow(dead_code)]
const PUBLISHER_META_OPCODES: [(&str, u32); 3] = [
    (
        "EvtPublisherMetadataOpcodeName",
        EvtPublisherMetadataOpcodeName,
    ),
    (
        "EvtPublisherMetadataOpcodeValue",
        EvtPublisherMetadataOpcodeValue,
    ),
    (
        "EvtPublisherMetadataOpcodeMessageID",
        EvtPublisherMetadataOpcodeMessageID,
    ),
];

#[allow(dead_code)]
const PUBLISHER_META_KEYWORDS: [(&str, u32); 3] = [
    (
        "EvtPublisherMetadataKeywordName",
        EvtPublisherMetadataKeywordName,
    ),
    (
        "EvtPublisherMetadataKeywordValue",
        EvtPublisherMetadataKeywordValue,
    ),
    (
        "EvtPublisherMetadataKeywordMessageID",
        EvtPublisherMetadataKeywordMessageID,
    ),
];

const PUBLISHER_METADATA: [(&str, u32); 12] = [
    (
        "EvtPublisherMetadataPublisherGuid",
        EvtPublisherMetadataPublisherGuid,
    ),
    (
        "EvtPublisherMetadataResourceFilePath",
        EvtPublisherMetadataResourceFilePath,
    ),
    (
        "EvtPublisherMetadataParameterFilePath",
        EvtPublisherMetadataParameterFilePath,
    ),
    (
        "EvtPublisherMetadataMessageFilePath",
        EvtPublisherMetadataMessageFilePath,
    ),
    ("EvtPublisherMetadataHelpLink", EvtPublisherMetadataHelpLink),
    (
        "EvtPublisherMetadataPublisherMessageID",
        EvtPublisherMetadataPublisherMessageID,
    ),
    (
        "EvtPublisherMetadataChannelReferences",
        EvtPublisherMetadataChannelReferences,
    ),
    ("EvtPublisherMetadataLevels", EvtPublisherMetadataLevels),
    ("EvtPublisherMetadataTasks", EvtPublisherMetadataTasks),
    ("EvtPublisherMetadataOpcodes", EvtPublisherMetadataOpcodes),
    ("EvtPublisherMetadataKeywords", EvtPublisherMetadataKeywords),
    (
        "EvtPublisherMetadataPropertyIdEND",
        EvtPublisherMetadataPropertyIdEND,
    ),
];

#[derive(Serialize, Debug)]
pub struct MetadataChannels(pub Vec<MetadataChannel>);

impl MetadataChannels {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata_channels: Vec<MetadataChannel> = Vec::new();

        let meta_prop_variant = evt_get_publisher_metadata_property(
            &metadata_handle,
            EvtPublisherMetadataChannelReferences,
        )?;

        // Get the Array Object handle
        let array_handle = EvtHandle(unsafe { *meta_prop_variant.0.u.EvtHandleVal() });
        let array_size = evt_get_object_array_size(&array_handle)?;

        for i in 0..array_size {
            let meta_channel = MetadataChannel::new(&metadata_handle, &array_handle, i)?;

            metadata_channels.push(meta_channel);
        }

        Ok(MetadataChannels(metadata_channels))
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataChannel {
    pub path: VariantValue,
    pub index: VariantValue,
    pub id: VariantValue,
    pub flags: VariantValue,
    pub message: Option<String>,
}

impl MetadataChannel {
    pub fn new(
        metadata_handle: &EvtHandle,
        array_handle: &EvtHandle,
        i: u32,
    ) -> Result<Self, WinThingError> {
        let path = evt_get_object_array_property(
            &array_handle,
            i,
            EvtPublisherMetadataChannelReferencePath,
        )?
        .get_variant_value()?;

        let index = evt_get_object_array_property(
            &array_handle,
            i,
            EvtPublisherMetadataChannelReferenceIndex,
        )?
        .get_variant_value()?;

        let id = evt_get_object_array_property(
            &array_handle,
            i,
            EvtPublisherMetadataChannelReferenceID,
        )?
        .get_variant_value()?;

        let flags = evt_get_object_array_property(
            &array_handle,
            i,
            EvtPublisherMetadataChannelReferenceFlags,
        )?
        .get_variant_value()?;

        let message_id: u32 = match evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataChannelReferenceMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataChannelReferenceMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = evt_format_message(Some(&metadata_handle), None, id)?;

                Some(m)
            }
        };

        Ok(Self {
            path,
            index,
            id,
            flags,
            message,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataTasks(pub Vec<MetadataTask>);

impl MetadataTasks {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata: Vec<MetadataTask> = Vec::new();

        let meta_prop_variant =
            evt_get_publisher_metadata_property(&metadata_handle, EvtPublisherMetadataTasks)?;

        // Get the Array Object handle
        let array_handle = EvtHandle(unsafe { *meta_prop_variant.0.u.EvtHandleVal() });
        if array_handle.is_null() {
            debug!("EvtPublisherMetadataTasks handle is null.");
            return Ok(Self(metadata));
        }

        let array_size = evt_get_object_array_size(&array_handle)?;

        for i in 0..array_size {
            let meta = MetadataTask::new(&metadata_handle, &array_handle, i)?;

            metadata.push(meta);
        }

        Ok(Self(metadata))
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataTask {
    pub name: VariantValue,
    pub guid: VariantValue,
    pub value: VariantValue,
    pub message: Option<String>,
}

impl MetadataTask {
    pub fn new(
        metadata_handle: &EvtHandle,
        array_handle: &EvtHandle,
        i: u32,
    ) -> Result<Self, WinThingError> {
        let name = evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataTaskName)?
            .get_variant_value()?;

        let guid =
            evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataTaskEventGuid)?
                .get_variant_value()?;

        let value = evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataTaskValue)?
            .get_variant_value()?;

        let message_id: u32 = match evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataTaskMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataTaskMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = evt_format_message(Some(&metadata_handle), None, id)?;

                Some(m)
            }
        };

        Ok(Self {
            name,
            guid,
            value,
            message,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataLevels(pub Vec<MetadataLevel>);

impl MetadataLevels {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata_levels: Vec<MetadataLevel> = Vec::new();

        let meta_prop_variant =
            evt_get_publisher_metadata_property(&metadata_handle, EvtPublisherMetadataLevels)?;

        // Get the Array Object handle
        let array_handle = EvtHandle(unsafe { *meta_prop_variant.0.u.EvtHandleVal() });
        if array_handle.is_null() {
            debug!("EvtPublisherMetadataLevels handle is null.");
            return Ok(MetadataLevels(metadata_levels));
        }

        let array_size = evt_get_object_array_size(&array_handle)?;

        for i in 0..array_size {
            let meta_level = MetadataLevel::new(&metadata_handle, &array_handle, i)?;

            metadata_levels.push(meta_level);
        }

        Ok(MetadataLevels(metadata_levels))
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataLevel {
    pub name: VariantValue,
    pub value: VariantValue,
    pub message: Option<String>,
}

impl MetadataLevel {
    pub fn new(
        metadata_handle: &EvtHandle,
        array_handle: &EvtHandle,
        i: u32,
    ) -> Result<Self, WinThingError> {
        let name = evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataLevelName)?
            .get_variant_value()
            .expect("Error EvtPublisherMetadataLevelName");

        let value =
            evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataLevelValue)?
                .get_variant_value()
                .expect("Error EvtPublisherMetadataLevelValue");

        let message_id: u32 = match evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataLevelMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataLevelMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = match evt_format_message(Some(&metadata_handle), None, id) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error Formatting Message: {:?}", e);
                        format!("<ERROR FORMATTING MESSAGE: {}>", e.message)
                    }
                };

                Some(m)
            }
        };

        Ok(Self {
            name: name,
            value: value,
            message: message,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataOpcodes(pub Vec<MetadataOpcode>);

impl MetadataOpcodes {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata: Vec<MetadataOpcode> = Vec::new();

        let meta_prop_variant =
            evt_get_publisher_metadata_property(&metadata_handle, EvtPublisherMetadataOpcodes)?;

        // Get the Array Object handle
        let array_handle = EvtHandle(unsafe { *meta_prop_variant.0.u.EvtHandleVal() });
        if array_handle.is_null() {
            debug!("EvtPublisherMetadataOpcodes handle is null.");
            return Ok(Self(metadata));
        }

        let array_size = evt_get_object_array_size(&array_handle)?;

        for i in 0..array_size {
            let meta = MetadataOpcode::new(&metadata_handle, &array_handle, i)?;

            metadata.push(meta);
        }

        Ok(Self(metadata))
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataOpcode {
    pub name: VariantValue,
    pub value: VariantValue,
    pub message: Option<String>,
}

impl MetadataOpcode {
    pub fn new(
        metadata_handle: &EvtHandle,
        array_handle: &EvtHandle,
        i: u32,
    ) -> Result<Self, WinThingError> {
        let name = evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataOpcodeName)?
            .get_variant_value()
            .expect("Error EvtPublisherMetadataOpcodeName");

        let value =
            evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataOpcodeValue)?
                .get_variant_value()
                .expect("Error EvtPublisherMetadataOpcodeValue");

        let message_id: u32 = match evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataOpcodeMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataOpcodeMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = match evt_format_message(Some(&metadata_handle), None, id) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error Formatting Message: {:?}", e);
                        format!("<ERROR FORMATTING MESSAGE: {}>", e.message)
                    }
                };

                Some(m)
            }
        };

        Ok(Self {
            name: name,
            value: value,
            message: message,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataKeywords(pub Vec<MetadataKeyword>);

impl MetadataKeywords {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata: Vec<MetadataKeyword> = Vec::new();

        let meta_prop_variant =
            evt_get_publisher_metadata_property(&metadata_handle, EvtPublisherMetadataKeywords)?;

        // Get the Array Object handle
        let array_handle = EvtHandle(unsafe { *meta_prop_variant.0.u.EvtHandleVal() });
        if array_handle.is_null() {
            debug!("EvtPublisherMetadataKeywords handle is null.");
            return Ok(Self(metadata));
        }

        let array_size = evt_get_object_array_size(&array_handle)?;

        for i in 0..array_size {
            let meta = MetadataKeyword::new(&metadata_handle, &array_handle, i)?;

            metadata.push(meta);
        }

        Ok(Self(metadata))
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataKeyword {
    pub name: VariantValue,
    pub value: VariantValue,
    pub message: Option<String>,
}

impl MetadataKeyword {
    pub fn new(
        metadata_handle: &EvtHandle,
        array_handle: &EvtHandle,
        i: u32,
    ) -> Result<Self, WinThingError> {
        let name =
            evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataKeywordName)?
                .get_variant_value()
                .expect("Error EvtPublisherMetadataKeywordName");

        let value =
            evt_get_object_array_property(&array_handle, i, EvtPublisherMetadataKeywordValue)?
                .get_variant_value()
                .expect("Error EvtPublisherMetadataKeywordValue");

        let message_id: u32 = match evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataKeywordMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataKeywordMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = match evt_format_message(Some(&metadata_handle), None, id) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error Formatting Message: {:?}", e);
                        format!("<ERROR FORMATTING MESSAGE: {}>", e.message)
                    }
                };

                Some(m)
            }
        };

        Ok(Self {
            name: name,
            value: value,
            message: message,
        })
    }
}

#[derive(Debug)]
pub struct PublisherMeta {
    pub name: String,
    handle: EvtHandle,
}

impl PublisherMeta {
    pub fn new(session: &Option<EvtHandle>, name: String) -> Result<Self, WinThingError> {
        let (publisher_id, logfile_path) = match Path::new(&name).is_file() {
            true => {
                debug!("{} is a file.", name);
                (None, Some(name.clone()))
            }
            false => (Some(name.clone()), None),
        };

        let handle = evt_open_publisher_metadata(&session, publisher_id, logfile_path)?;

        Ok(Self {
            name: name,
            handle: handle,
        })
    }

    pub fn get_property(
        &self,
        id: EVT_PUBLISHER_METADATA_PROPERTY_ID,
    ) -> Result<VariantValue, WinThingError> {
        let variant = evt_get_publisher_metadata_property(&self.handle, id)?;
        Ok(variant.get_variant_value()?)
    }

    pub fn get_property_string(&self, id: EVT_PUBLISHER_METADATA_PROPERTY_ID) -> Option<String> {
        let variant = match evt_get_publisher_metadata_property(&self.handle, id) {
            Ok(v) => v,
            Err(e) => {
                info!("[{}] Error getting property {}", self.name, e.message);
                return None;
            }
        };

        match variant.get_variant_value() {
            Ok(v) => Some(v.to_string()),
            Err(e) => {
                info!(
                    "[{}] Error getting value from varient: {}",
                    self.name, e.message
                );
                None
            }
        }
    }

    pub fn get_publisher_message(&self) -> Result<Option<String>, WinThingError> {
        let message_id: u32 = match evt_get_publisher_metadata_property(
            &self.handle, EvtPublisherMetadataPublisherMessageID
        )?.get_variant_value()? {
            VariantValue::UInt(i) => i as u32,
            _ => {
                return Err(
                    WinThingError::unhandled(
                        "Expected EvtPublisherMetadataPublisherMessageID property to contain a UInt VariantValue.".to_owned()
                    )
                )
            }
        };

        let message = match message_id {
            0xffffffff => None,
            id => {
                let m = match evt_format_message(Some(&self.handle), None, id) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Error Formatting Message: {:?}", e);
                        format!("<ERROR FORMATTING MESSAGE: {}>", e.message)
                    }
                };

                Some(m)
            }
        };

        Ok(message)
    }

    pub fn get_metadata_levels(&self) -> Result<MetadataLevels, WinThingError> {
        MetadataLevels::new(&self.handle)
    }

    pub fn get_metadata_tasks(&self) -> Result<MetadataTasks, WinThingError> {
        MetadataTasks::new(&self.handle)
    }

    pub fn get_metadata_channels(&self) -> Result<MetadataChannels, WinThingError> {
        MetadataChannels::new(&self.handle)
    }

    pub fn get_metadata_opcodes(&self) -> Result<MetadataOpcodes, WinThingError> {
        MetadataOpcodes::new(&self.handle)
    }

    pub fn get_metadata_keywords(&self) -> Result<MetadataKeywords, WinThingError> {
        MetadataKeywords::new(&self.handle)
    }

    pub fn to_json_value(&self) -> Result<Value, WinThingError> {
        let mut mapping = json!({
            "Name": self.name
        });

        for (key, id) in &PUBLISHER_METADATA {
            #[allow(non_upper_case_globals)]
            match id {
                &EvtPublisherMetadataPropertyIdEND => {
                    break;
                }
                &EvtPublisherMetadataPublisherMessageID => {
                    let message = match self.get_publisher_message() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataPublisherMessageID: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataPublisherMessageID"] =
                        serde_json::to_value(&message)?;
                }
                &EvtPublisherMetadataChannelReferences => {
                    let meta = match self.get_metadata_channels() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataChannelReferences: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataChannelReferences"] = serde_json::to_value(&meta)?;
                }
                &EvtPublisherMetadataLevels => {
                    let meta = match self.get_metadata_levels() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataLevels: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataLevels"] = serde_json::to_value(&meta)?;
                }
                &EvtPublisherMetadataTasks => {
                    let meta = match self.get_metadata_tasks() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataTasks: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataTasks"] = serde_json::to_value(&meta)?;
                }
                &EvtPublisherMetadataOpcodes => {
                    let meta = match self.get_metadata_opcodes() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataOpcodes: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataOpcodes"] = serde_json::to_value(&meta)?;
                }
                &EvtPublisherMetadataKeywords => {
                    let meta = match self.get_metadata_keywords() {
                        Ok(m) => m,
                        Err(e) => {
                            info!(
                                "[{}] Error retrieving EvtPublisherMetadataKeywords: {}",
                                self.name,
                                e.message.trim()
                            );
                            continue;
                        }
                    };
                    mapping["EvtPublisherMetadataKeywords"] = serde_json::to_value(&meta)?;
                }
                _ => {
                    let variant = match evt_get_publisher_metadata_property(&self.handle, *id) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("Error getting metadata property {}: {:?}", key, e);
                            continue;
                        }
                    };

                    match variant.get_json_value() {
                        Ok(v) => {
                            mapping[key] = serde_json::to_value(&v)?;
                        }
                        Err(e) => {
                            error!("Error getting variant value: {:?}", e);
                        }
                    }
                }
            }
        }

        Ok(mapping)
    }
}

#[derive(Debug)]
pub struct PublisherEnumerator {
    session: Option<EvtHandle>,
    pub_enum_handle: EvtHandle,
}

impl PublisherEnumerator {
    pub fn new(session: Option<EvtHandle>) -> Result<Self, WinThingError> {
        let handle = evt_open_publisher_enum(&session)?;

        Ok(Self {
            session: session,
            pub_enum_handle: handle,
        })
    }
}

impl Iterator for PublisherEnumerator {
    type Item = PublisherMeta;

    fn next(&mut self) -> Option<PublisherMeta> {
        loop {
            match evt_next_publisher_id(&self.pub_enum_handle) {
                Ok(o) => {
                    match o {
                        Some(name) => {
                            let meta = match PublisherMeta::new(&self.session, name.clone()) {
                                Ok(m) => m,
                                Err(e) => {
                                    error!("Error opening meta for {}: {:?}", name, e);
                                    continue;
                                }
                            };

                            return Some(meta);
                        }
                        None => break,
                    };
                }
                Err(e) => {
                    error!(
                        "Error on evt_next_publisher_id for PublisherEnumerator: {:?}",
                        e
                    );
                    continue;
                }
            }
        }

        None
    }
}
