use std::{fmt::Display, time::SystemTime, str::{FromStr, Lines}};
use base64_url::base64;
use thiserror::Error;

use crate::{latlong, consts::{USER_SPECIFIED_FOR_REQUEST, LOGGED_IN_USER_SPECIFIED}};

/// type alias for UULEv2 strings
pub type Uulev2 = String;

/// Uulev2Data is a struct that represents the data encoded in a UULEv2 string.
/// The meaning of most of the fields is unclear and is not documented.
/// Read more about this at: <https://valentin.app/uule.html>
///
/// Probably the only interesting fields are lat, long, and possibly radius.
///
/// Role - Some sort of role of the data. Default value is 1 meaning USER_SPECIFIED_FOR_REQUEST
///
/// Producer - Some sort of producer of the data. Default value is 12 meaning LOGGED_IN_USER_SPECIFIED
///
/// Provenance - Unknown. Default value is 0
///
/// Timestamp - Unix timestamp in milliseconds. Default value is the current time
///
/// Lat - Latitude in degrees. Default value is 0.0
///
/// Long - Longitude in degrees. Default value is 0.0
///
/// Radius - Supposedly the radius of the location in meters. Some experimentation showed that this is actually meters times 620. Default value of 1 means exact location of latlong.
///
/// # Examples
///
/// ```
/// use uule_converter::uulev2::Uulev2Data;
/// use uule_converter::uulev2::Uulev2Error;
///
/// // Constructing a Uulev2Data object with a builder like pattern
/// let uule = Uulev2Data::default().with_lat(37.4210000).with_long(-12.2084000).with_radius(6200);
///
/// /// Alternatively construct it yourself
/// let uule = Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 };
///
/// // encoding the Uulev2Data object to a UULEv2 string
///
/// let encoded = uule.encode();
/// assert_eq!(encoded, "a+cm9sZToxCnByb2R1Y2VyOjEyCnByb3ZlbmFuY2U6Ngp0aW1lc3RhbXA6MTU5MTUyMTI0OTAzNDAwMApsYXRsbmd7CmxhdGl0dWRlX2U3OjM3NDIxMDAwMApsb25naXR1ZGVfZTc6LTEyMjA4NDAwMAp9CnJhZGl1czotMQ");
///
/// // decoding the UULEv2 string to a Uulev2Data object
/// let uule = Uulev2Data::decode(&encoded).unwrap();
///
/// // The decoded object is the same as the original object
/// assert_eq!(uule, Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 });
///
/// let error = Uulev2Data::decode("asdf").unwrap_err();
/// assert_eq!(error, Uulev2Error::InvalidPrefix("asdf".to_string()));
///
///
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Uulev2Data {
    pub role: u8,
    pub producer: u8,
    pub provenance: i32,
    pub timestamp: u128,
    pub lat: f64,
    pub long: f64,
    pub radius: i32,
}

impl Default for Uulev2Data {
    fn default() -> Self {
        Self { role: USER_SPECIFIED_FOR_REQUEST, producer: LOGGED_IN_USER_SPECIFIED, provenance: 0, timestamp: generate_timestamp(), lat: 0.0, long: 0.0, radius: -1 }
    }
}

fn generate_timestamp() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Unable to generate timestamp").as_millis()
}

/// The Display implmentation for Uulev2Data represents data in its intermediate String form before it is base64 encoded.
///
/// # Examples
///
/// ```
/// use uule_converter::uulev2::Uulev2Data;
/// let uule = Uulev2Data{ role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 };
/// assert_eq!(uule.to_string(), "role:1
/// producer:12
/// provenance:6
/// timestamp:1591521249034000
/// latlng{
/// latitude_e7:374210000
/// longitude_e7:-122084000
/// }
/// radius:-1");
/// ```
impl Display for Uulev2Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat = latlong::latlong_to_e7(self.lat);
        let long = latlong::latlong_to_e7(self.long);
        write!(f, "role:{role}
producer:{producer}
provenance:{provenance}
timestamp:{timestamp}
latlng{{
latitude_e7:{lat}
longitude_e7:{long}
}}
radius:{radius}", role=self.role, producer=self.producer, provenance=self.provenance, timestamp=self.timestamp, lat=lat, long=long, radius=self.radius)
    }
}

impl Uulev2Data {
    pub fn with_lat(mut self, lat: f64) -> Self {
        self.lat = lat;
        self
    }

    pub fn with_long(mut self, long: f64) -> Self {
        self.long = long;
        self
    }

    pub fn with_radius(mut self, radius: i32) -> Self {
        self.radius = radius;
        self
    }

