#![deny(missing_docs)]

//! Interface to the LSM303 digital accelerometer and magnetometer.
//!
//! - [Datasheet](http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf)
//! - [Application notes](http://www.st.com/content/ccc/resource/technical/document/application_note/e6/f0/fa/af/94/5e/43/de/CD00269797.pdf/files/CD00269797.pdf/jcr:content/translations/en.CD00269797.pdf)
//!
//! ```no_run
//! # use std::time::Duration;
//! # fn main() { test().unwrap(); }
//! # fn test() -> lsm303::Result<()> {
//! let device = "/dev/i2c-1";
//! let mut accelerometer =
//!     lsm303::Accelerometer::new(device)?;
//! let mut magnetometer =
//!     lsm303::Magnetometer::new(device)?;
//!
//!  loop {
//!     let accel = accelerometer.read_acceleration()?;
//!     let mag = magnetometer.read_magnetic_field()?;
//!     println!("Accel: ({}, {}, {})  ||  Mag: ({}, {}, {})",
//!              accel.x, accel.y, accel.z,
//!              mag.x, mag.y, mag.z);
//!     std::thread::sleep(Duration::from_millis(100));
//! }
//! # Ok(())
//! # }
//! ```

// External crates

#[macro_use]
extern crate bitflags;

extern crate byteorder;

extern crate dimensioned;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

// Exports

pub mod common;

mod errors;
pub use errors::{Error, ErrorKind, Result, ResultExt};

#[macro_use]
pub mod registers;

pub mod accelerometer;
pub use accelerometer::Accelerometer;

pub mod magnetometer;
pub use magnetometer::Magnetometer;
