//! A subset of the registers on the LSM303 - just the ones that we need.

pub const ACCEL_CTRL_REG1_A: u8 = 0x20;
pub const ACCEL_CTRL_REG4_A: u8 = 0x23;
pub const ACCEL_OUT_X_L_A: u8 = 0x28;
pub const MAG_CRB_REG_M: u8 = 0x01;
pub const MAG_MR_REG_M: u8 = 0x02;
pub const MAG_OUT_X_H_M: u8 = 0x03;

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
    pub struct MrRegM: u8 {
        const MD1 = 1 << 1;
        const MD0 = 1 << 0;

        const MODE_CONTINUOUS = 0;
        const MODE_SINGLE_CONVERSION = MD0.bits;
        const SLEEP_MODE = MD1.bits;
    }
}
