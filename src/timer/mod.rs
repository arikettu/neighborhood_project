use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    pub start: Instant,
    pub duration: Duration,
}

lazy_static::lazy_static!(
    pub static ref TIMERS: Mutex<Vec<Timer>> = Mutex::new(Vec::new());
);

pub fn start(duration: Duration, callback: fn()) -> Timer {
    let start_time = Instant::now();
    
    thread::spawn(move || {
        thread::sleep(duration);
        callback();
    });
    
    Timer {
        start: start_time,
        duration,
    }
}