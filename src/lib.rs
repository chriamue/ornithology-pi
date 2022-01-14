pub mod capture;
pub use capture::Capture;
pub mod crop;
pub use crop::Crop;

pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
