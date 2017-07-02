//! I2C Commands for EC EZO Chip, taken from their Datasheet.
//! This chip is used for electrical conductivity measurement. It features
//! calibration, sleep mode, scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate i2cdev;

mod errors { error_chain! {} }

use errors::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

pub const MAX_RESPONSE_LENGTH: usize = 399;

/// Useful for properly building I2C parameters from a command.
pub trait I2cCommand {
    fn build(&self) -> CommandOptions;
}

/// Allowable baudrates used when changing the chip to UART mode.
#[derive(Debug)]
pub enum BpsRate {
    Bps300 = 300,
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

/// Commands for interacting with the EC EZO chip.
pub enum ConductivityCommand {
    // `Baud` command
    Baud(BpsRate),
    // `Cal` command.
    CalibrationDry,
    CalibrationSinglePoint(f64),
    CalibrationLow(f64),
    CalibrationHigh(f64),
    CalibrationClear,
    CalibrationState,
    // `Export`/`Import` command
    Export,
    ExportInfo,
    Import(String),
    // `Factory` command
    Factory,
    // `Find` command
    Find,
    // `I` command
    DeviceInformation,
    // `I2C` command.
    DeviceAddress(u8),
    // `K` command
    ProbeTypePointOne,
    ProbeTypeOne,
    ProbeTypeTen,
    ProbeTypeState,
    // `L` command
    LedOn,
    LedOff,
    LedState,
    // `O` command
    OutputDisableConductivity,
    OutputEnableConductivity,
    OutputDisableTds,
    OutputEnableTds,
    OutputDisableSalinity,
    OutputEnableSalinity,
    OutputDisableSpecificGravity,
    OutputEnableSpecificGravity,
    OutputState,
    // `Plock` command
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockState,
    // `R` command
    Reading,
    // `Sleep` command
    Sleep,
    // `Status` command
    Status,
    // `T` command
    TemperatureCompensation(f64),
    TemperatureCompensationValue,
}

/// Command-related parameters used to build I2C write/read interactions.
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct CommandOptions {
    pub command: String,
    pub delay: Option<u64>,
    pub response: Option<CommandResponse>,
}

/// Allowed responses from I2C read interactions.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum CommandResponse {
    Ack,
    CalibrationState,
    CompensationValue,
    DeviceInformation,
    Export,
    ExportInfo,
    LedState,
    OutputState,
    ProtocolLockState,
    ProbeTypeState,
    Reading,
    Status,
}

pub trait CommandBuilder {
    fn finish(&self) -> CommandOptions;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<()>;
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions;
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions;
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions;
}

impl CommandBuilder for CommandOptions {
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
        println!("COMMAND: {}", self.command);
        if let Err(_) = dev.write(self.command.as_bytes()) {
            thread::sleep(Duration::from_millis(300));
            dev.write(self.command.as_bytes())
                .chain_err(|| "Command could not be sent")?;
        };
        if let Some(delay) = self.delay {
            thread::sleep(Duration::from_millis(delay));
        }
        if let Some(_) = self.response {
            if let Err(_) = dev.read(&mut data_buffer) {
                thread::sleep(Duration::from_millis(300));
                dev.read(&mut data_buffer)
                    .chain_err(|| "Error reading from device")?;
            };
            match data_buffer[0] {
                255 => println!("No data expected."),
                254 => println!("Pending"),
                2 => println!("Error"),
                1 => {
                    let data: String = match data_buffer.into_iter().position(|&x| x == 0) {
                        Some(eol) => {
                            data_buffer[1..eol]
                                .into_iter()
                                .map(|c| (*c & !0x80) as char)
                                .collect()
                        }
                        _ => {
                            String::from_utf8(Vec::from(&data_buffer[1..]))
                                .chain_err(|| "Data is not readable")?
                        }
                    };
                    println!("RESPONSE: {}", data);
                }
                _ => println!("NO RESPONSE"),
            };
        }
        println!();
        Ok(())
    }
    /// Sets the ASCII string for the command to be sent
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions {
        self.command = command_str;
        self
    }
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions {
        self.delay = Some(delay);
        self
    }
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions {
        self.response = Some(response);
        self
    }
}

