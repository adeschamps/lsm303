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

    let mag_device = i2cdev::linux::LinuxI2CDevice::new(device, lsm303::constants::ADDRESS_MAG)
        .chain_err(|| "Failed to open I2C device")?;

    let mut lsm303 =
        lsm303::LSM303::new(accel_device, mag_device).chain_err(|| "Failed to open LSM303 sensor")?;

    loop {
        let (a_x, a_y, a_z) = lsm303.read_accel()
            .chain_err(|| "Failed to read the accelerometer")?;
        let (m_x, m_y, m_z) = lsm303.read_magnetometer()
            .chain_err(|| "Failed to read the magnetometer")?;
        println!("Accel: {}, {}, {}  ||  Mag: {}, {}, {}",
                 a_x,
                 a_y,
                 a_z,
                 m_x,
                 m_y,
                 m_z);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
