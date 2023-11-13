use base64_url::base64;
use thiserror::Error;

use crate::consts::{UULEV1_ROLE, UULEV1_PRODUCER};

/// type alias for UULEv1 strings
pub type Uulev1 = String;

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
/// use uule_converter::uulev1::Uulev1Error;
/// use base64_url::base64;
///
/// let uule = Uulev1Data::new("Queens County,New York,United States".to_string());
/// assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });
///
/// let encoded = uule.encode();
/// assert_eq!(encoded, "w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz");
///
/// let uule = Uulev1Data::decode(&encoded).unwrap();
/// assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });
///
/// let uule = Uulev1Data::decode("asdf").unwrap_err();
/// assert_eq!(uule, Uulev1Error::InvalidPrefix("asdf".to_string()));
///
/// let uule = Uulev1Data::decode("w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz ").unwrap_err(); // trailing whitespace makes it invalid base64
/// assert_eq!(uule, Uulev1Error::Base64DecodingError { source: base64::DecodeError::InvalidByte(56, 32) });
///
/// let uule = Uulev1Data::decode("w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz").unwrap_err(); // trailing whitespace
/// assert_eq!(uule, Uulev1Error::Base64DecodingError { source: base64::DecodeError::InvalidByte(56, 32) });
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

    pub fn encode(&self) -> Uulev1 {
        let mut name_bytes = self.canonical_name.as_bytes().to_vec();
        let mut bytes: Vec<u8> = vec![8, self.role, 16, self.producer, 34, self.canonical_name.len() as u8];
        bytes.append(&mut name_bytes);
        format!("w+{}", base64_url::encode(&bytes))
    }

    pub fn decode(input: &str) -> Result<Self, Uulev1Error> {
        if !input.starts_with("w+") {
            return Err(Uulev1Error::InvalidPrefix(input.to_string()));
        }
        let input = input.trim_start_matches("w+");
        let bytes = base64_url::decode(input)?;
        let role = bytes[1];
        let producer = bytes[3];
        let name_len = bytes[5] as usize;
        let name = String::from_utf8(bytes[6..6 + name_len].to_vec());
        Ok(Self { role, producer, canonical_name: name? })
    }
}

/// Uulev1Error is an enum that represents the possible errors that can occur when decoding a UULEv1 string.
#[derive(Error, Debug, PartialEq)]
pub enum Uulev1Error {
    /// Invalid prefix. UULEv1 strings must start with 'w+'. Received string is accessible as `error.0`
    #[error("Invalid prefix. UULEv1 strings must start with 'w+'. Received: {0}")]
    InvalidPrefix(String),
    /// Invalid Base64-URL string. Underlying error is accessible as `error.source`
    #[error("Invalid Base64-URL string. Underlying error: {source}")]
    Base64DecodingError {
        #[from] source: base64::DecodeError
    },
    /// Invalid UTF-8 string. Underlying error is accessible as `error.source`
    #[error("Invalid UTF-8 string. Underlying error: {source}")]
    Utf8DecodingError {
        #[from] source: std::string::FromUtf8Error
    },
}