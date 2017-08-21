use errors::{Error, ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use registers;
use std::ops::{Deref, DerefMut};


/// The I2C address of the magnetometer.
const I2C_ADDRESS: u16 = 0x3C >> 1;


/// Interface to an LSM303 digital magnetometer.
pub struct Magnetometer<Dev>
    where Dev: I2CDevice
{
    device: Dev,
}


impl Magnetometer<LinuxI2CDevice> {
    /// Initialize the magnetometer for a Linux I2C device.
    pub fn new<Path>(path: Path) -> Result<Magnetometer<LinuxI2CDevice>>
        where Path: AsRef<::std::path::Path>
    {
        let device = LinuxI2CDevice::new(&path, I2C_ADDRESS)
            .chain_err(|| ErrorKind::FailedToOpenDevice)?;

        Magnetometer::from_i2c_device(device)
    }
}


impl<Dev> Magnetometer<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>
{
    /// Initialize the magnetometer, given an open I2C device.
    ///
    /// The opening of the device is platform specific,
    /// but the initialization is not.
    fn from_i2c_device(mut device: Dev) -> Result<Magnetometer<Dev>> {
        let mr_reg_m: registers::MrRegM = registers::MODE_CONTINUOUS;
        device.smbus_write_byte_data(registers::MR_REG_M, mr_reg_m.bits())?;

        let mut cra_reg_m: registers::CraRegM = registers::OUT_RATE_15_0;
        cra_reg_m.set(registers::TEMP_EN, true);
        device.smbus_write_byte_data(registers::CRA_REG_M, cra_reg_m.bits())?;

        let magnetometer = Magnetometer { device };
        Ok(magnetometer)
    }


    /// Read the magnetometer
    ///
    /// Returns a tuple of (x, y, z).
    /// WIP: the units are unclear.
    pub fn read_magnetic_field(&mut self) -> Result<(i16, i16, i16)> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.device
            .smbus_read_i2c_block_data(registers::OUT_X_H_M, 6)?;
        if data.len() < 6 {
            bail!(ErrorKind::NotEnoughData);
        }

        let mut cursor = Cursor::new(&data);

        // Yes indeed, the registers are ordered as X, Z, Y
        let x = cursor.read_i16::<BigEndian>()?;
        let z = cursor.read_i16::<BigEndian>()?;
        let y = cursor.read_i16::<BigEndian>()?;

        let out = (x, y, z);
        Ok(out)
    }


    /// Set the gain of the magnetometer.
    pub fn set_gain(&mut self, gain: registers::MagGain) -> Result<()>
        where Dev::Error: Send + 'static
    {
        use registers::Register;
        let mut register: registers::CrbRegM = self.device.get()?;
        register.set_gain(gain);
        self.device.set(register)
    }


    /// Read the thermometer.
    pub fn read_temperature(&mut self) -> Result<i16> {

        // unimplemented!("Not yet ready");

        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.device
            .smbus_read_i2c_block_data(registers::TEMP_OUT_H_M, 2)?;

        let mut cursor = Cursor::new(&data);

        let temp = cursor.read_i16::<BigEndian>()? / 16;

        Ok(temp)
    }
}


/// Access the underlying `I2CDevice`.
///
/// Most of the methods require a mutable reference; `DerefMut` is implemented as well.
impl<Dev> Deref for Magnetometer<Dev>
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
impl<Dev> DerefMut for Magnetometer<Dev>
    where Dev: I2CDevice
{
    fn deref_mut(&mut self) -> &mut Dev {
        &mut self.device
    }
}
