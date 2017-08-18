
/// The I2C address of the accelerometer.
pub const ADDRESS_ACCEL: u16 = 0x32 >> 1;

/// The I2C address of the magnetometer.
pub const ADDRESS_MAG: u16 = 0x3C >> 1;

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
