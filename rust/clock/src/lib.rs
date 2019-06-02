use std::fmt;

const MINUTES_PER_DAY: i32 = 60 * 24;

fn euc_mod(x: i32, y: i32) -> i32 {
    ((x % y) + y) % y
}


#[derive(Debug, PartialEq)]
pub struct Clock {
    pub minutes: i32,
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = euc_mod(hours * 60 + minutes, MINUTES_PER_DAY);

        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}
