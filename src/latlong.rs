/// Convert a latitude or longitude to an e7 integer representation.
/// UULE encodes these values as integers raised to the 7th power.
///
/// # Examples
///
/// ```
/// use uule_converter::latlong::latlong_to_e7;
/// assert_eq!(latlong_to_e7(37.4210000), 374210000);
/// assert_eq!(latlong_to_e7(-12.20840000), -122084000);
/// assert_eq!(latlong_to_e7(40.730610), 407306100);
/// assert_eq!(latlong_to_e7(-73.9352420), -739352420);
/// ```
pub fn latlong_to_e7(input: f64) -> i64 {
    (input * 10_000_000.0).round() as i64
}

/// Convert an e7 integer representation to a float representation of latitude or longitude.
/// UULE encodes these values as integers raised to the 7th power.
///
/// # Examples
///
/// ```
/// use uule_converter::latlong::latlong_from_e7;
/// assert_eq!(latlong_from_e7(374210000), 37.4210000);
/// assert_eq!(latlong_from_e7(-122084000), -12.2084000);
/// assert_eq!(latlong_from_e7(407306100), 40.730610);
/// assert_eq!(latlong_from_e7(-739352420), -73.9352420);
/// ```
pub fn latlong_from_e7(input: i64) -> f64 {
    input as f64 / 10_000_000.0
}