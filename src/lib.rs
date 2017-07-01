//! I2C Commands for EC EZO Chip, taken from their Datasheet.
//! This chip is used for electrical conductivity measurement. It features
//! calibration, sleep mode, scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate i2cdev;

mod errors { error_chain! {} }

use errors::*;

enum EcEzoCommand {
    Baud(u16),
    CalibrationDry,
    CalibrationSinglePoint(f64),
    CalibrationLowEnd(u16),
    CalibrationHighEnd(u16),
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod(u8),
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress(u8),
    DeviceInformation,
    Export,
    ExportInfo,
    Import(String),
    Factory,
    Find,
    ProbeTypePointOne,
    ProbeTypeOne,
    ProbeTypeTen,
    ProbeTypeStatus,
    LedOn,
    LedOff,
    LedState,
    OutputDisableConductivity,
    OutputEnableConductivity,
    OutputDisableTds,
    OutputEnableTds,
    OutputDisableSalinity,
    OutputEnableSalinity,
    OutputDisableSpecificGravity,
    OutputEnableSpecificGravity,
    OutputStatus,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockStatus,
    Reading,
    Sleep,
    Status,
    TemperatureCompensation,
    TemperatureCompensatedValue,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ConductivityCommand::*;

    #[test]
    fn build_command_uart_300() {
        let cmd = SetUart(Bauds::Bps300).build();
        assert_eq!(cmd.command, "Baud,300\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_1200() {
        let cmd = SetUart(Bauds::Bps1200).build();
        assert_eq!(cmd.command, "Baud,1200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_2400() {
        let cmd = SetUart(Bauds::Bps2400).build();
        assert_eq!(cmd.command, "Baud,2400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_9600() {
        let cmd = SetUart(Bauds::Bps9600).build();
        assert_eq!(cmd.command, "Baud,9600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_19200() {
        let cmd = SetUart(Bauds::Bps19200).build();
        assert_eq!(cmd.command, "Baud,19200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_38400() {
        let cmd = SetUart(Bauds::Bps38400).build();
        assert_eq!(cmd.command, "Baud,38400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_57600() {
        let cmd = SetUart(Bauds::Bps57600).build();
        assert_eq!(cmd.command, "Baud,57600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_uart_115200() {
        let cmd = SetUart(Bauds::Bps115200).build();
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

    fn build_command_calibration_single_point() {
        let cmd = CalibrationSinglePoint(84.).build();
        assert_eq!(cmd.command, "Cal,84\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(12800.).build();
        assert_eq!(cmd.command, "Cal,high,12800\0");
        assert_eq!(cmd.delay, Some(800));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

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
}
