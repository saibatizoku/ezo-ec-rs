//! I2C Commands for EC EZO Chip.
//!
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    CompensationValue,
    DeviceInfo,
    DeviceStatus,
    Exported,
    ExportedInfo,
    ProbeType,
    LedStatus,
    OutputStringStatus,
    ProtocolLockStatus,
    ProbeReading,
};

use ezo_common::{
    BpsRate,
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_DATA: usize = 401;

/// I2C command for the EZO chip.
pub trait Command {
    type Response;

    fn get_command_string (&self) -> String;
    fn get_delay (&self) -> u64;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Self::Response>;
}

define_command! {
    doc: "`Baud,n` command, where `n` is a variant belonging to `BpsRate`. Switch chip to UART mode.",
    cmd: Baud(BpsRate), { format!("BAUD,{}", cmd.parse()) }, 0
}

impl FromStr for Baud {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("BAUD,") {
            let rest = supper.get(5..).unwrap();
            let mut split = rest.split(',');
            let rate = match split.next() {
                Some("300") => BpsRate::Bps300,
                Some("1200") => BpsRate::Bps1200,
                Some("2400") => BpsRate::Bps2400,
                Some("9600") => BpsRate::Bps9600,
                Some("19200") => BpsRate::Bps19200,
                Some("38400") => BpsRate::Bps38400,
                Some("57600") => BpsRate::Bps57600,
                Some("115200") => BpsRate::Bps115200,
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(Baud(rate)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,DRY` command. Performs calibration.",
    CalibrationDry, { "CAL,DRY".to_string() }, 800, Ack
}

impl FromStr for CalibrationDry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,DRY" => Ok(CalibrationDry),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}


define_command! {
    doc: "`CAL,n` command, where `n` is a `f64` number. Performs calibration.",
    cmd: CalibrationOnePoint(f64), { format!("CAL,{:.*}", 2, cmd) }, 800, Ack
}

impl FromStr for CalibrationOnePoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,") {
            let rest = supper.get(4..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => n.parse::<f64>().unwrap(),
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationOnePoint(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,LOW,t` command, where `t` is of type `f64`. Performs calibration.",
    cmd: CalibrationLow(f64), { format!("CAL,LOW,{:.*}", 2, cmd) }, 800, Ack
}

impl FromStr for CalibrationLow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,LOW,") {
            let rest = supper.get(8..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => n.parse::<f64>().unwrap(),
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationLow(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,HIGH,t` command, where `t` is of type `f64`. Performs calibration.",
    cmd: CalibrationHigh(f64), { format!("CAL,HIGH,{:.*}", 2, cmd) }, 800, Ack
}

impl FromStr for CalibrationHigh {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,HIGH,") {
            let rest = supper.get(9..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => n.parse::<f64>().unwrap(),
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationHigh(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,CLEAR` command. Clears current calibration.",
    CalibrationClear, { "CAL,CLEAR".to_string() }, 300, Ack
}

impl FromStr for CalibrationClear {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,CLEAR" => Ok(CalibrationClear),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`CAL,?` command. Returns a `CalibrationStatus` response. Current calibration status.",
    CalibrationState, { "CAL,?".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

impl FromStr for CalibrationState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,?" => Ok(CalibrationState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`EXPORT` command. Returns an `Exported` response. Exports current calibration.",
    Export, { "EXPORT".to_string() }, 300,
    resp: Exported, { Exported::parse(&resp) }
}

impl FromStr for Export {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "EXPORT" => Ok(Export),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`EXPORT,?` command. Returns an `ExportedInfo` response. Calibration string info.",
    ExportInfo, { "EXPORT,?".to_string() }, 300,
    resp: ExportedInfo, { ExportedInfo::parse(&resp) }
}

impl FromStr for ExportInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "EXPORT,?" => Ok(ExportInfo),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`IMPORT,n` command, where `n` is of type `String`.",
    cmd: Import(String), { format!("IMPORT,{}", cmd) }, 300, Ack
}

impl FromStr for Import {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("IMPORT,") {
            let rest = supper.get(7..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) if n.len() > 0 && n.len() < 13 => {
                    n.to_string()
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(Import(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`FACTORY` command. Enable factory reset.",
    Factory, { "FACTORY".to_string() }, 0
}

impl FromStr for Factory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "FACTORY" => Ok(Factory),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`FIND` command. Find device with blinking white LED.",
    Find, { "F".to_string() }, 300
}

impl FromStr for Find {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "F" => Ok(Find),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`I2C,n` command, where `n` is of type `u16`. Chance I2C address.",
    cmd: DeviceAddress(u16), { format!("I2C,{}", cmd) }, 300
}

impl FromStr for DeviceAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("I2C,") {
            let rest = supper.get(4..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<u16>().unwrap()
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(DeviceAddress(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`I` command. Returns a `DeviceInfo` response. Device information.",
    DeviceInformation, { "I".to_string() }, 300,
    resp: DeviceInfo, { DeviceInfo::parse(&resp) }
}

impl FromStr for DeviceInformation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "I" => Ok(DeviceInformation),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,1` command. Enable LED.",
    LedOn, { "L,1".to_string() }, 300, Ack
}

impl FromStr for LedOn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,1" => Ok(LedOn),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,0` command. Disable LED.",
    LedOff, { "L,0".to_string() }, 300, Ack
}

impl FromStr for LedOff {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,0" => Ok(LedOff),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,?` command. Returns a `LedStatus` response. Get current LED status.",
    LedState, { "L,?".to_string() }, 300,
    resp: LedStatus, { LedStatus::parse(&resp) }
}

impl FromStr for LedState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,?" => Ok(LedState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`K,0.1` command. Set probe type to `0.1`.",
    ProbeTypePointOne, { "K,0.1".to_string() }, 600, Ack
}

impl FromStr for ProbeTypePointOne {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "K,0.1" => Ok(ProbeTypePointOne),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`K,1.0` command. Set probe type to `1.0`.",
    ProbeTypeOne, { "K,1.0".to_string() }, 600, Ack
}

impl FromStr for ProbeTypeOne {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "K,1.0" => Ok(ProbeTypeOne),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`K,10.0` command. Set probe type to `10.0`.",
    ProbeTypeTen, { "K,10.0".to_string() }, 600, Ack
}

impl FromStr for ProbeTypeTen {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "K,10.0" => Ok(ProbeTypeTen),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`K,?` command. Returns a `ProbeType` response. Get current probe type.",
    ProbeTypeState, { "K,?".to_string() }, 300,
    resp: ProbeType, { ProbeType::parse(&resp) }
}

impl FromStr for ProbeTypeState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "K,?" => Ok(ProbeTypeState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`PLOCK,1` command. Enable I2C protocol lock.",
    ProtocolLockEnable, { "PLOCK,1".to_string() }, 300, Ack
}

impl FromStr for ProtocolLockEnable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,1" => Ok(ProtocolLockEnable),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`PLOCK,0` command. Disable I2C protocol lock.",
    ProtocolLockDisable, { "PLOCK,0".to_string() }, 300, Ack
}

impl FromStr for ProtocolLockDisable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,0" => Ok(ProtocolLockDisable),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`PLOCK,?` command. Returns a `ProtocolLockStatus` response. Get the Protocol Lock status.",
    ProtocolLockState, { "PLOCK,?".to_string() }, 300,
    resp: ProtocolLockStatus, { ProtocolLockStatus::parse(&resp) }
}

impl FromStr for ProtocolLockState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,?" => Ok(ProtocolLockState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`R` command. Returns a `ProbeReading` response. Returns a single reading.",
    Reading, { "R".to_string() }, 600,
    resp: ProbeReading, { ProbeReading::parse(&resp) }
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "R" => Ok(Reading),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,EC,0` command. Disable conductivity in the output string.",
    OutputDisableConductivity, { "O,EC,0".to_string() }, 300, Ack
}

impl FromStr for OutputDisableConductivity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,EC,0" => Ok(OutputDisableConductivity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,EC,1` command. Enable conductivity in the output string.",
    OutputEnableConductivity, { "O,EC,1".to_string() }, 300, Ack
}

impl FromStr for OutputEnableConductivity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,EC,1" => Ok(OutputEnableConductivity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,TDS,0` command. Disable total dissolved solids in the output string.",
    OutputDisableTds, { "O,TDS,0".to_string() }, 300, Ack
}

impl FromStr for OutputDisableTds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,TDS,0" => Ok(OutputDisableTds),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,TDS,1` command. Enable total dissolved solids in the output string.",
    OutputEnableTds, { "O,TDS,1".to_string() }, 300, Ack
}

impl FromStr for OutputEnableTds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,TDS,1" => Ok(OutputEnableTds),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,S,0` command. Disable salinity in the output string.",
    OutputDisableSalinity, { "O,S,0".to_string() }, 300, Ack
}

impl FromStr for OutputDisableSalinity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,S,0" => Ok(OutputDisableSalinity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,S,1` command. Enable salinity in the output string.",
    OutputEnableSalinity, { "O,S,1".to_string() }, 300, Ack
}

impl FromStr for OutputEnableSalinity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,S,1" => Ok(OutputEnableSalinity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,SG,0` command. Disable specific gravity in the output string.",
    OutputDisableSpecificGravity, { "O,SG,0".to_string() }, 300, Ack
}

impl FromStr for OutputDisableSpecificGravity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,SG,0" => Ok(OutputDisableSpecificGravity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,SG,1` command. Enable specific gravity in the output string.",
    OutputEnableSpecificGravity, { "O,SG,1".to_string() }, 300, Ack
}

