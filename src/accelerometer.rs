use errors::{Error, ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use registers;
use std::ops::{Deref, DerefMut};


/// The I2C address of the accelerometer.
const I2C_ADDRESS: u16 = 0x32 >> 1;


pub struct Accelerometer<Dev>
    where Dev: I2CDevice
{
    device: Dev,
}


impl Accelerometer<LinuxI2CDevice> {
    /// Initialize the accelerometer for a Linux I2C device.
    pub fn new<Path>(path: Path) -> Result<Accelerometer<LinuxI2CDevice>>
        where Path: AsRef<::std::path::Path>
    {
        let device = LinuxI2CDevice::new(&path, I2C_ADDRESS)
            .chain_err(|| ErrorKind::FailedToOpenDevice)?;

        Accelerometer::from_i2c_device(device)
    }
}


impl<Dev> Accelerometer<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>
{
    /// Initialize the accelerometer, given an open I2C device.
    ///
    /// The opening of the device is platform specific,
    /// but the initialization is not.
    pub fn from_i2c_device(mut device: Dev) -> Result<Accelerometer<Dev>> {

        device.smbus_write_byte_data(registers::ACCEL_CTRL_REG1_A, 0x27)?;

        let mut reg4_a = {
            let bits = device.smbus_read_byte_data(registers::ACCEL_CTRL_REG4_A)?;
            registers::CtrlReg4A::from_bits_truncate(bits)
        };
        reg4_a.set(registers::HR, true);
        device.smbus_write_byte_data(registers::ACCEL_CTRL_REG4_A, reg4_a.bits())?;

        let accelerometer = Accelerometer { device };
        Ok(accelerometer)
    }

    /// Read the accelerometer.
    ///
    /// Returns a tuple of (x, y, z) acceleration in cm/s^2.
    pub fn read_acceleration(&mut self) -> Result<(i16, i16, i16)> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.device
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
}


/// Access the underlying `I2CDevice`.
///
/// Most of the methods require a mutable reference; `DerefMut` is implemented as well.
impl<Dev> Deref for Accelerometer<Dev>
    where Dev: I2CDevice
{
    type Target = Dev;

    fn deref(&self) -> &Dev {
        &self.device
    }
}


/// Access the underlying I2C device.
///
/// Refer to the LSM303 datasheet if you plan on accessing the device directly.
impl<Dev> DerefMut for Accelerometer<Dev>
    where Dev: I2CDevice
{
    fn deref_mut(&mut self) -> &mut Dev {
        &mut self.device
    }
}
