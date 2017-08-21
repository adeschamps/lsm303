//! A subset of the registers on the LSM303 - just the ones that we need.

use errors::{ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;

/// A trait for a device with strongly typed registers.
///
/// A device may implement this trait for each type of register it manipulates.
/// There is no need to manipulate it by hand; use the `register!` macro instead:
///
/// ```
/// # #[macro_use] extern crate bitflags;
/// # bitflags!{ struct CtrlReg4A: u8 { const BDU = 1 << 7; } }
/// # #[macro_use] extern crate lsm303;
/// # use lsm303::registers::Register;
/// # use lsm303::{ErrorKind, Result, ResultExt};
/// # extern crate i2cdev;
/// # use i2cdev::core::I2CDevice;
/// # fn main() {
/// register!(0x23, CtrlReg4A);
/// # }
/// ```
///
/// Once implemented, registers may be read, modified, and written like so:
///
/// ```ignore
/// let mut register: RegA = device.get()?;
/// register.set(FLAG);
/// device.set(register)?;
/// ```
pub trait Register<T> {
    /// Retrieve the value of a register.
    ///
    /// ```ignore
    /// let register: RegA = device.get()?;
    /// ```
    fn get(&mut self) -> Result<T>;

    /// Set the value of a register.
    ///
    /// ```ignore
    /// let register = RegA::empty();
    /// device.set(register)?;
    /// ```
    fn set(&mut self, value: T) -> Result<()>;
}

/// Implement the `Register` trait for a bitflag.
///
/// ```ignore
/// register!(0x00, RegA);
/// ```
#[macro_export] macro_rules! register {
    ($address:expr, $bitflag:ident) => {
        impl<Dev> Register<$bitflag> for Dev
            where Dev: I2CDevice,
                  Dev::Error: Send + 'static
        {
            fn get(&mut self) -> Result<$bitflag> {
                self.smbus_read_byte_data($address)
                    .chain_err(|| ErrorKind::FailedToReadRegister)
                    .map($bitflag::from_bits_truncate)
            }

            fn set(&mut self, value: $bitflag) -> Result<()> {
                self.smbus_write_byte_data($address, value.bits())
                    .chain_err(|| ErrorKind::FailedToWriteRegister)
            }
        }
    };
}

/// Implement multiple `Register` traits in one macro invocation.
///
/// ```
/// registers!(
///     0x00, RegA;
///     0x01, RegB;
/// );
/// ```
///
/// is equivalent to
///
/// ```
/// register!(0x00, RegA);
/// register!(0x01, RegB);
/// ```
macro_rules! registers {
    ( $($address:expr, $bitflag:ident;)* ) => {
        $(register!($address, $bitflag); )*
    }
}

registers!(
    0x23, CtrlReg4A;

    0x00, CraRegM;
    0x01, CrbRegM;
    0x02, MrRegM;
);

/*
register!(0x00, CraRegM);
register!(0x01, CrbRegM);
register!(0x02, MrRegM);
 */

pub const ACCEL_CTRL_REG1_A: u8 = 0x20;
pub const ACCEL_CTRL_REG4_A: u8 = 0x23;
pub const ACCEL_OUT_X_L_A: u8 = 0x28;

pub const CRA_REG_M: u8 = 0x00;
pub const CRB_REG_M: u8 = 0x01;
pub const MR_REG_M: u8 = 0x02;
pub const OUT_X_H_M: u8 = 0x03;
pub const TEMP_OUT_H_M: u8 = 0x31;

bitflags!{
    pub struct CtrlReg4A: u8 {
        const BDU = 1 << 7;
        const BLE = 1 << 6;
        const FS1 = 1 << 5;
        const FS0 = 1 << 4;
        const HR  = 1 << 3;
        // not used 1 << 2;
        // not used 1 << 1;
        const SIM = 1 << 0;

        const FS_2G = 0;
        const FS_4G = FS0.bits;
        const FS_8G = FS1.bits;
        const FS_16G = FS1.bits | FS0.bits;
    }
}


bitflags!{
    pub struct CraRegM: u8 {
        const TEMP_EN = 1 << 7;
        const DO2 = 1 << 4;
        const DO1 = 1 << 3;
        const DO0 = 1 << 2;

        const OUT_RATE_0_75 = 0;
        const OUT_RATE_1_5 = DO0.bits;
        const OUT_RATE_3_0 = DO1.bits;
        const OUT_RATE_7_5 = DO1.bits | DO0.bits;
        const OUT_RATE_15_0 = DO2.bits;
        const OUT_RATE_30_0 = DO2.bits | DO0.bits;
    }
}


bitflags! {
    pub struct CrbRegM: u8 {
        const GN2 = 1 << 7;
        const GN1 = 1 << 6;
        const GN0 = 1 << 5;
    }
}

/// The allowed settings for the gain on the magnetometer.
#[allow(non_camel_case_types)]
pub enum MagGain {
    Gain_1_3,
    Gain_1_9,
    Gain_2_5,
    Gain_4_0,
    Gain_4_7,
    Gain_5_6,
    Gain_8_1,
}

impl CrbRegM {
    pub fn set_gain(&mut self, gain: MagGain) {
        let mask = GN2 | GN1 | GN0;

        let value = match gain {
            MagGain::Gain_1_3 => GN0,
            MagGain::Gain_1_9 => GN1,
            MagGain::Gain_2_5 => GN2 | GN1,
            MagGain::Gain_4_0 => GN2,
            MagGain::Gain_4_7 => GN2 | GN0,
            MagGain::Gain_5_6 => GN2 | GN1,
            MagGain::Gain_8_1 => GN2 | GN1 | GN0,
        };

        self.remove(mask);
        self.insert(value);
    }
}


bitflags!{
    pub struct MrRegM: u8 {
        const MD1 = 1 << 1;
        const MD0 = 1 << 0;

        const MODE_CONTINUOUS = 0;
        const MODE_SINGLE_CONVERSION = MD0.bits;
        const SLEEP_MODE = MD1.bits;
    }
}
