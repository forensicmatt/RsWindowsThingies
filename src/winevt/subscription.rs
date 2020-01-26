use crate::errors::WinThingError;
use crate::winevt::callback::CallbackContext;
use crate::winevt::wevtapi::register_event_callback;
use crate::winevt::EvtHandle;

pub struct ChannelSubscription {
    _channel: String,
    _subscription_handle: EvtHandle,
}

impl ChannelSubscription {
    pub fn new(
        session: &Option<EvtHandle>,
        channel: String,
        query: Option<String>,
        flags: Option<u32>,
        context: &mut CallbackContext,
    ) -> Result<Self, WinThingError> {
        let handle = register_event_callback(&session, &channel, query, flags, context)?;

        Ok(ChannelSubscription {
            _channel: channel,
            _subscription_handle: handle,
        })
    }
}
