use chrono::Utc;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

pub type DataSighting<'a> = (Sighting, &'a DynamicImage);

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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

    pub fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();

        file.write_all(format!("{}\n", serde_json::to_string(&self).unwrap()).as_bytes())
            .expect("Unable to write file");
        Ok(())
    }
}

pub fn load_from_file(filename: &str) -> Result<Vec<Sighting>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sightings = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let sighting: Sighting = serde_json::from_str(&line).unwrap();
        sightings.push(sighting);
    }
    Ok(sightings)
}

pub fn save_to_file(sightings: Vec<Sighting>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .unwrap();

    for sighting in sightings {
        let line = format!("{}\n", serde_json::to_string(&sighting).unwrap());
        file.write_all(line.as_bytes())
            .expect("Unable to write file");
        file.flush().unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn default() {
        if Path::new("test.db").exists() {
            std::fs::remove_file("test.db").unwrap();
        }

        let sighting = Sighting::default();

        sighting.save("test.db").unwrap();

        let sightings = load_from_file("test.db").unwrap();
        assert_eq!(sightings.len(), 1);

        let sighting = Sighting::default();
        sighting.save("test.db").unwrap();
        let sightings = load_from_file("test.db").unwrap();
        assert_eq!(sightings.len(), 2);
    }

    #[test]
    fn save_to_file_test() {
        if Path::new("test.db").exists() {
            std::fs::remove_file("test.db").unwrap();
        }

        let sighting = Sighting::default();

        sighting.save("test.db").unwrap();

        let mut sightings = load_from_file("test.db").unwrap();
        assert_eq!(sightings.len(), 1);

        let sighting = Sighting::default();
        sightings.push(sighting);

        save_to_file(sightings, "test.db").unwrap();
        let sightings = load_from_file("test.db").unwrap();
        assert_eq!(sightings.len(), 2);
    }
}