impl FromStr for OutputEnableSpecificGravity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,SG,1" => Ok(OutputEnableSpecificGravity),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`O,?` command. Returns an `OutputStringStatus` response. Displays the enabled parameters for the output string.",
    OutputState, { "O,?".to_string() }, 300,
    resp: OutputStringStatus, { OutputStringStatus::parse(&resp) }
}

impl FromStr for OutputState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "O,?" => Ok(OutputState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`STATUS` command. Returns a `DeviceStatus` response. Retrieve status information.",
    Status, { "STATUS".to_string() }, 300,
    resp: DeviceStatus, { DeviceStatus::parse(&resp) }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "STATUS" => Ok(Status),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`SLEEP` command. Enter sleep mode/low power.",
    Sleep, { "SLEEP".to_string() }, 0
}

impl FromStr for Sleep {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "SLEEP" => Ok(Sleep),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`T,t` command, where `t` is of type `f64`. Returns a `TemperatureCompensation` response. Temperature compensation.",
    cmd: TemperatureCompensation(f64), { format!("T,{:.*}", 3, cmd) }, 300, Ack
}

impl FromStr for TemperatureCompensation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("T,") {
            let rest = supper.get(2..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => n.parse::<f64>().unwrap(),
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(TemperatureCompensation(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`T,?` command. Returns a `CompensationValue` response. Compensated temperature value.",
    CompensatedTemperatureValue, { "T,?".to_string() }, 300,
    resp: CompensationValue, { CompensationValue::parse(&resp) }
}

impl FromStr for CompensatedTemperatureValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "T,?" => Ok(CompensatedTemperatureValue),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_baud_300() {
        let cmd = Baud(BpsRate::Bps300);
        assert_eq!(cmd.get_command_string(), "BAUD,300");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_300() {
        let cmd = "baud,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));

        let cmd = "BAUD,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));
    }

    #[test]
    fn build_command_baud_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "BAUD,1200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_1200() {
        let cmd = "baud,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));

        let cmd = "BAUD,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));
    }

    #[test]
    fn build_command_baud_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "BAUD,2400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_2400() {
        let cmd = "baud,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));

        let cmd = "BAUD,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));
    }

    #[test]
    fn build_command_baud_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "BAUD,9600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_9600() {
        let cmd = "baud,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));

        let cmd = "BAUD,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));
    }

