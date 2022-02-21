use std::sync::{Arc, Mutex};

use crate::Sighting;

mod message;
pub use message::Message;

pub mod gatt_srv;
pub mod l2cap_srv;
pub mod rfcomm_srv;

pub const MANUFACTURER_ID: u16 = 0xf00d;

pub async fn run_bluetooth(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;
    adapter.set_pairable(false).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter_name,
        adapter.address().await?
    );
    let gatt_handle = gatt_srv::run_advertise(&adapter, sightings.clone())
        .await
        .unwrap();

    rfcomm_srv::run_session(&session, sightings.clone())
        .await
        .unwrap();

    drop(gatt_handle);

    Ok(())
}
