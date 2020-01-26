use serde_json::Value;
use std::sync::mpsc::Receiver;
use crate::errors::WinThingError;
use crate::volume::liventfs;
use crate::usn::listener::{
    UsnListenerConfig,
    UsnVolumeListener
};


pub struct WindowsHandler {}

impl WindowsHandler {
    pub fn new() -> Self {
        Self{}
    }
    
    /// Listen to a file's MFT changes. Get the reciever.
    pub fn listen_mft(&self, _file_path: &str) -> Result<Receiver<Value>, WinThingError> {
        Err(
            WinThingError::unhandled(
                "listen_usn unimplemented.".to_string()
            )
        )
    }

    /// Listen to a volume's USN changes. Get the reciever.
    pub fn listen_usn(
        &self, 
        volume: &str, 
        config: Option<UsnListenerConfig>
    ) -> Result<Receiver<Value>, WinThingError> {
        let config = match config {
            Some(c) => c,
            None => UsnListenerConfig::default()
        };
        
        let listener = config.get_listener(volume);

        listener.listen_to_volume()
    }

    /// Listen to Windows events
    pub fn listen_events(&self) -> Result<Receiver<Value>, WinThingError> {
        Err(
            WinThingError::unhandled(
                "listen_events unimplemented.".to_string()
            )
        )
    }
}