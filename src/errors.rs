//! The error type for this crate.

error_chain!{
    errors {
        /// An insufficient amount of data was read from the device.
        NotEnoughData{}
    }

    foreign_links {
        I2C(::i2cdev::linux::LinuxI2CError) #[doc = "An error from an I2C device."];
        ByteOrder(::byteorder::Error) #[doc = "An error converting bytes."];
    }
}
