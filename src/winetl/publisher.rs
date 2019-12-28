use serde::Serialize;
use serde_json::Value;
use winapi::um::winevt::*;
use crate::winevt::EvtHandle;
use crate::errors::WinThingError;
use crate::winevt::variant::VariantValue;
use crate::winevt::wevtapi::evt_get_publisher_metadata_property;
use crate::winevt::wevtapi::evt_open_publisher_metadata;
use crate::winevt::wevtapi::evt_open_publisher_enum;
use crate::winevt::wevtapi::evt_next_publisher_id;
use crate::winevt::wevtapi::evt_get_object_array_size;
use crate::winevt::wevtapi::evt_get_object_array_property;
use crate::winevt::wevtapi::evt_format_message;


const PUBLISHER_META_REFERENCES: [(&str, u32); 5] = [
    ("EvtPublisherMetadataChannelReferencePath", EvtPublisherMetadataChannelReferencePath),
    ("EvtPublisherMetadataChannelReferenceIndex", EvtPublisherMetadataChannelReferenceIndex),
    ("EvtPublisherMetadataChannelReferenceID", EvtPublisherMetadataChannelReferenceID),
    ("EvtPublisherMetadataChannelReferenceFlags", EvtPublisherMetadataChannelReferenceFlags),
    ("EvtPublisherMetadataChannelReferenceMessageID", EvtPublisherMetadataChannelReferenceMessageID)
];

const PUBLISHER_META_TASKS: [(&str, u32); 4] = [
    ("EvtPublisherMetadataTaskName", EvtPublisherMetadataTaskName),
    ("EvtPublisherMetadataTaskEventGuid", EvtPublisherMetadataTaskEventGuid),
    ("EvtPublisherMetadataTaskValue", EvtPublisherMetadataTaskValue),
    ("EvtPublisherMetadataTaskMessageID", EvtPublisherMetadataTaskMessageID)
];

const PUBLISHER_META_LEVELS: [(&str, u32); 3] = [
    ("EvtPublisherMetadataLevelName", EvtPublisherMetadataLevelName),
    ("EvtPublisherMetadataLevelValue", EvtPublisherMetadataLevelValue),
    ("EvtPublisherMetadataLevelMessageID", EvtPublisherMetadataLevelMessageID)
];

const PUBLISHER_META_OPCODES: [(&str, u32); 3] = [
    ("EvtPublisherMetadataOpcodeName", EvtPublisherMetadataOpcodeName),
    ("EvtPublisherMetadataOpcodeValue", EvtPublisherMetadataOpcodeValue),
    ("EvtPublisherMetadataOpcodeMessageID", EvtPublisherMetadataOpcodeMessageID)
];

const PUBLISHER_META_KEYWORDS: [(&str, u32); 3] = [
    ("EvtPublisherMetadataKeywordName", EvtPublisherMetadataKeywordName),
    ("EvtPublisherMetadataKeywordValue", EvtPublisherMetadataKeywordValue),
    ("EvtPublisherMetadataKeywordMessageID", EvtPublisherMetadataKeywordMessageID)
];

const PUBLISHER_METADATA: [(&str, u32); 12] = [
    ("EvtPublisherMetadataPublisherGuid", EvtPublisherMetadataPublisherGuid),
    ("EvtPublisherMetadataResourceFilePath", EvtPublisherMetadataResourceFilePath),
    ("EvtPublisherMetadataParameterFilePath", EvtPublisherMetadataParameterFilePath),
    ("EvtPublisherMetadataMessageFilePath", EvtPublisherMetadataMessageFilePath),
    ("EvtPublisherMetadataHelpLink", EvtPublisherMetadataHelpLink),
    ("EvtPublisherMetadataPublisherMessageID", EvtPublisherMetadataPublisherMessageID),
    ("EvtPublisherMetadataChannelReferences", EvtPublisherMetadataChannelReferences),
    ("EvtPublisherMetadataLevels", EvtPublisherMetadataLevels),
    ("EvtPublisherMetadataTasks", EvtPublisherMetadataTasks),
    ("EvtPublisherMetadataOpcodes", EvtPublisherMetadataOpcodes),
    ("EvtPublisherMetadataKeywords", EvtPublisherMetadataKeywords),
    ("EvtPublisherMetadataPropertyIdEND", EvtPublisherMetadataPropertyIdEND)
];


#[derive(Serialize, Debug)]
pub struct MetadataChannels(Vec<MetadataChannel>);

impl MetadataChannels {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata_channels: Vec<MetadataChannel> = Vec::new();

        let meta_prop_variant = evt_get_publisher_metadata_property(
            &metadata_handle,
            EvtPublisherMetadataChannelReferences
        )?;

        // Get the Array Object handle
        let array_handle = EvtHandle(
            unsafe { *meta_prop_variant.0.u.EvtHandleVal() }
        );
        let array_size = evt_get_object_array_size(
            &array_handle
        )?;

        for i in 0..array_size {
            let meta_channel = MetadataChannel::new(
                &metadata_handle,
                &array_handle,
                i
            )?;

            metadata_channels.push(
                meta_channel
            );
        }

        Ok(
            MetadataChannels(
                metadata_channels
            )
        )
    }
}


#[derive(Serialize, Debug)]
pub struct MetadataChannel {
    path: VariantValue,
    index: VariantValue,
    id: VariantValue,
    flags: VariantValue,
    message: Option<String>
}

impl MetadataChannel {
    pub fn new(
        metadata_handle: &EvtHandle, 
        array_handle: &EvtHandle, 
        i: u32
    ) -> Result<Self, WinThingError> {
        let path = evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataChannelReferencePath
        )?.get_variant_value()?;

