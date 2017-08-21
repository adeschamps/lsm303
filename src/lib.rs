#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

mod accelerometer;
pub mod constants;
mod errors;
mod magnetometer;
pub mod registers;

pub use accelerometer::Accelerometer;
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use magnetometer::Magnetometer;
