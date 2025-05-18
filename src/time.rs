use std::{default, time::{Duration, Instant}};



pub struct Time{
    start: Instant,
    time_from: Duration,
    append: Duration,
    time_string: String,
}

impl Default for Time {
    fn default() -> Self {
        Self { 
            start: Instant::now(), 
            time_from: Default::default(), 
            append: Default::default(), 
            time_string: Default::default() 
        }
    }
}

impl Time {
    fn new(start: Instant) -> Self{
        Self {
            start: start,
            .. Default::default()
        }
    }

    fn to_string(&self) -> String {
        format_duration(self.time_from + self.append)
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
