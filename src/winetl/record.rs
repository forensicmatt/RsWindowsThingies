use serde_json::Value;
use winapi::um::eventtrace::{
    PEVENT_RECORD,
    PTRACE_EVENT_INFO,
    EVENT_HEADER,
    EVENT_DESCRIPTOR
};
use crate::utils::string::{
    read_wstring_from_pointer,
    read_wstring_array,
    guid_to_string
};


enum InfoProperty {
    ProviderName,
    LevelName,
    ChannelName,
    KeywordsName,
    TaskName,
    OpcodeName,
    EventMessage,
    ProviderMessage
}


fn header_to_value(
    header: EVENT_HEADER
) -> Value {
    let event_descriptor = json!({
        "id": header.EventDescriptor.Id,
        "version": header.EventDescriptor.Version,
        "channel": header.EventDescriptor.Channel,
        "level": header.EventDescriptor.Level,
        "opcode": header.EventDescriptor.Opcode,
        "task": header.EventDescriptor.Task,
        "keyword": header.EventDescriptor.Keyword
    });

    event_descriptor
}


/// Helper
pub struct EventRecord {
    event_record: PEVENT_RECORD,
    event_info: PTRACE_EVENT_INFO
}
impl EventRecord {
    pub fn new(
        event_record: PEVENT_RECORD,
        event_info: PTRACE_EVENT_INFO
    ) -> Self {
        Self {
            event_record,
            event_info
        }
    }

    unsafe fn get_info_value(
        &self, 
        prop: InfoProperty
    ) -> Option<Value> {
        match prop {
            InfoProperty::ProviderName => {
                if (*self.event_info).ProviderNameOffset == 0 {
                    return None;
                }
                debug!("ProviderNameOffset: {}", (*self.event_info).ProviderNameOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).ProviderNameOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::LevelName => {
                if (*self.event_info).LevelNameOffset == 0 {
                    return None;
                }
                debug!("LevelNameOffset: {}", (*self.event_info).LevelNameOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).LevelNameOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::ChannelName => {
                if (*self.event_info).ChannelNameOffset == 0 {
                    return None;
                }
                debug!("ChannelNameOffset: {}", (*self.event_info).ChannelNameOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).ChannelNameOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::KeywordsName => {
                if (*self.event_info).KeywordsNameOffset == 0 {
                    return None;
                }
                debug!("KeywordsNameOffset: {}", (*self.event_info).KeywordsNameOffset);

                if (*self.event_info).DecodingSource != 0 {
                    return None;
                }
        
                let value = read_wstring_array(
                    self.event_info as *const u8,
                    (*self.event_info).KeywordsNameOffset as isize
                );

                let value: Vec<String> = value
                    .iter()
                    .map(|v|v.to_string_lossy().to_string())
                    .collect();


                Some(json!(value))
            },
            InfoProperty::TaskName => {
                if (*self.event_info).TaskNameOffset == 0 {
                    return None;
                }
                debug!("TaskNameOffset: {}", (*self.event_info).TaskNameOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).TaskNameOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::OpcodeName => {
                if (*self.event_info).OpcodeNameOffset == 0 {
                    return None;
                }
                debug!("OpcodeNameOffset: {}", (*self.event_info).OpcodeNameOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).OpcodeNameOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::EventMessage => {
                if (*self.event_info).EventMessageOffset == 0 {
                    return None;
                }
                debug!("EventMessageOffset: {}", (*self.event_info).EventMessageOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).EventMessageOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            },
            InfoProperty::ProviderMessage => {
                if (*self.event_info).ProviderMessageOffset == 0 {
                    return None;
                }
                debug!("ProviderMessageOffset: {}", (*self.event_info).ProviderMessageOffset);
        
                let value = read_wstring_from_pointer(
                    self.event_info as *const u8,
                    (*self.event_info).ProviderMessageOffset as isize
                ).to_string_lossy().to_string();
        
                Some(json!(value))
            }
        }
    }

    pub unsafe fn get_header_value(&self) -> Value {
        header_to_value((*self.event_record).EventHeader)
    }

    pub unsafe fn get_value(&self) -> Value {
        let mut value = json!({});

        if let Some(provider) = self.get_info_value(InfoProperty::ProviderName) {
            value["provider"] = provider;
        }

        if let Some(level) = self.get_info_value(InfoProperty::LevelName) {
            value["level"] = level;
        }

        if let Some(channel) = self.get_info_value(InfoProperty::ChannelName) {
            value["channel"] = channel;
        }

        if let Some(keywords) = self.get_info_value(InfoProperty::KeywordsName) {
            value["keywords"] = keywords;
        }

        if let Some(task) = self.get_info_value(InfoProperty::TaskName) {
            value["task"] = task;
        }

        if let Some(opcode_name) = self.get_info_value(InfoProperty::OpcodeName) {
            value["opcode_name"] = opcode_name;
        }

        if let Some(event_message) = self.get_info_value(InfoProperty::EventMessage) {
            value["event_message"] = event_message;
        }

        if let Some(provider_message) = self.get_info_value(InfoProperty::ProviderMessage) {
            value["provider_message"] = provider_message;
        }

        value["binary_xml_offset"] = json!((*self.event_info).BinaryXMLOffset);
        value["decode_source"] = json!((*self.event_info).DecodingSource);

        value
    }
}
