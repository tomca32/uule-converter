use crate::consts::{UULEV1_ROLE, UULEV1_PRODUCER};

/// Uulev1Data is a struct that represents the data encoded in a UULEv1 string.
/// The meaning of most of the fields is unclear and is not documented.
/// Read more about this at: <https://valentin.app/uule.html>
///
/// Role - Some sort of role of the data. Default value is 2 just based on the linked post above
///
/// Producer - Some sort of producer of the data. Default value is 16 again based on the linked post above
///
/// Canoncial Name - The name of the location encoded in the UULEv1 string
///
/// # Examples
/// ```
/// use uule_converter::uulev1::Uulev1Data;
///
/// let uule = Uulev1Data::new("Queens County,New York,United States".to_string());
/// assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });
///
/// let encoded = uule.encode();
/// assert_eq!(encoded, "w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz");
///
/// let uule = Uulev1Data::decode(&encoded).unwrap();
/// assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Uulev1Data {
    pub role: u8,
    pub producer: u8,
    pub canonical_name: String,
}

impl Uulev1Data {
    pub fn new(place: String) -> Self {
        Self { role: UULEV1_ROLE, producer: UULEV1_PRODUCER, canonical_name: place }
    }

    pub fn encode(&self) -> String {
        let mut name_bytes = self.canonical_name.as_bytes().to_vec();
        let mut bytes: Vec<u8> = vec![8, self.role, 16, self.producer, 34, self.canonical_name.len() as u8];
        bytes.append(&mut name_bytes);
        format!("w+{}", base64_url::encode(&bytes))
    }

    pub fn decode(input: &str) -> Option<Self> {
        let input = input.trim_start_matches("w+");
        let bytes = base64_url::decode(input).ok()?;
        let role = bytes[1];
        let producer = bytes[3];
        let name_len = bytes[5] as usize;
        let name = String::from_utf8(bytes[6..6 + name_len].to_vec()).map_err(|_| "Invalid UTF-8".to_string()).ok()?;
        Some(Self { role, producer, canonical_name: name })
    }
}
