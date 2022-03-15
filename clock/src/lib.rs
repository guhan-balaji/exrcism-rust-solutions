use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Self {
            hours: 0,
            minutes: 0,
        };
        clock.add_hours(hours).add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let additional_hours = match self.minutes + minutes {
            m if m.is_positive() => (m / 60),
            m if m % 60 == 0 => m / 60,
            m => (m / 60) - 1,
        };

        let hours = self.hours;
        let minutes = match self.minutes + minutes {
            m if m.is_positive() => m % 60,
            m if m % 60 == 0 => 0,
            m => 60 + (m % 60),
        };

        let clock = Self { hours, minutes };
        clock.add_hours(additional_hours)
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        let hours = match self.hours + hours {
            h if (h % 24) == 0 => 0,
            h if h.is_negative() => 24 + (h % 24),
            h => h % 24,
        };
        let minutes = self.minutes;
        Self { hours, minutes }
    }
}
