# UULE Converter

This crate encodes and decodes UULEv1 and UULEv2 formats.

Examples:

```rust
use uule_converter::uulev1::Uulev1Data;

let uule = Uulev1Data::new("Queens County,New York,United States".to_string());
let encoded = uule.encode();
assert_eq!(encoded, "w+CAIQICIkUXVlZW5zIENvdW50eSxOZXcgWW9yayxVbml0ZWQgU3RhdGVz");
let uule = Uulev1Data::decode(&encoded).unwrap();
assert_eq!(uule, Uulev1Data { role: 2, producer: 32, canonical_name: "Queens County,New York,United States".to_string() });


use uule_converter::uulev2::Uulev2Data;

let uule = Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 };
let encoded = uule.encode();
assert_eq!(encoded, "a+cm9sZToxCnByb2R1Y2VyOjEyCnByb3ZlbmFuY2U6Ngp0aW1lc3RhbXA6MTU5MTUyMTI0OTAzNDAwMApsYXRsbmd7CmxhdGl0dWRlX2U3OjM3NDIxMDAwMApsb25naXR1ZGVfZTc6LTEyMjA4NDAwMAp9CnJhZGl1czotMQ");
let uule = Uulev2Data::decode(&encoded).unwrap();
assert_eq!(uule, Uulev2Data { role: 1, producer: 12, provenance: 6, timestamp: 1591521249034000, lat: 37.4210000, long: -12.2084000, radius: -1 });
```

Inspired by <https://valentin.app/uule.html>

Prior art:
UULEv1 in Python: <https://github.com/ogun/uule_grabber>
UULEv2 in Ruby: <https://github.com/serpapi/uule_converter>