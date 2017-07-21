use errors::*;

pub struct CalibrationStatus;

impl CalibrationStatus {
    pub fn parse(response: &str) -> Result<CalibrationStatus> {
        unimplemented!();
    }
}

pub struct CompensationValue;

impl CompensationValue {
    pub fn parse(response: &str) -> Result<CompensationValue> {
        unimplemented!();
    }
}
pub struct DeviceInfo;

impl DeviceInfo {
    pub fn parse(response: &str) -> Result<DeviceInfo> {
        unimplemented!();
    }
}
pub struct DeviceStatus;

impl DeviceStatus {
    pub fn parse(response: &str) -> Result<DeviceStatus> {
        unimplemented!();
    }
}
pub struct Exported;

impl Exported {
    pub fn parse(response: &str) -> Result<Exported> {
        unimplemented!();
    }
}
pub struct ExportedInfo;

impl ExportedInfo {
    pub fn parse(response: &str) -> Result<ExportedInfo> {
        unimplemented!();
    }
}
pub struct ProbeTypeStatus;

impl ProbeTypeStatus {
    pub fn parse(response: &str) -> Result<ProbeTypeStatus> {
        unimplemented!();
    }
}
pub struct LedStatus;

impl LedStatus {
    pub fn parse(response: &str) -> Result<LedStatus> {
        unimplemented!();
    }
}
pub struct OutputStatus;

impl OutputStatus {
    pub fn parse(response: &str) -> Result<OutputStatus> {
        unimplemented!();
    }
}
pub struct ProtocolLockStatus;

impl ProtocolLockStatus {
    pub fn parse(response: &str) -> Result<ProtocolLockStatus> {
        unimplemented!();
    }
}
pub struct SensorReading;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProbeReading {
    OneParameter(f64),
    TwoParameters(f64, f64),
    ThreeParameters(f64, f64, f64),
    FourParameters(f64, f64, f64, f64),
}

impl ProbeReading {
    pub fn parse(response: &str) -> Result<ProbeReading> {
        let mut split = response.split(",");

        let _one = if let Some(reading) = split.next() {
            f64::from_str(reading).chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Err(ErrorKind::ResponseParse.into());
        };

        let _two = if let Some(reading) = split.next() {
            f64::from_str(reading).chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::OneParameter(_one));
        };

        let _three = if let Some(reading) = split.next() {
            f64::from_str(reading).chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::TwoParameters(_one, _two));
        };

        let _four = if let Some(reading) = split.next() {
            f64::from_str(reading).chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Ok(ProbeReading::ThreeParameters(_one, _two, _three));
        };

        if let Some(_) = split.next() {
            return Err(ErrorKind::ResponseParse.into());
        };

        Ok(ProbeReading::FourParameters(_one, _two, _three, _four))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_calibration_status() {
        let response = "?CAL,1";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::OnePoint);

        let response = "?CAL,2";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::TwoPoint);

        let response = "?CAL,0";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::NotCalibrated);
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
    fn parses_data_export_string() {
        let response = "123456789012";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("123456789012".to_string()));

        let response = "myresponse";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("myresponse".to_string()));

        let response = "*DONE";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::Done);
    }

    #[test]
    fn parsing_invalid_export_string_yields_error() {
        let response = "*";
        assert!(Exported::parse(response).is_err());

        let response = "*DONE*";
        assert!(Exported::parse(response).is_err());

        let response = "**DONE";
        assert!(Exported::parse(response).is_err());

        let response = "12345678901234567890";
        assert!(Exported::parse(response).is_err());
    }

    #[test]
    fn parses_export_info() {
        let response = "?EXPORT,0,0";
        assert_eq!(ExportedInfo::parse(response).unwrap(),
                   ExportedInfo { lines: 0, total_bytes: 0 } );
    }

    #[test]
    fn parsing_invalid_export_info_yields_error() {
        let response = "?EXPORT,11,120,10";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "?EXPORT,1012";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "10,*DON";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "12,";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "";
        assert!(ExportedInfo::parse(response).is_err());
    }

    #[test]
    fn parses_device_information() {
        let response = "?I,EC,2.10";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "EC".to_string(),
                       firmware: "2.10".to_string(),
                   } );

        let response = "?I,,";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "".to_string(),
                       firmware: "".to_string(),
                   } );

    }

    #[test]
    fn parsing_invalid_device_info_yields_error() {
        let response = "";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,a,b,c";
        assert!(DeviceInfo::parse(response).is_err());
    }

    #[test]
    fn parses_led_status() {
        let response = "?L,1";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::On);

        let response = "?L,0";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::Off);
    }

    #[test]
    fn parsing_invalid_led_status_yields_error() {
        let response = "";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,b";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,17";
        assert!(LedStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_probe_type_status() {
        let response = "?K,0.1";
        assert_eq!(ProbeType::parse(&response).unwrap(),
                   ProbeType::PointOne);

        let response = "?K,1.0";
        assert_eq!(ProbeType::parse(&response).unwrap(),
                   ProbeType::One);

        let response = "?K,10.0";
        assert_eq!(ProbeType::parse(&response).unwrap(),
                   ProbeType::Ten);
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
    fn parses_protocol_lock_status() {
        let response = "?PLOCK,1";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::On);

        let response = "?PLOCK,0";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::Off);
    }

    #[test]
    fn parsing_invalid_protocol_lock_status_yields_error() {
        let response = "";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,57";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b,1";
        assert!(ProtocolLockStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_sensor_reading_single_parameter() {
        let response = "0";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::OneParameter(0.000));

        let response = "12.5";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::OneParameter(12.500));

        let response = "14.0";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::OneParameter(14.000));
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
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::TwoParameters(0.000, 0.000));

        let response = "12.500,0.000";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::TwoParameters(12.500, 0.0));

        let response = "14.000,434.050";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::TwoParameters(14.000, 434.050));
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
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::ThreeParameters(0.0, 0.0, 0.0));

        let response = "12.500,0.000,1423";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::ThreeParameters(12.5, 0.0, 1423.0));

        let response = "14.000,434.050,0.998";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::ThreeParameters(14.0, 434.05, 0.998));
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
    fn parses_sensor_reading_four_parameters() {
        let response = "0,0,0,0";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::FourParameters(0.0, 0.0, 0.0, 0.0));

        let response = "12.500,0.000,1423,1.004";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::FourParameters(12.5, 0.0, 1423.0, 1.004));

        let response = "14.000,434.050,12,1234";
        assert_eq!(ProbeReading::parse(response).unwrap(),
                   ProbeReading::FourParameters(14.0, 434.05, 12.0, 1234.0));
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
    fn parses_device_status() {
        let response = "?STATUS,P,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::PoweredOff,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,S,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::SoftwareReset,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,B,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::BrownOut,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,W,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Watchdog,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,U,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Unknown,
                       vcc_voltage: 1.5,
                   });
    }

    #[test]
    fn parsing_invalid_device_status_yields_error() {
        let response = "";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?STATUS,X,";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?STATUS,P,1.5,";
        assert!(DeviceStatus::parse(response).is_err());
    }

    #[test]
    fn parses_temperature_compensation_value() {
        let response = "?T,14.56";
        assert_eq!(CompensationValue::parse(response).unwrap(),
                   CompensationValue(14.56));
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
