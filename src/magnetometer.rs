//! Interface to the magnetometer.

use common::Vector3;
use dimensioned::{si, ucum};
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
    gain: Gain,
}


/// The output type of the magnetometer.
pub type MagneticField = Vector3<si::Tesla<f64>>;


/// The allowed settings for the gain on the magnetometer.
#[allow(non_camel_case_types)]
pub enum Gain {
    /// +/- 1.3 Gauss
    Gain_1_3,
    /// +/- 1.9 Gauss
    Gain_1_9,
    /// +/- 2.5 Gauss
    Gain_2_5,
    /// +/- 4.0 Gauss
    Gain_4_0,
    /// +/- 4.7 Gauss
    Gain_4_7,
    /// +/- 5,6 Gauss
    Gain_5_6,
    /// +/- 8.1 Gauss
    Gain_8_1,
}


impl Magnetometer<LinuxI2CDevice> {
    /// Initialize the magnetometer for a Linux I2C device.
    pub fn new<Path>(path: Path) -> Result<Magnetometer<LinuxI2CDevice>>
        where Path: AsRef<::std::path::Path>
    {
        let device =
            LinuxI2CDevice::new(&path, I2C_ADDRESS).chain_err(|| ErrorKind::FailedToOpenDevice)?;

        Magnetometer::from_i2c_device(device)
    }
}


impl<Dev> Magnetometer<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>,
          Dev::Error: Send + 'static
{
    /// Initialize the magnetometer, given an open I2C device.
    ///
    /// The opening of the device is platform specific,
    /// but initialization of the sensor is not.
    /// Prefer to use `Accelerometer::new`, unless you are using an
    /// implementation of `I2CDevice` that is not covered by this crate.
    pub fn from_i2c_device(mut device: Dev) -> Result<Magnetometer<Dev>> {
        use registers as r;

        // Set magnetometer to continuous mode
        let mr_reg_m = r::MrRegM::empty();
        write_register!(device, r::MR_REG_M, mr_reg_m)?;

        // enable temperature; set output rate to 15 Hz
        let cra_reg_m = r::TEMP_EN | r::DO2;
        write_register!(device, r::CRA_REG_M, cra_reg_m)?;

        let gain = Gain::Gain_1_3;

        let mut magnetometer = Magnetometer { device, gain };
        magnetometer.set_gain(Gain::Gain_1_3)?;

        Ok(magnetometer)
    }


    /// Read the magnetometer, returning the magnetic field as a vector.
    ///
    /// ```no_run
    /// # use lsm303::Magnetometer;
    /// # fn main() { test().unwrap(); }
    /// # fn test() -> lsm303::Result<()> {
    /// let mut sensor = Magnetometer::new("/dev/i2c-1")?;
    /// let field = sensor.read_magnetic_field()?;
    /// println!("Magnetic field: ({}, {}, {})",
    ///     field.x, field.y, field.z);
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_magnetic_field(&mut self) -> Result<MagneticField> {
        use byteorder::{ByteOrder, BigEndian};

        let data = self.device
            .smbus_read_i2c_block_data(registers::OUT_X_H_M, 6)?;
        if data.len() < 6 {
            bail!(ErrorKind::NotEnoughData);
        }

        // Refer to Table 3 or Table 75 of the datasheet.
        let (scale_xy, scale_z) = match self.gain {
            Gain::Gain_1_3 => (1100., 980.),
            Gain::Gain_1_9 => (855.0, 760.),
            Gain::Gain_2_5 => (670., 600.),
            Gain::Gain_4_0 => (450., 400.),
            Gain::Gain_4_7 => (400., 355.),
            Gain::Gain_5_6 => (330., 295.),
            Gain::Gain_8_1 => (230., 205.),
        };
        let scale_xy: si::Tesla<f64> = (ucum::GS / scale_xy).into();
        let scale_z: si::Tesla<f64> = (ucum::GS / scale_z).into();

        // Yes indeed, the registers are ordered as X, Z, Y
        let x = BigEndian::read_i16(&data[0..2]) as f64 * scale_xy;
        let z = BigEndian::read_i16(&data[2..4]) as f64 * scale_z;
        let y = BigEndian::read_i16(&data[4..6]) as f64 * scale_xy;

        let out = MagneticField { x, y, z };
        Ok(out)
    }


    /// Set the gain of the magnetometer.
    pub fn set_gain(&mut self, gain: Gain) -> Result<()>
        where Dev::Error: Send + 'static
    {
        use registers::{self as r, CRB_REG_M, CrbRegM};
        let mut flags = read_register!(self.device, CRB_REG_M, CrbRegM)?;

        flags.remove(r::GN2 | r::GN1 | r::GN0);
        let setting = match gain {
            Gain::Gain_1_3 => /* --  |  ---- */ r::GN0,
            Gain::Gain_1_9 => /* -- */ r::GN1,
            Gain::Gain_2_5 => /* -- */ r::GN1 | r::GN0,
            Gain::Gain_4_0 => r::GN2,
            Gain::Gain_4_7 => r::GN2 | /* -- */ r::GN0,
            Gain::Gain_5_6 => r::GN2 | r::GN1,
            Gain::Gain_8_1 => r::GN2 | r::GN1 | r::GN0,
        };
        flags.insert(setting);

        write_register!(self.device, CRB_REG_M, flags)?;
        self.gain = gain;

        Ok(())
    }


    // It is unclear how to interpret the TEMP_OUT registers.
    // The datasheet does not have quite enough information.
    // Discussions can be found in various places, such as
    // https://forum.pololu.com/t/16-bit-values-in-lsm303/8499/8
    // Until this is figured out, this function is being left out.
    #[cfg(none)]
    /// Read the thermometer.
    pub fn read_temperature(&mut self) -> Result<i16> {

        let data = self.device
            .smbus_read_i2c_block_data(registers::TEMP_OUT_H_M, 2)?;
        if data.len() < 2 {
            bail!(ErrorKind::NotEnoughData);
        }

        let temp = (data[0] as i16) << 4 | data[1] as i16 >> 4;
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


/// Access the underlying `I2CDevice`.
///
/// Refer to the LSM303 datasheet if you plan on accessing the device directly.
impl<Dev> DerefMut for Magnetometer<Dev>
    where Dev: I2CDevice
{
    fn deref_mut(&mut self) -> &mut Dev {
        &mut self.device
    }
}
