//! Parses I2C responses from the EC EZO Chip.
//!
//! Code modified from "Federico Mena Quintero <federico@gnome.org>"'s original.
use std::fmt;
use std::result;
use std::str::FromStr;

use errors::ErrorKind;
use failure::{Error, ResultExt};

pub use ezo_common::response::{
    DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus, ProtocolLockStatus,
    ResponseStatus, RestartReason,
};

pub type Result<T> = result::Result<T, Error>;

/// Calibration status of the EC EZO chip.
#[derive(Copy, Clone, PartialEq)]
pub enum CalibrationStatus {
    OnePoint,
    TwoPoint,
    NotCalibrated,
}

impl CalibrationStatus {
    /// Parses the result of the "Cal,?" command to query the device's
    /// calibration status.  Returns ...
    pub fn parse(response: &str) -> Result<CalibrationStatus> {
        if response.starts_with("?CAL,") {
            let rest = response.get(5..).unwrap();
            let mut split = rest.split(',');

            let _calibration = match split.next() {
                Some("2") => Ok(CalibrationStatus::TwoPoint),
                Some("1") => Ok(CalibrationStatus::OnePoint),
                Some("0") => Ok(CalibrationStatus::NotCalibrated),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _calibration,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::OnePoint => write!(f, "?CAL,1"),
            CalibrationStatus::TwoPoint => write!(f, "?CAL,2"),
            CalibrationStatus::NotCalibrated => write!(f, "?CAL,0"),
        }
    }
}

impl fmt::Display for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::OnePoint => write!(f, "one-point"),
            CalibrationStatus::TwoPoint => write!(f, "two-point"),
            CalibrationStatus::NotCalibrated => write!(f, "none"),
        }
    }
}

/// Current temperature value used for sensor-reading compensation.
#[derive(Copy, Clone, PartialEq)]
pub struct CompensationValue(pub f64);

impl CompensationValue {
    /// Parses the result of the "T,?" command to get the device's
    /// temperature compensation value.
    pub fn parse(response: &str) -> Result<CompensationValue> {
        if response.starts_with("?T,") {
            let rest = response.get(3..).unwrap();
            let val = f64::from_str(rest).context(ErrorKind::ResponseParse)?;
            Ok(CompensationValue(val))
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for CompensationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?T,{:.*}", 3, self.0)
    }
}

impl fmt::Display for CompensationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

/// The probe-type of the conductivity sensor.
#[derive(Copy, Clone, PartialEq)]
pub enum ProbeType {
    PointOne,
    One,
    Ten,
}

impl ProbeType {
    /// Parses the result of the "Cal,?" command to query the device's
    /// calibration status.  Returns ...
    pub fn parse(response: &str) -> Result<ProbeType> {
        if response.starts_with("?K,") {
            let rest = response.get(3..).unwrap();
            let mut split = rest.split(',');

            let _calibration = match split.next() {
                Some("0.1") => Ok(ProbeType::PointOne),
                Some("1.0") => Ok(ProbeType::One),
                Some("10.0") => Ok(ProbeType::Ten),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _calibration,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for ProbeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProbeType::PointOne => write!(f, "?K,0.1"),
            ProbeType::One => write!(f, "?K,1.0"),
            ProbeType::Ten => write!(f, "?K,10.0"),
        }
    }
}

impl fmt::Display for ProbeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProbeType::PointOne => write!(f, "0.1"),
            ProbeType::One => write!(f, "1.0"),
            ProbeType::Ten => write!(f, "10.0"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParameterStatus {
    On,
    Off,
}

/// Current configuration of which sensing metrics appear in the output string.
#[derive(Copy, Clone, PartialEq)]
pub struct OutputStringStatus {
    pub electric_conductivity: ParameterStatus,
    pub total_dissolved_solids: ParameterStatus,
    pub salinity: ParameterStatus,
    pub specific_gravity: ParameterStatus,
}

impl OutputStringStatus {
    pub fn new() -> OutputStringStatus {
        OutputStringStatus {
            electric_conductivity: ParameterStatus::Off,
            total_dissolved_solids: ParameterStatus::Off,
            salinity: ParameterStatus::Off,
            specific_gravity: ParameterStatus::Off,
        }
    }

    pub fn parse(response: &str) -> Result<OutputStringStatus> {
        if response.starts_with("?O,") {
            let rest = response.get(3..).unwrap();
            let mut split = rest.split(',');

            let mut _output = OutputStringStatus::new();

            let _first = match split.next() {
                Some("EC") => _output.electric_conductivity = ParameterStatus::On,

                Some("TDS") => _output.total_dissolved_solids = ParameterStatus::On,

                Some("S") => _output.salinity = ParameterStatus::On,

                Some("SG") => _output.specific_gravity = ParameterStatus::On,

                Some("No output") | None => (),

                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            let _second = match split.next() {
                Some("TDS") => _output.total_dissolved_solids = ParameterStatus::On,

                Some("S") => _output.salinity = ParameterStatus::On,

                Some("SG") => _output.specific_gravity = ParameterStatus::On,

                None => (),

                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            let _third = match split.next() {
                Some("S") => _output.salinity = ParameterStatus::On,

                Some("SG") => _output.specific_gravity = ParameterStatus::On,

                None => (),

                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            let _fourth = match split.next() {
                Some("SG") => _output.specific_gravity = ParameterStatus::On,

                None => (),

                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            };

            Ok(_output)
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }

    pub fn to_string(&self) -> String {
        let mut _out: Vec<&str> = Vec::new();

        if self.electric_conductivity == ParameterStatus::On {
            _out.push("EC");
        }
        if self.total_dissolved_solids == ParameterStatus::On {
            _out.push("TDS");
        }
        if self.salinity == ParameterStatus::On {
            _out.push("S");
        }
        if self.specific_gravity == ParameterStatus::On {
            _out.push("SG");
        }
        match _out.len() {
            1...4 => _out.join(","),
            0 | _ => "No output".to_string(),
        }
    }
}

impl fmt::Debug for OutputStringStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?O,{}", self.to_string())
    }
}

impl fmt::Display for OutputStringStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProbeMetric {
    ElectricConductivity(f64),
    TotalDissolvedSolids(f64),
    Salinity(f64),
    SpecificGravity(f64),
}

/// Sample reading, can include from `None` to `FourParameters`.
#[derive(Copy, Clone, PartialEq)]
pub enum ProbeReading {
    None,
    OneParameter(f64),
    TwoParameters(f64, f64),
    ThreeParameters(f64, f64, f64),
    FourParameters(f64, f64, f64, f64),
}

impl ProbeReading {
    pub fn parse(response: &str) -> Result<ProbeReading> {
        let mut split = response.split(",");

        let _one = if let Some(reading) = split.next() {
            f64::from_str(reading).context(ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::None);
        };

        let _two = if let Some(reading) = split.next() {
            f64::from_str(reading).context(ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::OneParameter(_one));
        };

        let _three = if let Some(reading) = split.next() {
            f64::from_str(reading).context(ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::TwoParameters(_one, _two));
        };

        let _four = if let Some(reading) = split.next() {
            f64::from_str(reading).context(ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::ThreeParameters(_one, _two, _three));
        };

        if let Some(_) = split.next() {
            return Err(ErrorKind::ResponseParse.into());
        };

        Ok(ProbeReading::FourParameters(_one, _two, _three, _four))
    }
}

impl fmt::Debug for ProbeReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ProbeReading::None => write!(f, "none"),
            &ProbeReading::OneParameter(a) => write!(f, "{}", a),
            &ProbeReading::TwoParameters(a, b) => write!(f, "{},{}", a, b),
            &ProbeReading::ThreeParameters(a, b, c) => write!(f, "{},{},{}", a, b, c),
            &ProbeReading::FourParameters(a, b, c, d) => write!(f, "{},{},{},{}", a, b, c, d),
        }
    }
}

impl fmt::Display for ProbeReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_calibration_status() {
        let response = "?CAL,1";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::OnePoint
        );

