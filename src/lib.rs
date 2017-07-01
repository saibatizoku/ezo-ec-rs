//! I2C Commands for EC EZO Chip, taken from their Datasheet.
//! This chip is used for electrical conductivity measurement. It features
//! calibration, sleep mode, scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate i2cdev;

mod errors { error_chain! {} }

use errors::*;

pub trait I2cCommand {
    fn build(&self) -> CommandOptions;
}

#[derive(Debug)]
pub enum Rate {
    Bps300 = 300,
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

pub enum ConductivityCommand {
    // `Baud` command
    Baud(Rate),
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
    ProbeTypeStatus,
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
    OutputStatus,
    // `Plock` command
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockStatus,
    // `R` command
    Reading,
    // `Sleep` command
    Sleep,
    // `Status` command
    Status,
    // `T` command
    TemperatureCompensation,
    TemperatureCompensatedValue,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct CommandOptions {
    pub command: String,
    pub delay: Option<u64>,
    pub response: Option<CommandResponse>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum CommandResponse {
    Ack,
    CalibrationState,
    Export,
    ExportInfo,
}

impl I2cCommand for ConductivityCommand {
    fn build(&self) -> CommandOptions {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ConductivityCommand::*;

    #[test]
    fn build_command_baud_300() {
        let cmd = Baud(Rate::Bps300).build();
        assert_eq!(cmd.command, "Baud,300\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_1200() {
        let cmd = Baud(Rate::Bps1200).build();
        assert_eq!(cmd.command, "Baud,1200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_2400() {
        let cmd = Baud(Rate::Bps2400).build();
        assert_eq!(cmd.command, "Baud,2400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_9600() {
        let cmd = Baud(Rate::Bps9600).build();
        assert_eq!(cmd.command, "Baud,9600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_19200() {
        let cmd = Baud(Rate::Bps19200).build();
        assert_eq!(cmd.command, "Baud,19200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_38400() {
        let cmd = Baud(Rate::Bps38400).build();
        assert_eq!(cmd.command, "Baud,38400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_57600() {
        let cmd = Baud(Rate::Bps57600).build();
        assert_eq!(cmd.command, "Baud,57600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_115200() {
        let cmd = Baud(Rate::Bps115200).build();
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
}
