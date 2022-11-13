use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const HOUR: i32 = 24 * 60;
const MINUTE: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: ((((hours * MINUTE) + minutes) % HOUR) + HOUR) % HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / MINUTE, self.minutes % MINUTE)
    }
}