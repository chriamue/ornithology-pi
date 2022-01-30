use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub camera: Camera,
}

pub fn load_config() -> Config {
    Figment::new()
        .merge(Toml::file(Env::var_or("ROCKET_CONFIG", "Rocket.toml")).nested())
        .merge(Env::prefixed("ROCKET_").ignore(&["PROFILE"]).global())
        .extract()
        .unwrap_or_default()
}
