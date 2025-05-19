use std::{default, time::{Duration, Instant}};


#[derive(Debug, PartialEq, Clone)]
pub struct Time{
    start: Instant,
    time_from: Duration,
    append: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self { 
            start: Instant::now(), 
            time_from: Default::default(), 
            append: Default::default(), 
        }
    }
}

impl Time {
    pub fn new(start: Instant) -> Self{
        Self {
            start: start,
            .. Default::default()
        }
    }

    pub fn from(time: Duration) -> Self{
        Self {
            start: Instant::now(),
            append: time,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        self.time_from = self.start.elapsed();
    }

    // Moves the current time to append, resets current time to 0
    pub fn swap_current(&mut self) {
        self.append = self.append + self.time_from;
        self.new_start();
    }

    // Resets the start time to current time,
    pub fn new_start(&mut self) {
        self.start = Instant::now();
        self.tick();
    }

    pub fn to_string(&self) -> String {
        format_duration(self.time_from + self.append)
    }

    pub fn copy(&mut self, source:&mut Self) {
        source.swap_current();
        self.append = source.append;
        self.new_start();
    }

    pub fn export_time(&self) -> Duration {
        self.append + self.time_from
    }

}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
