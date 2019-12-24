use crate::winevt::EvtHandle;
use crate::errors::WinThingError;
use crate::winevt::callback::CallbackContext;
use crate::winevt::wevtapi::register_event_callback;


pub struct ChannelSubscription {
    _channel: String,
    _subscription_handle: EvtHandle
}

impl ChannelSubscription {
    pub fn new(
        channel: String, 
        query: Option<String>, 
        flags: Option<u32>, 
        context: &CallbackContext
    ) -> Result<Self, WinThingError> {
        let handle = register_event_callback(
            &channel,
            query,
            flags,
            context
        )?;

        Ok(ChannelSubscription {
            _channel: channel,
            _subscription_handle: handle
        })
    }
}