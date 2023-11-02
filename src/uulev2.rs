use std::{fmt::Display, time::SystemTime};
use crate::{latlong, consts::{USER_SPECIFIED_FOR_REQUEST, LOGGED_IN_USER_SPECIFIED}};

type Uulev2 = String;

/// Uulev2Data is a struct that represents the data encoded in a UULEv2 string.
/// The meaning of most of the fields is unclear and is not documented.
/// Read more about this at: https://valentin.app/uule.html
///
/// Probably the only interesting fields are lat, long, and possibly radius.
///
/// Role - Some sort of role of the data. Default value is 1 meaning USER_SPECIFIED_FOR_REQUEST
/// Producer - Some sort of producer of the data. Default value is 12 meaning LOGGED_IN_USER_SPECIFIED
/// Provenance - Unknown. Default value is 0
/// Timestamp - Unix timestamp in milliseconds. Default value is the current time
/// Lat - Latitude in degrees. Default value is 0.0
/// Long - Longitude in degrees. Default value is 0.0
/// Radius - Supposedly the radius of the location in meters. Some experimentation showed that this is actually meters times 620. Default value of 1 means exact location of latlong.
///
/// # Examples
///
/// ```
/// use uule_converter::uulev2::Uulev2Data;
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

    pub fn decode(input: &str) -> Option<Uulev2Data> {
        let input = input.trim_start_matches("a+");
        let decoded = base64_url::decode(input).unwrap();
        let decoded = String::from_utf8(decoded).unwrap();
        let mut lines = decoded.lines();
        let line = lines.next()?;
        if !line.starts_with("role:") {
            return None;
        }
        let role = line.split(':').nth(1)?.parse::<u8>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("producer:") {
            return None;
        }
        let producer = line.split(':').nth(1)?.parse::<u8>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("provenance:") {
            return None;
        }
        let provenance = line.split(':').nth(1)?.parse::<i32>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("timestamp:") {
            return None;
        }
        let timestamp = line.split(':').nth(1)?.parse::<u128>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("latlng{") {
            return None;
        }
        let line = lines.next()?;
        if !line.starts_with("latitude_e7:") {
            return None;
        }
        let lat = line.split(':').nth(1)?.parse::<i64>().ok()?;
        let lat = latlong::latlong_from_e7(lat);

        let line = lines.next()?;
        if !line.starts_with("longitude_e7:") {
            return None;
        }
        let long = line.split(':').nth(1)?.parse::<i64>().ok()?;
        let long = latlong::latlong_from_e7(long);

        let line = lines.next()?;
        if !line.starts_with('}') {
            return None;
        }

        let line = lines.next()?;
        if !line.starts_with("radius:") {
            return None;
        }
        let radius = line.split(':').nth(1)?.parse::<i32>().ok()?;
        Some(Uulev2Data {
            role,
            producer,
            provenance,
            timestamp,
            lat,
            long,
            radius,
        })
    }

}