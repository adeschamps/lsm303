//! Interface to the accelerometer.

use errors::{Error, ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use registers;
use std::ops::{Deref, DerefMut};


/// The I2C address of the accelerometer.
const I2C_ADDRESS: u16 = 0x32 >> 1;


/// Interface to an LSM303 digital accelerometer.
pub struct Accelerometer<Dev>
    where Dev: I2CDevice
{
    device: Dev,
    scale: Scale,
    rate:Rate,
}


/// Settings for the scale of the acceleration measurement.
pub enum Scale {
    /// +/- 2G
    Scale2G,
    /// +/- 4G
    Scale4G,
    /// +/- 8G
    Scale8G,
    /// +/- 16G
    Scale16G,
}


/// Available values for the data rate.
///
/// See Table 20 of the LSM303 datasheet.
/// The default rate is `10 Hz`.
///
/// While most rates are valid for both normal
/// and low-power mode, note that the available
/// options differ at higher rates.
///
/// ```no_run
/// # use lsm303::accelerometer::{Accelerometer, Rate};
/// # fn main() { test().unwrap(); }
/// # fn test() -> lsm303::Result<()> {
/// let mut accel = Accelerometer::new("/dev/i2c-1")?;
/// accel.set_rate(Rate::Rate100Hz)?;
/// # Ok(())
/// # }
/// ```
pub enum Rate {
    /// Power down mode
    PowerDown,
    /// Normal / low-power mode (1 Hz)
    Rate1Hz,
    /// Normal / low-power mode (10 Hz)
    Rate10Hz,
    /// Normal / low-power mode (25 Hz)
    Rate25Hz,
    /// Normal / low-power mode (50 Hz)
    Rate50Hz,
    /// Normal / low-power mode (100 Hz)
    Rate100Hz,
    /// Normal / low-power mode (200 Hz)
    Rate200Hz,
    /// Normal / low-power mode (400 Hz)
    Rate400Hz,
    /// Low-power mode (1.620 KHz)
    Rate1620Hz,
    /// Normal (1.344 KHz)
    Rate1344Hz,
    /// Low-power mode (5.376 KHz)
    Rate5376Hz,
}


impl Accelerometer<LinuxI2CDevice> {
    /// Initialize the accelerometer for a Linux I2C device.
    pub fn new<Path>(path: Path) -> Result<Accelerometer<LinuxI2CDevice>>
        where Path: AsRef<::std::path::Path>
    {
        let device =
            LinuxI2CDevice::new(&path, I2C_ADDRESS).chain_err(|| ErrorKind::FailedToOpenDevice)?;

        Accelerometer::from_i2c_device(device)
    }
}


impl<Dev> Accelerometer<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>,
          Dev::Error: Send + 'static
{
    /// Initialize the accelerometer, given an open I2C device.
    ///
    /// The opening of the device is platform specific,
    /// but initialization of the sensor is not.
    /// Prefer to use `Accelerometer::new`, unless you are using an
    /// implementation of `I2CDevice` that is not covered by this crate.
    pub fn from_i2c_device(mut device: Dev) -> Result<Accelerometer<Dev>> {
        use registers::{self as r, CTRL_REG1_A, CTRL_REG4_A, CtrlReg4A};

        // Set data rate to 10 Hz, enable all axes.
        let ctrl_reg1_a = r::ODR1 | r::Zen | r::Yen | r::Xen;
        write_register!(device, CTRL_REG1_A, ctrl_reg1_a)?;

        // Enable high resolution output mode.
        let mut ctrl_reg4_a = read_register!(device, CTRL_REG4_A, CtrlReg4A)?;
        ctrl_reg4_a.insert(r::HR);
        write_register!(device, CTRL_REG4_A, ctrl_reg4_a)?;

        // Default scale is +/- 2G
        let scale = Scale::Scale2G;

        // Default rate
        let rate = Rate::Rate10Hz;

        let accelerometer = Accelerometer { device, scale, rate };
        Ok(accelerometer)
    }

    /// Read the accelerometer.
    ///
    /// Returns a tuple of (x, y, z) acceleration measured in milli-g's.
    pub fn read_acceleration(&mut self) -> Result<(i16, i16, i16)> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.device
            .smbus_read_i2c_block_data(registers::OUT_X_L_A | 0x80, 6)?;

        if data.len() < 6 {
            bail!(ErrorKind::NotEnoughData);
        }

        let mut cursor = Cursor::new(&data);

        let x = cursor.read_i16::<LittleEndian>()? >> 4;
        let y = cursor.read_i16::<LittleEndian>()? >> 4;
        let z = cursor.read_i16::<LittleEndian>()? >> 4;

        let scale = match self.scale {
            Scale::Scale2G => 1,
            Scale::Scale4G => 2,
            Scale::Scale8G => 4,
            Scale::Scale16G => 12, // This one doesn't follow the pattern - is the datasheet correct?
        };

        let out = (x * scale, y * scale, z * scale);
        Ok(out)
    }

    /// Set the scale of the acceleration measurement.
    pub fn set_scale(&mut self, scale: Scale) -> Result<()> {
        use registers::{CTRL_REG4_A, CtrlReg4A, FS1, FS0};

        let mut flags = read_register!(self.device, CTRL_REG4_A, CtrlReg4A)?;
        flags.remove(FS1 | FS0);
        let setting = match scale {
            Scale::Scale2G => CtrlReg4A::empty(),
            Scale::Scale4G => FS0,
            Scale::Scale8G => FS1,
            Scale::Scale16G => FS1 | FS0,
        };
        flags.insert(setting);

        write_register!(self.device, CTRL_REG4_A, flags)?;
        self.scale = scale;

        Ok(())
    }

    /// Set the rate at which acceleration is measured.
    ///
    /// ```no_run
    /// # use lsm303::accelerometer::{Accelerometer, Rate};
    /// # fn main() { test().unwrap(); }
    /// # fn test() -> lsm303::Result<()> {
    /// let mut accel = Accelerometer::new("/dev/i2c-1")?;
    /// accel.set_rate(Rate::Rate100Hz)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_rate(&mut self, rate: Rate) -> Result<()> {
        use registers::{self as r, CTRL_REG1_A, CtrlReg1A};

        let mut flags = read_register!(self.device, CTRL_REG1_A, CtrlReg1A)?;
        flags.remove(r::ODR3 | r::ODR2 | r::ODR1 | r::ODR0);

        let setting = match rate {
            _ => CtrlReg1A::empty()
        };
        flags.insert(setting);

        write_register!(self.device, CTRL_REG1_A, flags)?;
        self.rate = rate;

        Ok(())
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


/// Access the underlying `I2CDevice`.
///
/// Refer to the LSM303 datasheet if you plan on accessing the device directly.
impl<Dev> DerefMut for Accelerometer<Dev>
    where Dev: I2CDevice
{
    fn deref_mut(&mut self) -> &mut Dev {
        &mut self.device
    }
}