    #[test]
    fn build_command_baud_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "BAUD,19200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_19200() {
        let cmd = "baud,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));

        let cmd = "BAUD,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));
    }

    #[test]
    fn build_command_baud_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "BAUD,38400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_38400() {
        let cmd = "baud,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));

        let cmd = "BAUD,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));
    }

    #[test]
    fn build_command_baud_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "BAUD,57600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_57600() {
        let cmd = "baud,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));

        let cmd = "BAUD,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));
    }

    #[test]
    fn build_command_baud_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "BAUD,115200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_115200() {
        let cmd = "baud,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));

        let cmd = "BAUD,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));
    }

    #[test]
    fn build_command_calibration_dry() {
        let cmd = CalibrationDry;
        assert_eq!(cmd.get_command_string(), "CAL,DRY");
        assert_eq!(cmd.get_delay(), 800);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_dry() {
        let cmd = "cal,dry".parse::<CalibrationDry>().unwrap();
        assert_eq!(cmd, CalibrationDry);

        let cmd = "Cal,Dry".parse::<CalibrationDry>().unwrap();
        assert_eq!(cmd, CalibrationDry);
    }

    #[test]
    fn build_command_calibration_one_point() {
        let cmd = CalibrationOnePoint(84.);
        assert_eq!(cmd.get_command_string(), "CAL,84.00");
        assert_eq!(cmd.get_delay(), 800);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_one_point() {
        let cmd = "cal,0".parse::<CalibrationOnePoint>().unwrap();
        assert_eq!(cmd, CalibrationOnePoint(0_f64));

        let cmd = "Cal,11.43".parse::<CalibrationOnePoint>().unwrap();
        assert_eq!(cmd, CalibrationOnePoint(11.43));
    }

    #[test]
    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(12800.);
        assert_eq!(cmd.get_command_string(), "CAL,HIGH,12800.00");
        assert_eq!(cmd.get_delay(), 800);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_high() {
        let cmd = "cal,high,0".parse::<CalibrationHigh>().unwrap();
        assert_eq!(cmd, CalibrationHigh(0_f64));

        let cmd = "Cal,HIGH,4121.43".parse::<CalibrationHigh>().unwrap();
        assert_eq!(cmd, CalibrationHigh(4121.43));
    }

    #[test]
    fn build_command_calibration_low() {
        let cmd = CalibrationLow(1413.);
        assert_eq!(cmd.get_command_string(), "CAL,LOW,1413.00");
        assert_eq!(cmd.get_delay(), 800);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_low() {
        let cmd = "cal,low,0".parse::<CalibrationLow>().unwrap();
        assert_eq!(cmd, CalibrationLow(0_f64));

        let cmd = "Cal,loW,-121.43".parse::<CalibrationLow>().unwrap();
        assert_eq!(cmd, CalibrationLow(-121.43));
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "CAL,CLEAR");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_clear() {
        let cmd = "cal,clear".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);

        let cmd = "Cal,CLEAR".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "CAL,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_state() {
        let cmd = "cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);

        let cmd = "Cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);
    }

    #[test]
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "EXPORT");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export() {
        let cmd = "export".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);

        let cmd = "EXPORT".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "EXPORT,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export_info() {
        let cmd = "export,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);

        let cmd = "EXPORT,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "IMPORT,ABCDEFGHIJKLMNO");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_import() {
        let cmd = "import,1".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("1".to_string()));

        let cmd = "IMPORT,abcdef".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("ABCDEF".to_string()));
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "FACTORY");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_factory() {
        let cmd = "factory".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);

        let cmd = "FACTORY".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_find() {
        let cmd = "f".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);

        let cmd = "F".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_information() {
        let cmd = "i".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);

        let cmd = "I".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_device_address() {
        let cmd = "i2c,1".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(1));

        let cmd = "I2C,123".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(123));
    }

    #[test]
    fn build_command_probe_type_point_one() {
        let cmd = ProbeTypePointOne;
        assert_eq!(cmd.get_command_string(), "K,0.1");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn parse_case_insensitive_command_probe_type_point_one() {
        let cmd = "k,0.1".parse::<ProbeTypePointOne>().unwrap();
        assert_eq!(cmd, ProbeTypePointOne);

        let cmd = "K,0.1".parse::<ProbeTypePointOne>().unwrap();
        assert_eq!(cmd, ProbeTypePointOne);
    }

    #[test]
    fn build_command_probe_type_one() {
        let cmd = ProbeTypeOne;
        assert_eq!(cmd.get_command_string(), "K,1.0");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn parse_case_insensitive_command_probe_type_one() {
        let cmd = "k,1.0".parse::<ProbeTypeOne>().unwrap();
        assert_eq!(cmd, ProbeTypeOne);

        let cmd = "K,1.0".parse::<ProbeTypeOne>().unwrap();
        assert_eq!(cmd, ProbeTypeOne);
    }

    #[test]
    fn build_command_probe_type_ten() {
        let cmd = ProbeTypeTen;
        assert_eq!(cmd.get_command_string(), "K,10.0");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn parse_case_insensitive_command_probe_type_ten() {
        let cmd = "k,10.0".parse::<ProbeTypeTen>().unwrap();
        assert_eq!(cmd, ProbeTypeTen);

        let cmd = "K,10.0".parse::<ProbeTypeTen>().unwrap();
        assert_eq!(cmd, ProbeTypeTen);
    }

    #[test]
    fn build_command_probe_type_state() {
        let cmd = ProbeTypeState;
        assert_eq!(cmd.get_command_string(), "K,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_probe_type_state() {
        let cmd = "k,?".parse::<ProbeTypeState>().unwrap();
        assert_eq!(cmd, ProbeTypeState);

        let cmd = "K,?".parse::<ProbeTypeState>().unwrap();
        assert_eq!(cmd, ProbeTypeState);
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn;
        assert_eq!(cmd.get_command_string(), "L,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_on() {
        let cmd = "l,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);

        let cmd = "L,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff;
        assert_eq!(cmd.get_command_string(), "L,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_off() {
        let cmd = "l,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);

        let cmd = "L,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState;
        assert_eq!(cmd.get_command_string(), "L,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_state() {
        let cmd = "l,?".parse::<LedState>().unwrap();
        assert_eq!(cmd, LedState);

        let cmd = "L,?".parse::<LedState>().unwrap();
        assert_eq!(cmd, LedState);
    }

    #[test]
    fn build_command_output_disable_conductivity() {
        let cmd = OutputDisableConductivity;
        assert_eq!(cmd.get_command_string(), "O,EC,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_disable_conductivity() {
        let cmd = "o,ec,0".parse::<OutputDisableConductivity>().unwrap();
        assert_eq!(cmd, OutputDisableConductivity);

        let cmd = "O,EC,0".parse::<OutputDisableConductivity>().unwrap();
        assert_eq!(cmd, OutputDisableConductivity);
    }

    #[test]
    fn build_command_output_enable_conductivity() {
        let cmd = OutputEnableConductivity;
        assert_eq!(cmd.get_command_string(), "O,EC,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_enable_conductivity() {
        let cmd = "o,ec,1".parse::<OutputEnableConductivity>().unwrap();
        assert_eq!(cmd, OutputEnableConductivity);

        let cmd = "O,EC,1".parse::<OutputEnableConductivity>().unwrap();
        assert_eq!(cmd, OutputEnableConductivity);
    }

    #[test]
    fn build_command_output_disable_tds() {
        let cmd = OutputDisableTds;
        assert_eq!(cmd.get_command_string(), "O,TDS,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_disable_tds() {
        let cmd = "o,tds,0".parse::<OutputDisableTds>().unwrap();
        assert_eq!(cmd, OutputDisableTds);

        let cmd = "O,TDS,0".parse::<OutputDisableTds>().unwrap();
        assert_eq!(cmd, OutputDisableTds);
    }

    #[test]
    fn build_command_output_enable_tds() {
        let cmd = OutputEnableTds;
        assert_eq!(cmd.get_command_string(), "O,TDS,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_enable_tds() {
        let cmd = "o,tds,1".parse::<OutputEnableTds>().unwrap();
        assert_eq!(cmd, OutputEnableTds);

        let cmd = "O,TDS,1".parse::<OutputEnableTds>().unwrap();
        assert_eq!(cmd, OutputEnableTds);
    }

    #[test]
    fn build_command_output_disable_salinity() {
        let cmd = OutputDisableSalinity;
        assert_eq!(cmd.get_command_string(), "O,S,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_disable_salinity() {
        let cmd = "o,s,0".parse::<OutputDisableSalinity>().unwrap();
        assert_eq!(cmd, OutputDisableSalinity);

        let cmd = "O,S,0".parse::<OutputDisableSalinity>().unwrap();
        assert_eq!(cmd, OutputDisableSalinity);
    }

    #[test]
    fn build_command_output_enable_salinity() {
        let cmd = OutputEnableSalinity;
        assert_eq!(cmd.get_command_string(), "O,S,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_enable_salinity() {
        let cmd = "o,s,1".parse::<OutputEnableSalinity>().unwrap();
        assert_eq!(cmd, OutputEnableSalinity);

        let cmd = "O,S,1".parse::<OutputEnableSalinity>().unwrap();
        assert_eq!(cmd, OutputEnableSalinity);
    }

    #[test]
    fn build_command_output_disable_specific_gravity() {
        let cmd = OutputDisableSpecificGravity;
        assert_eq!(cmd.get_command_string(), "O,SG,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_disable_specific_gravity() {
        let cmd = "o,sg,0".parse::<OutputDisableSpecificGravity>().unwrap();
        assert_eq!(cmd, OutputDisableSpecificGravity);

        let cmd = "O,SG,0".parse::<OutputDisableSpecificGravity>().unwrap();
        assert_eq!(cmd, OutputDisableSpecificGravity);
    }

    #[test]
    fn build_command_output_enable_specific_gravity() {
        let cmd = OutputEnableSpecificGravity;
        assert_eq!(cmd.get_command_string(), "O,SG,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_enable_specific_gravity() {
        let cmd = "o,sg,1".parse::<OutputEnableSpecificGravity>().unwrap();
        assert_eq!(cmd, OutputEnableSpecificGravity);

        let cmd = "O,SG,1".parse::<OutputEnableSpecificGravity>().unwrap();
        assert_eq!(cmd, OutputEnableSpecificGravity);
    }

    #[test]
    fn build_command_output_state() {
        let cmd = OutputState;
        assert_eq!(cmd.get_command_string(), "O,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_output_state() {
        let cmd = "o,?".parse::<OutputState>().unwrap();
        assert_eq!(cmd, OutputState);

        let cmd = "O,?".parse::<OutputState>().unwrap();
        assert_eq!(cmd, OutputState);
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable;
        assert_eq!(cmd.get_command_string(), "PLOCK,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_enable() {
        let cmd = "plock,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);

        let cmd = "PLOCK,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "PLOCK,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_disable() {
        let cmd = "plock,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);

        let cmd = "PLOCK,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "PLOCK,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_status() {
        let cmd = "plock,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);

        let cmd = "PLOCK,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn parse_case_insensitive_command_reading() {
        let cmd = "r".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);

        let cmd = "R".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep;
        assert_eq!(cmd.get_command_string(), "SLEEP");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_sleep() {
        let cmd = "Sleep".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);

        let cmd = "SLEEP".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "STATUS");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_status() {
        let cmd = "status".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);

        let cmd = "STATUS".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);
    }

    #[test]
    fn build_command_temperature_compensation() {
        let cmd = TemperatureCompensation(19.5);
        assert_eq!(cmd.get_command_string(), "T,19.500");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_temperature_compensation() {
        let cmd = "t,0".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(0_f64));

        let cmd = "T,10.5".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(10.5));
    }

    #[test]
    fn build_command_temperature_compensation_value() {
        let cmd = CompensatedTemperatureValue;
        assert_eq!(cmd.get_command_string(), "T,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_temperature_compensation_value() {
        let cmd = "t,?".parse::<CompensatedTemperatureValue>().unwrap();
        assert_eq!(cmd, CompensatedTemperatureValue);

        let cmd = "T,?".parse::<CompensatedTemperatureValue>().unwrap();
        assert_eq!(cmd, CompensatedTemperatureValue);
    }
}
