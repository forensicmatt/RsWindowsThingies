use rswinthings::winevt::callback::CallbackContext;
use rswinthings::winevt::subscription::ChannelSubscription;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create context
    let context = CallbackContext::new();

    // Create subscription
    let _subscription_security = ChannelSubscription::new(
        &None,
        "Security".to_owned(),
        Some("*".to_owned()),
        None,
        &context,
    );

    // Create subscription
    let _subscription_power = ChannelSubscription::new(
        &None,
        "Windows PowerShell".to_owned(),
        Some("*".to_owned()),
        None,
        &context,
    );

    loop {
        sleep(Duration::from_millis(200));
    }
}
