use std::sync::{LazyLock, Mutex};
use std::time::Duration;

pub(crate) static mut TIMERS: LazyLock<Mutex<Vec<Duration>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub(crate) fn create(hours: i32, minutes: i32, seconds: i32) {
    let duration = Duration::from_secs(((hours * 3600) + (minutes * 60) + seconds) as u64);
    #[allow(static_mut_refs)]
    let mut timers = unsafe { TIMERS.lock().unwrap() };
    timers.push(duration);
    crate::APP_HANDLE.get().unwrap().upgrade().unwrap().set_timers(slint::ModelRc::from(timers.iter().map(|d| d.as_secs() as i32).collect::<Vec<i32>>().as_slice()));
}

pub(crate) fn close() {
    std::process::exit(0);
}