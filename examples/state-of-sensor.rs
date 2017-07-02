#![recursion_limit = "1024"]
//! An example that retrieves the current settings of the RTD EC chip.
//!
extern crate ezo_rtd;
extern crate i2cdev;

use ezo_rtd::errors::*;
use ezo_rtd::{CommandBuilder, I2cCommand, TemperatureCommand};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    TemperatureCommand::Status.build().run(&mut dev)?;
    TemperatureCommand::CalibrationState.build().run(&mut dev)?;
    TemperatureCommand::OutputState.build().run(&mut dev)?;
    TemperatureCommand::LedState.build().run(&mut dev)?;
    TemperatureCommand::ExportInfo.build().run(&mut dev)?;
    TemperatureCommand::Sleep.build().run(&mut dev)?;
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}
