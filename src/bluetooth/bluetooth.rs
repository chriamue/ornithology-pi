use crate::Sighting;
use bluster::{
    gatt::{
        characteristic, characteristic::Characteristic, descriptor, descriptor::Descriptor,
        event::Event, service::Service,
    },
    Peripheral, SdpShortUuid,
};
use futures::channel::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::{collections::HashSet, time::Duration};
use uuid::Uuid;

pub struct Bluetooth {
    name: String,
    timeout: Duration,
    sender_characteristic: Sender<Event>,
    sender_descriptor: Sender<Event>,
    sightings: Arc<Mutex<Vec<Sighting>>>,
}

impl Default for Bluetooth {
    fn default() -> Self {
        Self::new(Arc::new(Mutex::new(Vec::new())))
    }
}

impl Bluetooth {
    pub fn new(sightings: Arc<Mutex<Vec<Sighting>>>) -> Self {
        let (sender_characteristic, _) = channel(1);
        let (sender_descriptor, _) = channel(1);

        Self {
            name: "ornithology-pi".to_string(),
            timeout: Duration::from_secs(60),
            sender_characteristic,
            sender_descriptor,
            sightings,
        }
    }

    pub async fn peripheral(&self) -> Peripheral {
        let peripheral = Peripheral::new().await.unwrap();
        peripheral
            .add_service(&Service::new(
                Uuid::from_sdp_short_uuid(0x1234 as u16),
                true,
                self.characteristics(),
            ))
            .unwrap();
        peripheral
    }

    pub fn characteristics(&self) -> HashSet<Characteristic> {
        let mut characteristics: HashSet<Characteristic> = HashSet::new();
        characteristics.insert(Characteristic::new(
            Uuid::from_sdp_short_uuid(0x2A3D as u16),
            characteristic::Properties::new(
                Some(characteristic::Read(characteristic::Secure::Insecure(
                    self.sender_characteristic.clone(),
                ))),
                Some(characteristic::Write::WithResponse(
                    characteristic::Secure::Insecure(self.sender_characteristic.clone()),
                )),
                Some(self.sender_characteristic.clone()),
                None,
            ),
            None,
            {
                let mut descriptors = HashSet::<Descriptor>::new();
                descriptors.insert(Descriptor::new(
                    Uuid::from_sdp_short_uuid(0x2A3D as u16),
                    descriptor::Properties::new(
                        Some(descriptor::Read(descriptor::Secure::Insecure(
                            self.sender_descriptor.clone(),
                        ))),
                        Some(descriptor::Write(descriptor::Secure::Insecure(
                            self.sender_descriptor.clone(),
                        ))),
                    ),
                    None,
                ));
                descriptors
            },
        ));
        characteristics
    }

    pub async fn run(&self) {
        let peripheral = self.peripheral().await;
        let main_fut = async move {
            while !peripheral.is_powered().await.unwrap() {}
            println!("Peripheral powered on");
            peripheral.register_gatt().await.unwrap();
            peripheral.start_advertising(&self.name, &[]).await.unwrap();
            println!("Peripheral started advertising");
            let ad_check = async { while !peripheral.is_advertising().await.unwrap() {} };
            let timeout = tokio::time::sleep(self.timeout);
            futures::join!(ad_check, timeout);
            peripheral.stop_advertising().await.unwrap();
            while peripheral.is_advertising().await.unwrap() {}
            println!("Peripheral stopped advertising");
        };
        main_fut.await
    }
}
