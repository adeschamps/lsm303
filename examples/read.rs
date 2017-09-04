#![allow(unused_doc_comment)]

extern crate dimensioned;
use dimensioned::si;
use dimensioned::f64prefixes::MILLI;

#[macro_use]
extern crate error_chain;

extern crate lsm303;

error_chain!{}

quick_main!(run);

fn run() -> Result<()> {
    let device = "/dev/i2c-1";

    let mut accelerometer =
        lsm303::Accelerometer::new(device).chain_err(|| "Failed to initialize the accelerometer")?;
    let mut magnetometer =
        lsm303::Magnetometer::new(device).chain_err(|| "Failed to initialize the magnetometer")?;

    loop {
        let accel = accelerometer.read_acceleration()
            .chain_err(|| "Failed to read the accelerometer")?;
        let mag = magnetometer.read_magnetic_field()
            .chain_err(|| "Failed to read the magnetometer")?;

        println!("Accel: ({:02.2}, {:02.2}, {:02.2}) m/s^2  ||  Mag: ({:02.2}, {:02.2}, {:02.2}) mT",
                 accel.x / si::MPS2,
                 accel.y / si::MPS2,
                 accel.z / si::MPS2,
                 mag.x / (MILLI * si::T),
                 mag.y / (MILLI * si::T),
                 mag.z / (MILLI * si::T));
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
