use chrono::{DateTime, Duration, Utc};


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGASECOND: i64 = 1_000_000_000;
    start + Duration::seconds(GIGASECOND)   
}