    pub fn encode(&self) -> Uulev2 {
        format!("a+{}", base64_url::encode(&self.to_string()))
    }

    pub fn decode(input: &str) -> Result<Uulev2Data, Uulev2Error> {
        if !input.starts_with("a+") {
            return Err(Uulev2Error::InvalidPrefix(input.to_string()));
        }
        let input = input.trim_start_matches("a+");
        let decoded = base64_url::decode(input).unwrap();
        let decoded = String::from_utf8(decoded).unwrap();
        let mut lines = decoded.lines();

        let role: u8 = Uulev2Data::parse_int_line(lines.next(), "role")?;
        let producer: u8 = Uulev2Data::parse_int_line(lines.next(), "producer")?;
        let provenance: i32 = Uulev2Data::parse_int_line(lines.next(), "provenance")?;
        let timestamp: u128 = Uulev2Data::parse_int_line(lines.next(), "timestamp")?;
        let (lat, long): (f64, f64) = Uulev2Data::parse_lat_long(&mut lines)?;
        let radius: i32 = Uulev2Data::parse_int_line(lines.next(), "radius")?;

        Ok(Uulev2Data {
            role,
            producer,
            provenance,
            timestamp,
            lat,
            long,
            radius,
        })
    }

    fn parse_int_line<T>(line: Option<&str>, field: &str) -> Result<T, Uulev2Error> where T: FromStr::<Err=std::num::ParseIntError> {
        Uulev2Data::get_field_value(line, field)?.parse::<T>().map_err(|e| Uulev2Error::InvalidIntegerValue { source: e })
    }

    fn parse_lat_long(lines: &mut Lines) -> Result<(f64, f64), Uulev2Error> {
        let line = lines.next().ok_or_else(|| Uulev2Error::UnexpectedEnd("latlng{".to_string()))?;
        if line != "latlng{" {
            return Err(Uulev2Error::UnexpectedLine{expected: "latlng{".to_string(), actual: line.to_string()});
        }
        let lat: f64 = latlong::latlong_from_e7(Uulev2Data::parse_int_line(lines.next(), "latitude_e7")?);
        let long: f64 = latlong::latlong_from_e7(Uulev2Data::parse_int_line(lines.next(), "longitude_e7")?);
        let line = lines.next().ok_or_else(|| Uulev2Error::UnexpectedEnd("}".to_string()))?;
        if line != "}" {
            return Err(Uulev2Error::UnexpectedLine{expected: "}".to_string(), actual: line.to_string()});
        }

        Ok((lat, long))
    }

    fn get_field_value<'a>(line: Option<&'a str>, field: &str) -> Result<&'a str, Uulev2Error> {
        let line = line.ok_or_else(|| Uulev2Error::UnexpectedEnd(field.to_string()))?;
        if !line.starts_with(field) {
            return Err(Uulev2Error::UnexpectedLine{expected: field.to_string(), actual: line.to_string()});
        }
        line.split(':').nth(1).ok_or_else(|| Uulev2Error::MissingValue(field.to_owned()))
    }
}

/// Uulev2Error is the error type for UULEv2 decoding
#[derive(Clone, Debug, PartialEq, Error)]
pub enum Uulev2Error {
    /// Invalid prefix. UULEv2 strings must start with 'a+'. Received string is accessible as `error.0`
    #[error("Invalid prefix. UULEv2 strings must start with 'a+'. Received: {0}")]
    InvalidPrefix(String),
    /// Invalid Base64-URL string. Underlying error is accessible as `error.source`
    #[error("Invalid Base64-URL string. Underlying error: {source}")]
    Base64DecodingError { source: base64::DecodeError },
    /// Unexpected end of string while decoding. Expected line is accessible as `error.0`
    #[error("Unexpected end of string, expected line {0}")]
    UnexpectedEnd(String),
    /// Unexpected line while decoding. Expected line is accessible as `error.expected` and actual line is accessible as `error.actual`
    #[error("Unexpected line: {actual} - expected: {expected}")]
    UnexpectedLine {
        expected: String,
        actual: String,
    },
    /// Missing value for field. Field name is accessible as `error.0`
    #[error("Missing value for field {0}")]
    MissingValue(String),
    /// Invalid value while passing a supposed integer. Underlying error is accessible as `error.source`
    #[error("Invalid integer value. Underlying error: {source}")]
    InvalidIntegerValue { source: std::num::ParseIntError },
    /// Invalid value while passing a supposed float. Underlying error is accessible as `error.source`
    #[error("Invalid float value. Underlying error: {source}")]
    InvalidFloatValue { source: std::num::ParseFloatError },
}