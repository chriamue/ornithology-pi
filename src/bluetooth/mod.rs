use std::sync::{Arc, Mutex};

use crate::Sighting;

mod message;
pub use message::Message;

pub mod gatt_srv;
pub mod l2cap_srv;
pub mod rfcomm_srv;

pub const MANUFACTURER_ID: u16 = 0xf00d;

pub async fn run_bluetooth(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    //let gatt_thread = tokio::spawn(gatt_srv::run(sightings.clone()));
    //let l2cap_thread = tokio::spawn(l2cap_srv::run(sightings.clone()));
    let rfcomm_thread = tokio::spawn(rfcomm_srv::run(sightings.clone()));
    /*gatt_thread
        .await
        .unwrap()
        .expect("The thread being joined has panicked");
    */
    /*
    l2cap_thread
    .await
    .unwrap()
    .expect("The thread being joined has panicked");
    */
    rfcomm_thread
        .await
        .unwrap()
        .expect("The thread being joined has panicked");
    Ok(())
}
