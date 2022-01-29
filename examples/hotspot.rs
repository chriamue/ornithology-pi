use ornithology_pi::hotspot::Hotspot;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let mut hotspot = Hotspot::default();
    hotspot.start();
    for i in 1..10 {
        println!("Terminating in {} seconds ...", 10 - i);
        thread::sleep(time::Duration::from_secs(1));
    }
    hotspot.stop();
}
