use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "ornithology-pi", about = "Ornithology Pi Application.")]
pub struct Cli {
    /// Activate server mode
    #[structopt(short, long)]
    pub server: Option<bool>,

    /// Activate hotspot mode
    #[structopt(short, long)]
    pub hotspot: Option<bool>,

    /// Activate bluetooth mode
    #[structopt(short, long)]
    pub bluetooth: Option<bool>,

    /// Activate detect mode
    #[structopt(short, long)]
    pub detect: Option<bool>,

    /// Set the camera width
    #[structopt(long)]
    pub width: Option<u32>,

    /// Set the camera height
    #[structopt(long)]
    pub height: Option<u32>,

    /// Set the camera fps
    #[structopt(long)]
    pub fps: Option<u32>,

    /// Set the server port
    #[structopt(long)]
    pub port: Option<u16>,

    /// Set the server address
    #[structopt(long)]
    pub address: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        let mut cli = Cli::from_args();

        #[cfg(feature = "server")]
        if cli.server.is_none() {
            // If the server feature is enabled, default to server mode.
            cli.server = Some(true);
        }

        #[cfg(feature = "hotspot")]
        if cli.hotspot.is_none() {
            // If the hotspot feature is enabled, default to hotspot mode.
            cli.hotspot = Some(true);
        }

        #[cfg(feature = "bluetooth")]
        if cli.bluetooth.is_none() {
            // If the bluetooth feature is enabled, default to bluetooth mode.
            cli.bluetooth = Some(true);
        }

        #[cfg(feature = "detect")]
        if cli.detect.is_none() {
            // If the detect feature is enabled, default to detect mode.
            cli.detect = Some(true);
        }

        cli
    }

    pub fn evaluate(&self) {
        #[cfg(not(feature = "server"))]
        if self.server.unwrap_or(false) {
            eprintln!("Error: The server feature is not enabled and can not be used.");
        }

        #[cfg(not(feature = "hotspot"))]
        if self.hotspot.unwrap_or(false) {
            eprintln!("Error: The hotspot feature is not enabled and can not be used.");
        }

        #[cfg(not(feature = "bluetooth"))]
        if self.bluetooth.unwrap_or(false) {
            eprintln!("Error: The bluetooth feature is not enabled and can not be used.");
        }

        #[cfg(not(feature = "detect"))]
        if self.detect.unwrap_or(false) {
            eprintln!("Error: The detect feature is not enabled and can not be used.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cli = Cli::new();
        #[cfg(feature = "server")]
        assert_eq!(cli.server, Some(true));

        #[cfg(feature = "hotspot")]
        assert_eq!(cli.hotspot, Some(true));

        #[cfg(feature = "bluetooth")]
        assert_eq!(cli.bluetooth, Some(true));

        #[cfg(feature = "detect")]
        assert_eq!(cli.detect, Some(true));
    }
}
