// This module contains type and constant definitions for registers.
// It is derived directly from the datasheet,
// which should serve as its best documentation.
#![allow(missing_docs)]

//! A subset of the registers on the LSM303 - just the ones that we need.

/// Read a register and convert to a bitflag.
///
/// ```
/// let mut flags = read_register(self.device, CRA_REG_M, CraRegM)?;
/// ```
macro_rules! read_register {
    ( $device:expr, $register:expr, $flag_type:ident ) => {
        $device
            .smbus_read_byte_data($register)
            .chain_err(|| ErrorKind::FailedToReadRegister)
            .map($flag_type::from_bits_truncate)
    }
}


/// Write a bitflag to a register.
///
/// ```
/// write_register!(self.device, CRA_REG_M, flags)?;
/// ```
macro_rules! write_register {
    ( $device:expr, $register:expr, $bitflag:ident ) => {
        $device
            .smbus_write_byte_data($register, $bitflag.bits())
            .chain_err(|| ErrorKind::FailedToWriteRegister)
    }
}


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


/// Declare multiple bitflags using an abbreviated syntax.
///
/// All of the registers are 8 bits, with each flag being a single bit.
macro_rules! define_registers {
    (
        $(
            $name:ident {
                $( $shift:expr,$b:ident | )*
            }
        )*
    ) => {
        $(
            bitflags!{
                pub struct $name: u8 {
                    $(
                        #[allow(non_upper_case_globals)]
                        const $b = 1 << $shift;
                    )*
                }
            }
        )*
    }
}

// The following definitions cover many, but not all, of the LSM303 bitflags.
//
// Most of the flags are unique to a single register.
// In those cases, the names directly correspond to the register address.
//
// In a few cases, such as for `INT1_*_A` and `INT2_*_A`,
// multiple registers have the same flag types.
//
// Registers that contain numeric values for a single purpose,
// such as `TEMP_OUT_*_M` and `TIME_WINDOW_A` are also not defined here.

define_registers!{

    // Accelerometer

    CtrlReg1A {
        7, ODR3        | 6, ODR2        | 5, ODR1        | 4, ODR0        |
        3, LPen        | 2, Zen         | 1, Yen         | 0, Xen         |
    }
    CtrlReg2Af {
        7, HPM1        | 6, HPM0        | 5, HPCF2       | 4, HPCF1       |
        3, FDS         | 2, HPCLICK     | 1, HPIS2       | 0, HPIS1       |
    }
    CtrlReg3A {
        7, I1_CLICK    | 6, I1_AOI1     | 5, I1_AOI2     | 4, I1_DRDY1    |
        3, I1_DRDY2    | 2, I1_WTM      | 1, I1_OVERRUN  | /* ---------- */
    }
    CtrlReg4A {
        7, BDU         | 6, BLE         | 5, FS1         | 4, FS0         |
        3, HR          | /* ----------- | ------------- */ 0, SIM         |
    }
    CtrlReg5A {
        7, BOOT        | 6, FIFO_EN     | /* ----------- | ------------- */
        3, LIR_INT1    | 2, D4D_INT1    | 1, LIR_INT2    | 0, D4D_INT2    |
    }
    CtrlReg6A {
        7, I2_CLICK    | 6, I2_INT1     | 5, I2_INT2     | 4, BOOT_I1     |
        3, P2_ACT      | /* ---------- */ 1, H_LACTIVE   | /* ---------- */
    }
    Reference {
        7, Ref7        | 6, Ref6        | 5, Ref5        | 4, Ref4        |
        3, Ref3        | 2, Ref2        | 1, Ref1        | 0, Ref0        |
    }
    StatusRegA {
        7, ZYXOR       | 6, ZOR         | 5, YOR         | 4, XOR         |
        3, ZYXDA       | 2, ZDA         | 1, YDA         | 0, XDA         |
    }
    FifoCtrlRegA {
        7, FM1         | 6, FM0         | 5, TR          | 4, FTH4        |
        3, FTH3        | 2, FTH2        | 1, FTH1        | 0, FTH0        |
    }
    FifoSrcRegA {
        7, WTM         | 6, OVRN_FIFO   | 5, EMPTY       | 4, FSS4        |
        3, FSS3        | 2, FSS2        | 1, FSS1        | 0, FSS0        |
    }
    IntCfgA {
        7, AOI         | 6, _6D         | 5, ZHIE        | 4, ZLIO        |
        3, YHIE        | 2, YLIE        | 1, XHIE        | 0, XLIE        |
    }
    IntSrcA {
        /* ---------- */ 6, IA          | 5, ZH          | 4, ZL          |
        3, YH          | 2, YL          | 1, XH          | 0, XL          |
    }
    IntDurationA {
        /* ---------- */ 6, D6          | 5, D5          | 4, D4          |
        3, D3          | 2, D2          | 1, D1          | 0, D0          |
    }
    ClickCfgA {
        /* ----------- | ------------- */ 5, ZD          | 4, ZS          |
        3, YD          | 2, YS          | 1, XD          | 0, XS          |
    }
    ClickSrcA {
        /* ---------- */ 6, IA_click    | 5, DCLICK      | 5, SCLICK      |
        3, Sign        | 2, Z           | 1, Y           | 0, X           |
    }

    // Magnetometer

    CraRegM {
        7, TEMP_EN     | /* ----------- | ------------- */ 4, DO2         |
        3, DO1         | 2, DO0         | /* ----------- | ------------- */
    }
    CrbRegM {
        7, GN2         | 6, GN1         | 5, GN0         | /* ---------- */
        /* ----------- | -------------- | -------------- | ------------- */
    }
    MrRegM {
        /* ----------- | -------------- | -------------- | ------------- */
        /* ----------- | ------------- */ 1, MD1         | 0, MD0         |
    }
    SrRegM {
        /* ----------- | -------------- | -------------- | ------------- */
        /* ----------- | ------------- */ 1, LOCK        | 0, DRDY        |
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
