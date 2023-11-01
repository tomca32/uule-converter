use std::fmt::Display;

type Uule = String;

#[derive(Debug, PartialEq)]
pub struct UuleData {
    role: i32,
    producer: i32,
    provenance: i32,
    timestamp: i64,
    lat: f64,
    long: f64,
    radius: i32,
}

impl Display for UuleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "role:{role}
producer:{producer}
provenance:{provenance}
timestamp:{timestamp}
latlng{{
latitude_e7:{lat}
longitude_e7:{long}
}}
radius:{radius}", role=self.role, producer=self.producer, provenance=self.provenance, timestamp=self.timestamp, lat=self.lat_as_e7(), long=self.long_as_e7(), radius=self.radius)
    }
}

impl UuleData {
    pub fn encode(&self) -> Uule {
        format!("a+{}", base64_url::encode(&self.to_string()))
    }

    pub fn decode(input: &str) -> Option<UuleData> {
        let input = input.trim_start_matches("a+");
        let decoded = base64_url::decode(input).unwrap();
        let decoded = String::from_utf8(decoded).unwrap();
        let mut lines = decoded.lines();
        let line = lines.next()?;
        if !line.starts_with("role:") {
            return None;
        }
        let role = line.split(':').nth(1)?.parse::<i32>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("producer:") {
            return None;
        }
        let producer = line.split(':').nth(1)?.parse::<i32>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("provenance:") {
            return None;
        }
        let provenance = line.split(':').nth(1)?.parse::<i32>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("timestamp:") {
            return None;
        }
        let timestamp = line.split(':').nth(1)?.parse::<i64>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("latlng{") {
            return None;
        }
        let line = lines.next()?;
        if !line.starts_with("latitude_e7:") {
            return None;
        }
        let lat = line.split(':').nth(1)?.parse::<i64>().ok()?;

        let line = lines.next()?;
        if !line.starts_with("longitude_e7:") {
            return None;
        }
        let long = line.split(':').nth(1)?.parse::<i64>().ok()?;

        let line = lines.next()?;
        if !line.starts_with('}') {
            return None;
        }

        let line = lines.next()?;
        if !line.starts_with("radius:") {
            return None;
        }
        let radius = line.split(':').nth(1)?.parse::<i32>().ok()?;
        Some(UuleData {
            role,
            producer,
            provenance,
            timestamp,
            lat: Self::normalize_latlong(lat),
            long: Self::normalize_latlong(long),
            radius,
        })
    }

    pub fn lat_as_e7(&self) -> i64 {
        (self.lat * 10_000_000.0).round() as i64
    }

    pub fn long_as_e7(&self) -> i64 {
        (self.long * 10_000_000.0).round() as i64
    }

    pub fn normalize_latlong(input: i64) -> f64 {
        input as f64 / 10_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Uule, UuleData};

    const ENCODED_UULE: &str = "a+cm9sZToxCnByb2R1Y2VyOjEyCnByb3ZlbmFuY2U6Ngp0aW1lc3RhbXA6MTU5MTUyMTI0OTAzNDAwMApsYXRsbmd7CmxhdGl0dWRlX2U3OjM3NDIxMDAwMApsb25naXR1ZGVfZTc6LTEyMjA4NDAwMAp9CnJhZGl1czotMQ";
        const TEST_DATA: UuleData = UuleData {
            role: 1,
            producer: 12,
            provenance: 6,
            timestamp: 1591521249034000,
            lat: 37.4210000,  // 374210000
            long: -12.20840000,
            radius: -1,
        };

    #[test]
    fn test_encoding() {
        let encoded: Uule = TEST_DATA.encode();
        assert_eq!(encoded, ENCODED_UULE);
    }

    #[test]
    fn test_decoding() {
        let decoded: UuleData = UuleData::decode(ENCODED_UULE).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }


}
