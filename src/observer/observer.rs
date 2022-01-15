use std::any::Any;

use crate::DataSighting;


pub trait ObserverToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> ObserverToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Observer: Send + Sync + ObserverToAny {
    fn notify(&self, sighting: DataSighting);
}
