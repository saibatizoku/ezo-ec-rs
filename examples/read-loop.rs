//! An example that takes readings from the EC EZO chip in a loop.
//!
#![recursion_limit = "1024"]
extern crate chrono;
extern crate ezo_ec;
extern crate failure;
extern crate i2cdev;

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

use ezo_ec::command::{Command, OutputState, Reading, Sleep};
use ezo_ec::errors::*;
use ezo_ec::response::{OutputStringStatus, ProbeReading};

use failure::{Error, ResultExt};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

fn run() -> Result<(), Error> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);

    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .context("Could not open I2C device")?;

    let status = OutputState.run(&mut dev)?;

    loop {
        let ec_value = Reading.run(&mut dev)?;

        let _out = _print_response(ec_value, &status)?;

        let _sleep = Sleep.run(&mut dev)?;

        // Ideally, every 10 seconds, fine-tune this to your hardware.
        thread::sleep(Duration::from_millis(9_400));
    }
}

fn _print_response(reading: ProbeReading, status: &OutputStringStatus) -> Result<(), Error> {
    let dt: DateTime<Utc> = Utc::now();
    println!("{:?},{:?},{:?}", dt, reading, status,);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        let backtrace = e.backtrace();
        println!("backtrace: {:?}", backtrace);
        ::std::process::exit(1);
    }
}
