use crate::Sighting;
use bluster::{
    gatt::{
        characteristic,
        characteristic::Characteristic,
        descriptor,
        descriptor::Descriptor,
        event::{Event, Response},
        service::Service,
    },
    Peripheral, SdpShortUuid,
};
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    prelude::*,
};
use serde_json::json;
use std::{
    collections::HashSet,
    sync::{atomic, Arc, Mutex},
    thread,
    time::Duration,
};
use uuid::Uuid;

pub struct Bluetooth {
    name: String,
    timeout: Duration,
    sender_characteristic: Sender<Event>,
    receiver_characteristic: Receiver<Event>,
    sender_descriptor: Sender<Event>,
    receiver_descriptor: Receiver<Event>,
    sightings: Arc<Mutex<Vec<Sighting>>>,
}

impl Default for Bluetooth {
    fn default() -> Self {
        Self::new(Arc::new(Mutex::new(Vec::new())))
    }
}

impl Bluetooth {
    pub fn new(sightings: Arc<Mutex<Vec<Sighting>>>) -> Self {
        let (sender_characteristic, receiver_characteristic) = channel(1);
        let (sender_descriptor, receiver_descriptor) = channel(1);

        Self {
            name: "ornithology-pi".to_string(),
            timeout: Duration::from_secs(60),
            sender_characteristic,
            receiver_characteristic,
            sender_descriptor,
            receiver_descriptor,
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

    pub async fn run(&mut self) {
        let peripheral = self.peripheral().await;

        peripheral
            .add_service(&Service::new(
                Uuid::from_sdp_short_uuid(0x1234 as u16),
                true,
                self.characteristics(),
            ))
            .unwrap();

        let characteristic_handler = async {
            let notifying = Arc::new(atomic::AtomicBool::new(false));
            while let Some(event) = self.receiver_characteristic.next().await {
                match event {
                    Event::ReadRequest(read_request) => {
                        println!(
                            "GATT server got a read request with offset {}!",
                            read_request.offset
                        );
                        let value = {
                            let count = self.sightings.lock().unwrap().len();
                            format!("{}", count)
                        };
                        read_request
                            .response
                            .send(Response::Success(value.clone().into()))
                            .unwrap();
                        println!("GATT server responded with \"{}\"", value);
                    }
                    Event::WriteRequest(_) => {}
                    Event::NotifySubscribe(notify_subscribe) => {
                        println!("GATT server got a notify subscription!");
                        let notifying = Arc::clone(&notifying);
                        notifying.store(true, atomic::Ordering::Relaxed);
                        thread::spawn(move || {
                            let mut count = 0;
                            loop {
                                if !(&notifying).load(atomic::Ordering::Relaxed) {
                                    break;
                                };
                                count += 1;
                                println!("GATT server notifying \"hi {}\"!", count);
                                notify_subscribe
                                    .clone()
                                    .notification
                                    .try_send(format!("hi {}", count).into())
                                    .unwrap();
                                thread::sleep(Duration::from_secs(2));
                            }
                        });
                    }
                    Event::NotifyUnsubscribe => {
                        println!("GATT server got a notify unsubscribe!");
                        notifying.store(false, atomic::Ordering::Relaxed);
                    }
                };
            }
        };

        let descriptor_handler = async {
            while let Some(event) = self.receiver_descriptor.next().await {
                match event {
                    Event::ReadRequest(read_request) => {
                        println!(
                            "GATT server got a read request with offset {}!",
                            read_request.offset
                        );
                        let value = {
                            let count = self.sightings.lock().unwrap();
                            json!(count.clone()).to_string()
                        };
                        read_request
                            .response
                            .send(Response::Success(value.clone().into()))
                            .unwrap();
                        println!("GATT server responded with \"{}\"", value);
                    }
                    Event::WriteRequest(_) => {}
                    _ => panic!("Event not supported for Descriptors!"),
                };
            }
        };

        let name = self.name.clone();
        let timeout = self.timeout.clone();
        let main_fut = async move {
            while !peripheral.is_powered().await.unwrap() {}
            println!("Peripheral powered on");
            peripheral.register_gatt().await.unwrap();
            peripheral
                .start_advertising(&name.clone(), &[])
                .await
                .unwrap();
            println!("Peripheral started advertising");
            let ad_check = async { while !peripheral.is_advertising().await.unwrap() {} };
            let timeout = tokio::time::sleep(timeout.clone());
            futures::join!(ad_check, timeout);
            peripheral.stop_advertising().await.unwrap();
            while peripheral.is_advertising().await.unwrap() {}
            println!("Peripheral stopped advertising");
        };
        futures::join!(characteristic_handler, descriptor_handler, main_fut);
    }
}
