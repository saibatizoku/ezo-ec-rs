//! An example that retrieves the current settings of the EC EZO chip.
//!

#![recursion_limit = "1024"]

extern crate ezo_ec;
extern crate i2cdev;

use ezo_ec::errors::*;

use ezo_ec::command::{Command, DeviceInformation, CalibrationState, LedState, OutputState, Reading, Sleep, Status};

use ezo_ec::response::{CalibrationStatus, DeviceInfo, DeviceStatus, LedStatus, OutputStringStatus, ProbeReading};

use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    let info: DeviceInfo = DeviceInformation.run(&mut dev)?;
    println!("{:?}", info);

    let status: DeviceStatus = Status.run(&mut dev)?;
    println!("DeviceStatus: {:?}", status);

    let calibration: CalibrationStatus = CalibrationState.run(&mut dev)?;
    println!("CalibrationState: {:?}", calibration);

    let led_status: LedStatus = LedState.run(&mut dev)?;
    println!("LedState: {:?}", led_status);

    let ec_value: ProbeReading = Reading.run(&mut dev)?;
    println!("{:?}", ec_value);

    let output_string: OutputStringStatus = OutputState.run(&mut dev)?;
    println!("{:?}", output_string);

    let _sleep = Sleep.run(&mut dev)?;
    println!("Sleeping....");

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
