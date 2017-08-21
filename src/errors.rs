// This can probably be removed soon. See:
// https://github.com/steveklabnik/rustdoc/issues/96
#![allow(unused_doc_comment)]

//! The error type for this crate.

error_chain!{
    errors {
        /// Error opening the I2C device
        FailedToOpenDevice{}

        /// An insufficient amount of data was read from the device.
        NotEnoughData{}

        /// An error occurred receiving information from the I2C slave.
        FailedToReadRegister{}

        /// An error occurred sending information to the I2C slave.
        FailedToWriteRegister{}
    }

    foreign_links {
        I2C(::i2cdev::linux::LinuxI2CError) #[doc = "An error from an I2C device."];
        ByteOrder(::byteorder::Error) #[doc = "An error converting bytes."];
    }
}
