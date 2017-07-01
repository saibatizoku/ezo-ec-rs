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
