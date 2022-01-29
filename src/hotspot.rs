use pnet::datalink::{self, NetworkInterface};
use wifi_rs::{prelude::*, WiFi};

pub struct Hotspot {
    wifi: WiFi,
    interface: String,
}

impl Default for Hotspot {
    fn default() -> Self {
        let interface = datalink::interfaces()
            .into_iter()
            .filter(|iface: &NetworkInterface| iface.name.starts_with("w"))
            .last()
            .unwrap();

        let config = Some(Config {
            interface: Some(&interface.name),
        });
        Hotspot {
            wifi: WiFi::new(config),
            interface: interface.name,
        }
    }
}

impl Hotspot {
    pub fn start(&mut self) {
        let config = HotspotConfig::new(Some(HotspotBand::Bg), Some(Channel::Three));
        self.wifi
            .create_hotspot("ornithology-pi", "ornithology", Some(&config))
            .unwrap();
        println!("started hotspot on interface {}", self.interface)
    }

    pub fn stop(&mut self) {
        self.wifi.stop_hotspot().unwrap();
        println!("stopped hotspot on interface {}", self.interface)
    }
}
