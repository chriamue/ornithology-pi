use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Shutdown {
    pub ctrlc: bool,
    pub signals: Vec<String>,
}

impl Default for Shutdown {
    fn default() -> Self {
        Shutdown {
            ctrlc: false,
            signals: vec!["term".into(), "hup".into()],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            width: 640,
            height: 480,
            fps: 30,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub port: u16,
    pub address: String,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8000,
            address: "127.0.0.1".into(),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub shutdown: Shutdown,
    pub camera: Camera,
    pub server: Server,
}

pub fn load_config() -> Config {
    Figment::new()
        .merge(Toml::file(Env::var_or("APP_CONFIG", "config.toml")).nested())
        .merge(Env::prefixed("APP_").ignore(&["PROFILE"]).global())
        .extract()
        .unwrap_or_default()
}
