use super::Observer;

pub trait Observable: Send {
    fn register(&mut self, observer: Box<dyn Observer>);
}
