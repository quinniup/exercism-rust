use std::ops::Add;
use time::{Duration, PrimitiveDateTime as DateTime};

const GIGA_SECOND: i64 = 1000000000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::new(GIGA_SECOND, 0))
}
