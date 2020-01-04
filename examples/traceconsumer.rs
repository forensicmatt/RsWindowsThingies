use std::thread::sleep;
use std::time::Duration;
use rswinthings::winetl::consumer::TraceConsumer;


fn main() {
    // Create context
    let context = TraceConsumer::new("NT Kernel Logger".to_string()).unwrap();

    loop {
        println!("Sleeping...");
        sleep(Duration::from_millis(200));
    }
}