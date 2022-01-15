use crate::DataSighting;

pub trait Observer: Send + Sync {
    fn notify(&self, sighting: DataSighting);
}
