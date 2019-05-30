use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let nine_millon = 10_i64.pow(9);
    start + Duration::seconds(nine_millon)
}
