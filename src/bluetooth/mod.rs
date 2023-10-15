use std::sync::{Arc, Mutex};

use crate::Sighting;

pub mod handle_message;
pub use handle_message::handle_message;

pub mod message;
pub use message::Message;

pub mod gatt_srv;
pub mod rfcomm_srv;

pub const MANUFACTURER_ID: u16 = 0xf00d;
pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHANNEL: u8 = 7;
pub const MTU: u16 = 8192;

pub async fn setup_session(session: &bluer::Session) -> bluer::Result<()> {
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;

    log::info!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
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
