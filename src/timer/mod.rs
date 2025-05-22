use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    pub start: Instant,
    pub duration: Duration,
}

pub static TIMERS: LazyLock<Mutex<Vec<Timer>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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