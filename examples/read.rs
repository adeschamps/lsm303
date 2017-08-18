#![allow(unused_doc_comment)]

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

extern crate lsm303;

error_chain!{}

quick_main!(run);

fn run() -> Result<()> {
    let device = "/dev/i2c-1";

    let accel_device = i2cdev::linux::LinuxI2CDevice::new(device, lsm303::constants::ADDRESS_ACCEL)
        .chain_err(|| "Failed to open I2C device")?;

    let mut lsm303 =
        lsm303::LSM303::new(accel_device).chain_err(|| "Failed to open LSM303 sensor")?;

    loop {
        let (x, y, z) = lsm303.read_accel()
            .chain_err(|| "Failed to read the accelerometer")?;
        println!("Accel: {}, {}, {}", x, y, z);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
