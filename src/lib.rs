#[macro_use]
extern crate bitflags;

extern crate byteorder;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

pub mod constants;
mod errors;
pub mod registers;

pub use errors::{Error, ErrorKind, Result, ResultExt};

use i2cdev::core::I2CDevice;

/// An LSM303 accelerometer and magnetometer.
pub struct LSM303<Dev: I2CDevice>
    where Error: From<Dev::Error>
{
    accel_device: Dev,
    mag_device: Dev,
}

/// Implement the sensor for Linux I2C devices.
use i2cdev::linux::LinuxI2CDevice;
impl LSM303<LinuxI2CDevice> {
    /// Initialize the sensor.
    pub fn new<Path>(path: Path) -> Result<LSM303<LinuxI2CDevice>>
        where Path: AsRef<std::path::Path>
    {
        let mut accel_device = LinuxI2CDevice::new(&path, constants::ADDRESS_ACCEL)
            .chain_err(|| ErrorKind::FailedToOpenDevice)?;
        accel_device.smbus_write_byte_data(registers::ACCEL_CTRL_REG1_A, 0x27)?;

        // Initialize the accelerometer
        let mut reg4_a = {
            let bits = accel_device.smbus_read_byte_data(registers::ACCEL_CTRL_REG4_A)?;
            registers::CtrlReg4A::from_bits_truncate(bits)
        };
        reg4_a.set(registers::HR, true);
        accel_device.smbus_write_byte_data(registers::ACCEL_CTRL_REG4_A, reg4_a.bits())?;

        // Initialize the magnetometer
        let mut mag_device = LinuxI2CDevice::new(&path, constants::ADDRESS_MAG)
            .chain_err(|| ErrorKind::FailedToOpenDevice)?;
        let reg_m: registers::MrRegM = registers::MODE_CONTINUOUS;
        mag_device.smbus_write_byte_data(registers::MAG_MR_REG_M, reg_m.bits())?;

        let lsm303 = LSM303 {
            accel_device,
            mag_device,
        };
        Ok(lsm303)
    }
}

impl<Dev> LSM303<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>
{
    /// Read the accelerometer.
    /// Returns a tuple of (x, y, z) acceleration in cm/s^2.
    pub fn read_accel(&mut self) -> Result<(i16, i16, i16)> {

        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.accel_device
            .smbus_read_i2c_block_data(registers::ACCEL_OUT_X_L_A | 0x80, 6)?;

        if data.len() < 6 {
            bail!(ErrorKind::NotEnoughData);
        }

        let mut cursor = Cursor::new(&data);

        let x = cursor.read_i16::<LittleEndian>()? >> 4;
        let y = cursor.read_i16::<LittleEndian>()? >> 4;
        let z = cursor.read_i16::<LittleEndian>()? >> 4;

        let out = (x, y, z);
        Ok(out)
    }

    /// Read the magnetometer.
    /// Returns a tuple of (x, y, z).
    /// WIP: the units are unclear.
    pub fn read_magnetometer(&mut self) -> Result<(i16, i16, i16)> {

        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.mag_device
            .smbus_read_i2c_block_data(registers::MAG_OUT_X_H_M, 6)?;

        let mut cursor = Cursor::new(&data);

        // Yes indeed, the registers are ordered as X, Z, Y
        let x = cursor.read_i16::<BigEndian>()?;
        let z = cursor.read_i16::<BigEndian>()?;
        let y = cursor.read_i16::<BigEndian>()?;

        let out = (x, y, z);
        Ok(out)
    }
}
