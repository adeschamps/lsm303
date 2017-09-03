#![allow(unused_doc_comment)]

#[macro_use]
extern crate error_chain;

extern crate i2cdev;

extern crate lsm303;

error_chain!{}

quick_main!(run);

fn run() -> Result<()> {
    let device = "/dev/i2c-1";

    let mut accelerometer =
        lsm303::Accelerometer::new(device).chain_err(|| "Failed to initialize the accelerometer")?;
    let mut magnetometer =
        lsm303::Magnetometer::new(device).chain_err(|| "Failed to initialize the magnetometer")?;

    loop {
        let accel = accelerometer.read_acceleration()
            .chain_err(|| "Failed to read the accelerometer")?;
        let (m_x, m_y, m_z) = magnetometer.read_magnetic_field()
            .chain_err(|| "Failed to read the magnetometer")?;

        println!("Accel: {:4}, {:4}, {:4}  ||  Mag: {:02.4}, {:02.4}, {:02.4}",
                 accel.x,
                 accel.y,
                 accel.z,
                 m_x,
                 m_y,
                 m_z);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