        let index = evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataChannelReferenceIndex
        )?.get_variant_value()?;

        let id = evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataChannelReferenceID
        )?.get_variant_value()?;

        let flags = evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataChannelReferenceFlags
        )?.get_variant_value()?;

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
                let m = evt_format_message(
                    Some(&metadata_handle),
                    None,
                    id
                )?;

                Some(m)
            }
        };

        Ok( Self {
            path,
            index,
            id,
            flags,
            message
        })
    }
}


#[derive(Serialize, Debug)]
pub struct MetadataLevels(Vec<MetadataLevel>);

impl MetadataLevels {
    pub fn new(metadata_handle: &EvtHandle) -> Result<Self, WinThingError> {
        let mut metadata_levels: Vec<MetadataLevel> = Vec::new();

        let meta_prop_variant = evt_get_publisher_metadata_property(
            &metadata_handle,
            EvtPublisherMetadataLevels
        )?;

        // Get the Array Object handle
        let array_handle = EvtHandle(
            unsafe { *meta_prop_variant.0.u.EvtHandleVal() }
        );
        let array_size = evt_get_object_array_size(
            &array_handle
        )?;

        for i in 0..array_size {
            let meta_level = MetadataLevel::new(
                &metadata_handle,
                &array_handle,
                i
            )?;

            metadata_levels.push(
                meta_level
            );
        }

        Ok(
            MetadataLevels(
                metadata_levels
            )
        )
    }
}


#[derive(Serialize, Debug)]
pub struct MetadataLevel {
    name: VariantValue,
    value: VariantValue,
    message: String
}

impl MetadataLevel {
    pub fn new(
        metadata_handle: &EvtHandle, 
        array_handle: &EvtHandle, 
        i: u32
    ) -> Result<Self, WinThingError> {
        let name = evt_get_object_array_property(
            &array_handle,
            i,
            EvtPublisherMetadataLevelName
        )?.get_variant_value()?;

        let value = evt_get_object_array_property(
            &array_handle, i, EvtPublisherMetadataLevelValue
        )?.get_variant_value()?;

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

        let message = evt_format_message(
            Some(&metadata_handle),
            None,
            message_id
        )?;

        Ok( Self {
            name,
            value,
            message
        })
    }
}


#[derive(Debug)]
pub struct PublisherMeta {
    pub name: String,
    handle: EvtHandle
}

impl PublisherMeta {
    pub fn new(name: String) -> Result<Self, WinThingError> {
        let handle = evt_open_publisher_metadata(
            &None,
            Some(name.clone()),
            None
        )?;

        Ok(
            Self {
                name: name,
                handle: handle
            }
        )
    }

    pub fn get_metadata_levels(&self) -> Result<MetadataLevels, WinThingError> {
        MetadataLevels::new(
            &self.handle
        )
    }

    pub fn get_metadata_channels(&self) -> Result<MetadataChannels, WinThingError> {
        MetadataChannels::new(
            &self.handle
        )
    }

    pub fn to_json_value(&self) -> Result<Value, WinThingError> {
        let mut mapping = json!({});

        for (key, id) in &PUBLISHER_METADATA {
            match id {
                &EvtPublisherMetadataPropertyIdEND => {
                    break;
                },
                &EvtPublisherMetadataPublisherMessageID => {
                },
                &EvtPublisherMetadataChannelReferences => {
                    let meta_channels = self.get_metadata_channels()?;
                    mapping["EvtPublisherMetadataChannelReferences"] = serde_json::to_value(
                        &meta_channels
                    )?;
                },
                &EvtPublisherMetadataLevels => {
                    let meta_levels = self.get_metadata_levels()?;
                    mapping["EvtPublisherMetadataLevels"] = serde_json::to_value(
                        &meta_levels
                    )?;
                },
                &EvtPublisherMetadataTasks => {
                },
                &EvtPublisherMetadataOpcodes => {
                },
                &EvtPublisherMetadataKeywords => {
                },
                &EvtPublisherMetadataKeywords => {
                },
                _ => {
                    let variant = match evt_get_publisher_metadata_property(
                        &self.handle, *id
                    ) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("Error getting metadata property {}: {:?}", key, e);
                            continue;
                        }
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
            }
        }

        Ok(mapping)
    }
}


#[derive(Debug)]
pub struct PublisherEnumerator {
    session: Option<EvtHandle>,
    pub_enum_handle: EvtHandle
}

impl PublisherEnumerator {
    pub fn new(session: Option<EvtHandle>) -> Result<Self, WinThingError> {
        let handle = evt_open_publisher_enum(
            &session
        )?;

        Ok(
            Self {
                session: session,
                pub_enum_handle: handle
            }
        )
    }
}

impl Iterator for PublisherEnumerator {
    type Item = PublisherMeta;

    fn next(&mut self) -> Option<PublisherMeta> {
        loop {
            match evt_next_publisher_id(
                &self.pub_enum_handle
            ){
                Ok(o) => {
                    match o {
                        Some(name) => {
                            let meta = match PublisherMeta::new(name.clone()) {
                                Ok(m) => m,
                                Err(e) => {
                                    error!("Error openting meta for {}: {:?}", name, e);
                                    continue;
                                }
                            };

                            return Some(meta)
                        },
                        None => break
                    };
                },
                Err(e) => {
                    error!("Error on evt_next_publisher_id for PublisherEnumerator: {:?}", e);
                    continue;
                }
            }
        }

        None
    }
}
