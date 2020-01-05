use serde_json::Value;
use serde_json::to_value;
use crate::errors::WinThingError;
use crate::volume::liventfs::WindowsLiveNtfs;
use crate::file::helper::{
    get_entry_from_path,
    get_volume_path_from_path
};


pub struct MftDifferencer {
    live_volume: WindowsLiveNtfs,
    path_to_monitor: String,
    entry_to_monitor: i64
}
impl MftDifferencer {
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
        let current_value = to_value(&mft_entry)?;

        Ok(current_value)
    }
}