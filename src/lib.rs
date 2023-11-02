//! Library for encoding and decoding Google's UULE format.
//!
//! UULEv1 encodes city names while UULEv2 encodes latitude, longitude, and radius, among other meta data.
//!
//! For a list of UULEv1 locations go to <https://developers.google.com/google-ads/api/data/geotargets>
//!
//! Inspired by <https://valentin.app/uule.html>
//!
//! UULEv1 in Python: <https://github.com/ogun/uule_grabber>
//! UULEv2 in Ruby: <https://github.com/serpapi/uule_converter>
//!
//! # Examples
//!
//! ## UULEv1
//! ```
//! use uule_converter::uulev1::Uulev1Data;
//!
//! // Constructing a Uulev1Data object
//! let uule = Uulev1Data::new("Queens County,New York,United States".to_string());
//! // Encoding the Uulev1Data object to a UULEv1 string
//! let encoded = uule.encode();
//! assert_eq!(encoded, "w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz");
//! // Decoding the UULEv1 string to a Uulev1Data object
//! let uule = Uulev1Data::decode(&encoded).unwrap();
//! // The decoded object is the same as the original object
//! assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });
//! ```
//!
//! ## UULEv2
//! ```
//! use uule_converter::uulev2::Uulev2Data;
//!
//! // Constructing a Uulev2Data object with a builder like pattern
//! let uule = Uulev2Data::default().with_lat(37.4210000).with_long(-12.2084000).with_radius(6200);
//! // Alternatively construct it yourself
//! let uule = Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 };
//!
//! // Encoding the Uulev2Data object to a UULEv2 string
//! let encoded = uule.encode();
//! assert_eq!(encoded, "a+cm9sZToxCnByb2R1Y2VyOjEyCnByb3ZlbmFuY2U6Ngp0aW1lc3RhbXA6MTU5MTUyMTI0OTAzNDAwMApsYXRsbmd7CmxhdGl0dWRlX2U3OjM3NDIxMDAwMApsb25naXR1ZGVfZTc6LTEyMjA4NDAwMAp9CnJhZGl1czotMQ");
//!
//! // Decoding the UULEv2 string to a Uulev2Data object
//! let uule = Uulev2Data::decode(&encoded).unwrap();
//! // The decoded object is the same as the original object
//! assert_eq!(uule, Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 });
//! ```

/// Contains the UULEv1 implementation
pub mod uulev1;
/// Contains the UULEv2 implementation
pub mod uulev2;
/// Contains the latitude and longitude conversion functions
pub mod latlong;
/// Contains the constants used in the UULEv1 and UULEv2 implementations
pub mod consts;
