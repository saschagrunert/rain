extern crate rain;
use rain::error::*;

use std::io;
use std::error::Error;

#[test]
fn success_convert_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let rain_error: RainError = io_error.into();
    println!("{} {:?} {}", rain_error, rain_error, rain_error.description());
    assert_eq!(rain_error.code, ErrorType::Other);
    assert_eq!(rain_error.description, "Not found".to_string());
}
