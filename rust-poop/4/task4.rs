use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl Clock {
	pub fn new(hours: i32, minutes: i32) -> Self {
		let total_minutes = hours * 60 + minutes;
		let valid_minutes = (total_minutes % (24 * 60) + 24 * 60) % (24 * 60);
		
		Clock {
			hours: valid_minutes / 60,
			minutes: valid_minutes % 60,
		}
	}
	
	pub fn add_minutes(&self, minutes: i32) -> Self {
		Clock::new(self.hours, self.minutes + minutes)
	}
	
	pub fn subtract_minutes(&self, minutes: i32) -> Self {
		Clock::new(self.hours, self.minutes - minutes)
	}
}

impl Display for Clock {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{:02}:{:02}", self.hours, self.minutes)
	}
}

