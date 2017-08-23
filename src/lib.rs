#![deny(missing_docs)]

//! Interface to the LSM303 digital accelerometer and magnetometer.
//!
//! [Datasheet](http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf)

#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

mod accelerometer;
mod errors;
mod magnetometer;
pub mod registers;

pub use accelerometer::Accelerometer;
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use magnetometer::Magnetometer;
