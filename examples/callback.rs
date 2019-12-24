use std::thread::sleep;
use std::time::Duration;
use rswinthings::winevt::subscription::ChannelSubscription;
use rswinthings::winevt::callback::CallbackContext;


fn main() {
    // Create context
    let context = CallbackContext::new();

    // Create subscription
    let _subscription_security = ChannelSubscription::new(
        "Security".to_owned(),
        Some("*".to_owned()),
        None,
        &context
    );

    // Create subscription
    let _subscription_power = ChannelSubscription::new(
        "Windows PowerShell".to_owned(),
        Some("*".to_owned()),
        None,
        &context
    );

    loop {
        sleep(Duration::from_millis(200));
    }
}