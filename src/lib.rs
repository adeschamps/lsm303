#![deny(missing_docs)]

//! Interface to the LSM303 digital accelerometer and magnetometer.
//!
//! - [Datasheet](http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf)
//! - [Application notes](http://www.st.com/content/ccc/resource/technical/document/application_note/e6/f0/fa/af/94/5e/43/de/CD00269797.pdf/files/CD00269797.pdf/jcr:content/translations/en.CD00269797.pdf)

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
