use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Camera {
    pub device: Option<String>,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            device: None,
            width: 640,
            height: 480,
            fps: 30,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
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

pub fn merge_cli_config(config: &Config, cli: &Cli) -> Config {
    let mut config: Config = config.clone();

    if let Some(width) = cli.width {
        config.camera.width = width;
    }

    if let Some(height) = cli.height {
        config.camera.height = height;
    }

    if let Some(fps) = cli.fps {
        config.camera.fps = fps;
    }

    if let Some(port) = cli.port {
        config.server.port = port;
    }

    if let Some(address) = &cli.address {
        config.server.address = address.clone();
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_cli_config() {
        let config = Config::default();
        let cli = Cli {
            width: Some(1920),
            height: Some(1080),
            fps: Some(15),
            port: Some(8080),
            address: Some("localhost".into()),
            ..Default::default()
        };

        let merged = merge_cli_config(&config, &cli);

        assert_eq!(merged.camera.device, None);
        assert_eq!(merged.camera.width, cli.width.unwrap());
        assert_eq!(merged.camera.height, cli.height.unwrap());
        assert_eq!(merged.camera.fps, cli.fps.unwrap());
        assert_eq!(merged.server.port, cli.port.unwrap());
        assert_eq!(merged.server.address, cli.address.unwrap());
    }
}
