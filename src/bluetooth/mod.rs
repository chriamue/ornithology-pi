use std::sync::{Arc, Mutex};

use crate::Sighting;

mod message;
pub use message::Message;

pub mod gatt_srv;
pub mod rfcomm_srv;

pub const MANUFACTURER_ID: u16 = 0xf00d;
pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHANNEL: u8 = 7;
pub const MTU: u16 = 8192;

pub async fn setup_session(session: &bluer::Session) -> bluer::Result<()> {
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
    Ok(())
}

pub async fn run_bluetooth(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let mut session = bluer::Session::new().await?;
    setup_session(&session).await?;
    /*let gatt_handle = gatt_srv::run_advertise(&adapter, sightings.clone())
    .await
    .unwrap();
    */

    rfcomm_srv::run_session(&session, sightings.clone()).await?;

    Ok(())
}
