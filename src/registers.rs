// This module contains type and constant definitions for registers.
// It is derived directly from the datasheet,
// which should serve as its best documentation.
#![allow(missing_docs)]

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
macro_rules! register {
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


/// A macro to declare a bunch of u8 constants
macro_rules! register_addresses {
    ( $($address:expr => $name:ident;)* ) => {
        $( pub const $name: u8 = $address; )*
    }
}


// Declare all the register addresses.
// This is based on Table 17 of the LSM303DLHC datasheet.
register_addresses! {
    // Accelerometer

    // 0x00 - 0x1F => reserved
    0x20 => CTRL_REG1_A;
    0x21 => CTRL_REG2_A;
    0x22 => CTRL_REG3_A;
    0x23 => CTRL_REG4_A;
    0x24 => CTRL_REG5_A;
    0x25 => CTRL_REG6_A;
    0x26 => REFERENCE_A;
    0x27 => STATUS_REG_A;
    0x28 => OUT_X_L_A;
    0x29 => OUT_X_H_H;
    0x2A => OUT_Y_L_A;
    0x2B => OUT_Y_H_A;
    0x2C => OUT_Z_L_A;
    0x2D => OUT_Z_H_A;
    0x2E => FIFO_CTRL_REG_A;
    0x2F => FIFO_SRC_REG_A;
    0x30 => INT1_CFG_A;
    0x31 => INT1_SOURCE_A;
    0x32 => INT1_THS_A;
    0x33 => INT1_DURATION_A;
    0x34 => INT2_CFG_A;
    0x35 => INT2_SOURCE_A;
    0x36 => INT2_THS_A;
    0x37 => INT2_DURATION_A;
    0x38 => CLICK_CFG_A;
    0x39 => CLICK_SRC_A;
    0x3A => CLICK_THS_A;
    0x3B => TIME_LIMIT_A;
    0x3C => TIME_LATENCY_A;
    0x3D => TIME_WINDOW_A;
    // 0x3E - 0x3F => reserved

    // Magnetometer

    0x00 => CRA_REG_M;
    0x01 => CRB_REG_M;
    0x02 => MR_REG_M;
    0x03 => OUT_X_H_M;
    0x04 => OUT_X_L_M;
    0x05 => OUT_Z_H_M;
    0x06 => OUT_Z_L_M;
    0x07 => OUT_Y_H_M;
    0x08 => OUT_Y_L_M;
    0x09 => SR_REG_M;
    0x0A => IRA_REG_M;
    0x0B => IRB_REG_M;
    0x0C => IRC_REG_M;
    // 0x0D - 0x30 => reserved
    0x31 => TEMP_OUT_H_M;
    0x32 => TEMP_OUT_L_M;
    // 0x33 - 0x3A => reserved
}


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
