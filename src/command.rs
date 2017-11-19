//! I2C Commands for EC EZO Chip.
//!
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    CompensationValue,
    ProbeType,
    OutputStringStatus,
    ProbeReading,
};

use ezo_common::{
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
pub use ezo_common::Command;
pub use ezo_common::command::{
    Baud,
    CalibrationClear,
    DeviceAddress,
    DeviceInformation,
    Export,
    ExportInfo,
    Factory,
    Find,
    Import,
    LedOff,
    LedOn,
    LedState,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockState,
    Status,
    Sleep,
};

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
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
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
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
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
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
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
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
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
    fn parse_invalid_command_calibration_one_point_yields_err() {
        let cmd = "cal,".parse::<CalibrationOnePoint>();
        assert!(cmd.is_err());

        let cmd = "CAL,1a21.43".parse::<CalibrationOnePoint>();
        assert!(cmd.is_err());
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
    fn parse_invalid_command_calibration_high_yields_err() {
        let cmd = "cal,high,".parse::<CalibrationHigh>();
        assert!(cmd.is_err());

        let cmd = "CAL,High,1a21.43".parse::<CalibrationHigh>();
        assert!(cmd.is_err());
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
    fn parse_invalid_command_calibration_low_yields_err() {
        let cmd = "cal,low,".parse::<CalibrationLow>();
        assert!(cmd.is_err());

        let cmd = "CAL,LOW,1a21.43".parse::<CalibrationLow>();
        assert!(cmd.is_err());
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
    fn parse_invalid_command_temperature_compensation_yields_err() {
        let cmd = "T,".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());

        let cmd = "T,$".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());

        let cmd = "T,1a21.43".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());
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
