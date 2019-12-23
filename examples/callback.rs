use std::thread::sleep;
use std::time::Duration;
use rswinthings::winevt::wevtapi::register_event_callback;


fn main() {
    let channel = "Security".to_owned();

    register_event_callback(
        &channel,
        None
    );

    loop {
        sleep(Duration::from_millis(200));
    }
}