use serde_json::Value;
use serde_json::to_value;
use mft::entry::MftEntry;
use mft::attribute::{MftAttribute, MftAttributeType};
use crate::errors::WinThingError;
use crate::volume::liventfs::WindowsLiveNtfs;
use crate::file::helper::{
    get_entry_from_path,
    get_volume_path_from_path
};


fn get_attr_name(attribute: &MftAttributeType) -> String {
    match attribute {
        MftAttributeType::StandardInformation => "StandardInformation".to_string(),
        MftAttributeType::AttributeList => "AttributeList".to_string(),
        MftAttributeType::FileName => "FileName".to_string(),
        MftAttributeType::ObjectId => "ObjectId".to_string(),
        MftAttributeType::SecurityDescriptor => "SecurityDescriptor".to_string(),
        MftAttributeType::VolumeName => "VolumeName".to_string(),
        MftAttributeType::VolumeInformation => "VolumeInformation".to_string(),
        MftAttributeType::DATA => "DATA".to_string(),
        MftAttributeType::IndexRoot => "IndexRoot".to_string(),
        MftAttributeType::IndexAllocation => "IndexAllocation".to_string(),
        MftAttributeType::BITMAP => "BITMAP".to_string(),
        MftAttributeType::ReparsePoint => "ReparsePoint".to_string()
    }
}


/// Generate a custom JSON view of the mft entry
pub fn custom_entry_value(entry: MftEntry) -> Result<Value, WinThingError> {
    let mut entry_value = json!({});
    
    entry_value["header"] = to_value(&entry.header)?;
    entry_value["attributes"] = json!({});

    let attributes: Vec<MftAttribute> = entry.iter_attributes().filter_map(Result::ok).collect();
    for attribute in attributes {
        let attr_type_str = get_attr_name(&attribute.header.type_code);
        let instance = attribute.header.instance.to_string();

        entry_value["attributes"][&attr_type_str] = json!({
            instance: to_value(&attribute.to_owned())?
        });
    }
    
    Ok(entry_value)
}


pub struct EntryListener {
    live_volume: WindowsLiveNtfs,
    path_to_monitor: String,
    entry_to_monitor: i64
}
impl EntryListener {
    pub fn new(path_to_monitor: &str) -> Result<Self, WinThingError> {
        let entry = get_entry_from_path(
            path_to_monitor
        )?;

        let volume = get_volume_path_from_path(
            path_to_monitor
        )?;

        let live_volume = WindowsLiveNtfs::from_volume_path(
            &volume
        )?;

        Ok(
            Self {
                live_volume: live_volume,
                path_to_monitor: path_to_monitor.to_string(),
                entry_to_monitor: entry as i64
            }
        )
    }

    pub fn get_current_value(&mut self) -> Result<Value, WinThingError> {
        let mft_entry = self.live_volume.get_entry(
            self.entry_to_monitor
        )?;

        custom_entry_value(mft_entry)
    }
}