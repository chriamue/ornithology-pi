use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NoDevice;

impl fmt::Display for NoDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no capture device set")
    }
}

impl error::Error for NoDevice {}
