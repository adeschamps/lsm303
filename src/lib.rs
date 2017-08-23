#![deny(missing_docs)]

//! Interface to the LSM303 digital accelerometer and magnetometer.
//!
//! [Datasheet](http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf)

// External crates

#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

// Exports

mod errors;
pub use errors::{Error, ErrorKind, Result, ResultExt};

#[macro_use]
pub mod registers;

mod accelerometer;
pub use accelerometer::Accelerometer;

mod magnetometer;
pub use magnetometer::Magnetometer;