impl I2cCommand for ConductivityCommand {
    fn build(&self) -> CommandOptions {
        use self::ConductivityCommand::*;
        let mut opts = CommandOptions::default();
        match *self {
            Baud(ref baud) => {
                let rate = match *baud {
                    BpsRate::Bps300 => BpsRate::Bps300 as u32,
                    BpsRate::Bps1200 => BpsRate::Bps1200 as u32,
                    BpsRate::Bps2400 => BpsRate::Bps2400 as u32,
                    BpsRate::Bps9600 => BpsRate::Bps9600 as u32,
                    BpsRate::Bps19200 => BpsRate::Bps19200 as u32,
                    BpsRate::Bps38400 => BpsRate::Bps38400 as u32,
                    BpsRate::Bps57600 => BpsRate::Bps57600 as u32,
                    BpsRate::Bps115200 => BpsRate::Bps115200 as u32,
                };
                opts.set_command(format!("Baud,{}\0", rate)).finish()
            }
            CalibrationDry => {
                opts.set_command("Cal,dry\0".to_string())
                    .set_delay(800)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationSinglePoint(cal) => {
                opts.set_command(format!("Cal,{}\0", cal))
                    .set_delay(800)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationLow(cal) => {
                opts.set_command(format!("Cal,low,{}\0", cal))
                    .set_delay(800)
                    .set_response(CommandResponse::Ack)
                    .finish()
            },
            CalibrationHigh(cal) => {
                opts.set_command(format!("Cal,high,{}\0", cal))
                    .set_delay(800)
                    .set_response(CommandResponse::Ack)
                    .finish()
            },
            CalibrationClear => {
                opts.set_command("Cal,clear\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationState => {
                opts.set_command("Cal,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::CalibrationState)
                    .finish()
            }
            Export => {
                opts.set_command("Export\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Export)
                    .finish()
            }
            ExportInfo => {
                opts.set_command("Export,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ExportInfo)
                    .finish()
            }
            Import(ref calib) => {
                opts.set_command(format!("Import,{}\0", calib))
                    .set_delay(300)
                    .finish()
            }
            Factory => opts.set_command("Factory\0".to_string()).finish(),
            Find => {
                opts.set_command("F\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            DeviceInformation => {
                opts.set_command("I\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::DeviceInformation)
                    .finish()
            }
            DeviceAddress(addr) => {
                opts.set_command(format!("I2C,{}\0", addr))
                    .set_delay(300)
                    .finish()
            }
            ProbeTypePointOne => {
                opts.set_command("K,0.1\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProbeTypeOne => {
                opts.set_command("K,1.0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProbeTypeTen => {
                opts.set_command("K,10.0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProbeTypeState => {
                opts.set_command("K,?\0".to_string())
                    .set_delay(600)
                    .set_response(CommandResponse::ProbeTypeState)
                    .finish()
            }
            LedOn => unimplemented!(),
            LedOff => unimplemented!(),
            LedState => unimplemented!(),
            OutputDisableConductivity => unimplemented!(),
            OutputEnableConductivity => unimplemented!(),
            OutputDisableTds => unimplemented!(),
            OutputEnableTds => unimplemented!(),
            OutputDisableSalinity => unimplemented!(),
            OutputEnableSalinity => unimplemented!(),
            OutputDisableSpecificGravity => unimplemented!(),
            OutputEnableSpecificGravity => unimplemented!(),
            OutputState => unimplemented!(),
            ProtocolLockEnable => unimplemented!(),
            ProtocolLockDisable => unimplemented!(),
            ProtocolLockState => unimplemented!(),
            Reading => unimplemented!(),
            Sleep => unimplemented!(),
            Status => unimplemented!(),
            TemperatureCompensation(temp) => unimplemented!(),
            TemperatureCompensationValue => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ConductivityCommand::*;

    #[test]
    fn build_command_baud_300() {
        let cmd = Baud(BpsRate::Bps300).build();
        assert_eq!(cmd.command, "Baud,300\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_1200() {
        let cmd = Baud(BpsRate::Bps1200).build();
        assert_eq!(cmd.command, "Baud,1200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_2400() {
        let cmd = Baud(BpsRate::Bps2400).build();
        assert_eq!(cmd.command, "Baud,2400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_9600() {
        let cmd = Baud(BpsRate::Bps9600).build();
        assert_eq!(cmd.command, "Baud,9600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_19200() {
        let cmd = Baud(BpsRate::Bps19200).build();
        assert_eq!(cmd.command, "Baud,19200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_38400() {
        let cmd = Baud(BpsRate::Bps38400).build();
        assert_eq!(cmd.command, "Baud,38400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_57600() {
        let cmd = Baud(BpsRate::Bps57600).build();
        assert_eq!(cmd.command, "Baud,57600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_115200() {
        let cmd = Baud(BpsRate::Bps115200).build();
        assert_eq!(cmd.command, "Baud,115200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_calibration_dry() {
        let cmd = CalibrationDry.build();
        assert_eq!(cmd.command, "Cal,dry\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_single_point() {
        let cmd = CalibrationSinglePoint(84.).build();
        assert_eq!(cmd.command, "Cal,84\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(12800.).build();
        assert_eq!(cmd.command, "Cal,high,12800\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_low() {
        let cmd = CalibrationLow(1413.).build();
        assert_eq!(cmd.command, "Cal,low,1413\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear.build();
        assert_eq!(cmd.command, "Cal,clear\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState.build();
        assert_eq!(cmd.command, "Cal,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::CalibrationState));
    }

    #[test]
    fn build_command_export() {
        let cmd = Export.build();
        assert_eq!(cmd.command, "Export\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Export));
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo.build();
        assert_eq!(cmd.command, "Export,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ExportInfo));
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string).build();
        assert_eq!(cmd.command, "Import,ABCDEFGHIJKLMNO\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory.build();
        assert_eq!(cmd.command, "Factory\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find.build();
        assert_eq!(cmd.command, "F\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation.build();
        assert_eq!(cmd.command, "I\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::DeviceInformation));
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88).build();
        assert_eq!(cmd.command, "I2C,88\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_probe_type_point_one() {
        let cmd = ProbeTypePointOne.build();
        assert_eq!(cmd.command, "K,0.1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_probe_type_one() {
        let cmd = ProbeTypeOne.build();
        assert_eq!(cmd.command, "K,1.0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_probe_type_ten() {
        let cmd = ProbeTypeTen.build();
        assert_eq!(cmd.command, "K,10.0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_probe_type_state() {
        let cmd = ProbeTypeState.build();
        assert_eq!(cmd.command, "K,?\0");
        assert_eq!(cmd.delay, Some(600));
        assert_eq!(cmd.response, Some(CommandResponse::ProbeTypeState));
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn.build();
        assert_eq!(cmd.command, "L,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff.build();
        assert_eq!(cmd.command, "L,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState.build();
        assert_eq!(cmd.command, "L,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::LedState));
    }

    #[test]
    fn build_command_output_disable_conductivity() {
        let cmd = OutputDisableConductivity.build();
        assert_eq!(cmd.command, "O,EC,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_enable_conductivity() {
        let cmd = OutputEnableConductivity.build();
        assert_eq!(cmd.command, "O,EC,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_disable_tds() {
        let cmd = OutputDisableTds.build();
        assert_eq!(cmd.command, "O,TDS,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_enable_tds() {
        let cmd = OutputDisableTds.build();
        assert_eq!(cmd.command, "O,EC,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_disable_salinity() {
        let cmd = OutputDisableSalinity.build();
        assert_eq!(cmd.command, "O,S,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_enable_salinity() {
        let cmd = OutputDisableSalinity.build();
        assert_eq!(cmd.command, "O,S,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_disable_specific_gravity() {
        let cmd = OutputDisableSpecificGravity.build();
        assert_eq!(cmd.command, "O,S,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_enable_specific_gravity() {
        let cmd = OutputEnableSpecificGravity.build();
        assert_eq!(cmd.command, "O,S,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_output_state() {
        let cmd = OutputState.build();
        assert_eq!(cmd.command, "O,S,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::OutputState));
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable.build();
        assert_eq!(cmd.command, "Plock,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable.build();
        assert_eq!(cmd.command, "Plock,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState.build();
        assert_eq!(cmd.command, "Plock,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ProtocolLockState));
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading.build();
        assert_eq!(cmd.command, "R\0");
        assert_eq!(cmd.delay, Some(600));
        assert_eq!(cmd.response, Some(CommandResponse::Reading));
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep.build();
        assert_eq!(cmd.command, "Sleep\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status.build();
        assert_eq!(cmd.command, "Status\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Status));
    }

    #[test]
    fn build_command_temperature_compensation() {
        let cmd = TemperatureCompensation(19.5).build();
        assert_eq!(cmd.command, "T,19.5\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_temperature_compensation_value() {
        let cmd = TemperatureCompensationValue.build();
        assert_eq!(cmd.command, "T,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::CompensationValue));
    }
}