        let response = "?CAL,2";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::TwoPoint
        );

        let response = "?CAL,0";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::NotCalibrated
        );
    }

    #[test]
    fn parsing_invalid_calibration_status_yields_error() {
        let response = "";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,2.";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,-1";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,4";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,b";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,1,";
        assert!(CalibrationStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_probe_type_status() {
        let response = "?K,0.1";
        assert_eq!(ProbeType::parse(&response).unwrap(), ProbeType::PointOne);

        let response = "?K,1.0";
        assert_eq!(ProbeType::parse(&response).unwrap(), ProbeType::One);

        let response = "?K,10.0";
        assert_eq!(ProbeType::parse(&response).unwrap(), ProbeType::Ten);
    }

    #[test]
    fn parsing_invalid_probe_type_status_yields_error() {
        let response = "";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,2.";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,-1";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,4";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,b";
        assert!(ProbeType::parse(&response).is_err());

        let response = "?K,1,";
        assert!(ProbeType::parse(&response).is_err());
    }

    #[test]
    fn parses_sensor_reading_single_parameter() {
        let response = "0";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::OneParameter(0.000)
        );

        let response = "12.5";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::OneParameter(12.500)
        );

        let response = "14.0";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::OneParameter(14.000)
        );
    }

    #[test]
    fn parsing_invalid_sensor_reading_single_parameter_yields_error() {
        let response = "";
        assert!(ProbeReading::parse(response).is_err());

        let response = "-x";
        assert!(ProbeReading::parse(response).is_err());

        let response = "0_5";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5.5";
        assert!(ProbeReading::parse(response).is_err());

        let response = "14.1b";
        assert!(ProbeReading::parse(response).is_err());
    }

    #[test]
    fn parses_sensor_reading_two_parameters() {
        let response = "0,000";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::TwoParameters(0.000, 0.000)
        );

        let response = "12.500,0.000";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::TwoParameters(12.500, 0.0)
        );

        let response = "14.000,434.050";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::TwoParameters(14.000, 434.050)
        );
    }

    #[test]
    fn parsing_invalid_sensor_reading_two_parameters_yields_error() {
        let response = ",";
        assert!(ProbeReading::parse(response).is_err());

        let response = "-x,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "5.000,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5.5,6";
        assert!(ProbeReading::parse(response).is_err());

        let response = "14.1,b";
        assert!(ProbeReading::parse(response).is_err());
    }

    #[test]
    fn parses_sensor_reading_three_parameters() {
        let response = "0,0,0";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::ThreeParameters(0.0, 0.0, 0.0)
        );

        let response = "12.500,0.000,1423";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::ThreeParameters(12.5, 0.0, 1423.0)
        );

        let response = "14.000,434.050,0.998";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::ThreeParameters(14.0, 434.05, 0.998)
        );
    }

    #[test]
    fn parsing_invalid_sensor_reading_three_parameters_yields_error() {
        let response = ",,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "1,0,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "1,0,-x";
        assert!(ProbeReading::parse(response).is_err());

        let response = ",,5.000";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5,6,b";
        assert!(ProbeReading::parse(response).is_err());

        let response = "105,6,6.5.5";
        assert!(ProbeReading::parse(response).is_err());
    }

    #[test]
    fn parses_output_string_status() {
        let response = "?O,EC";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::On,
                total_dissolved_solids: ParameterStatus::Off,
                salinity: ParameterStatus::Off,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,EC,TDS,S,SG";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::On,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::On,
            }
        );

        let response = "?O,EC,TDS,S";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::On,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,EC,TDS";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::On,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::Off,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,TDS,S,SG";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::On,
            }
        );

        let response = "?O,TDS,S";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,TDS";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::On,
                salinity: ParameterStatus::Off,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,S,SG";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::Off,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::On,
            }
        );

        let response = "?O,S";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::Off,
                salinity: ParameterStatus::On,
                specific_gravity: ParameterStatus::Off,
            }
        );

        let response = "?O,SG";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::Off,
                salinity: ParameterStatus::Off,
                specific_gravity: ParameterStatus::On,
            }
        );

        let response = "?O,No output";
        assert_eq!(
            OutputStringStatus::parse(response).unwrap(),
            OutputStringStatus {
                electric_conductivity: ParameterStatus::Off,
                total_dissolved_solids: ParameterStatus::Off,
                salinity: ParameterStatus::Off,
                specific_gravity: ParameterStatus::Off,
            }
        );
    }

    #[test]
    fn writes_output_string_status_as_string() {
        let response = "?O,EC";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,EC,TDS,S,SG";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,EC,TDS,S";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,EC,TDS";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,TDS,S,SG";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,TDS,S";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,TDS";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,S,SG";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,S";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,SG";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());

        let response = "?O,No output";
        let output_state = OutputStringStatus::parse(response).unwrap();
        assert_eq!(output_state.to_string(), response.get(3..).unwrap());
    }

    #[test]
    fn parsing_invalid_output_string_status_yields_error() {
        let response = "?O,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,,,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,,,,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,a,b,c,d";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,ECB";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,EC,TDS,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,EC,S,TDS";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,EC,,TDS";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,EC,TDS,S,SG,";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,EC,TDS,S,SG,X";
        assert!(OutputStringStatus::parse(response).is_err());

        let response = "?O,SG,S,TDS,EC";
        assert!(OutputStringStatus::parse(response).is_err());
    }

    #[test]
    fn parses_sensor_reading_four_parameters() {
        let response = "0,0,0,0";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::FourParameters(0.0, 0.0, 0.0, 0.0)
        );

        let response = "12.500,0.000,1423,1.004";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::FourParameters(12.5, 0.0, 1423.0, 1.004)
        );

        let response = "14.000,434.050,12,1234";
        assert_eq!(
            ProbeReading::parse(response).unwrap(),
            ProbeReading::FourParameters(14.0, 434.05, 12.0, 1234.0)
        );
    }

    #[test]
    fn parsing_invalid_sensor_reading_four_parameters_yields_error() {
        let response = ",,,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "1,0,1,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "1,0,1,-x";
        assert!(ProbeReading::parse(response).is_err());

        let response = ",,,5.000";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5,6,7,6.5.5";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5,6,7,6.5,";
        assert!(ProbeReading::parse(response).is_err());

        let response = "10.5,6,7,6.5,4";
        assert!(ProbeReading::parse(response).is_err());
    }

    #[test]
    fn parses_temperature_compensation_value() {
        let response = "?T,14.56";
        assert_eq!(
            CompensationValue::parse(response).unwrap(),
            CompensationValue(14.56)
        );
    }

    #[test]
    fn parsing_invalid_temperature_compensation_value_yields_error() {
        let response = "";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,X.00";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,1.2,43";
        assert!(CompensationValue::parse(response).is_err());
    }
}
