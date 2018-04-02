extern crate chrono;
extern crate num_traits;

use chrono::{DateTime, Utc, Duration};
use num_traits::pow;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(pow(10, 9))
}
