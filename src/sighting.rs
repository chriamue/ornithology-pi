use chrono::Utc;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type DataSighting<'a> = (Sighting, &'a DynamicImage);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sighting {
    pub uuid: String,
    pub timestamp: i64,
    pub species: String,
}

impl Sighting {
    pub fn new(species: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            species,
        }
    }
}
