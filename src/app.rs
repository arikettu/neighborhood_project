use std::sync::{LazyLock, RwLock};
use std::time::Duration;
use slint::ComponentHandle;

pub(crate) static mut TIMERS: LazyLock<RwLock<Vec<Duration>>> = LazyLock::new(|| RwLock::new(Vec::new()));

pub(crate) fn show() { slint::invoke_from_event_loop(|| crate::APP_HANDLE.get().unwrap().upgrade().unwrap().show().unwrap()).unwrap(); }
pub(crate) fn hide() { slint::invoke_from_event_loop(|| crate::APP_HANDLE.get().unwrap().upgrade().unwrap().hide().unwrap()).unwrap(); }

pub(crate) fn create(hours: i32, minutes: i32, seconds: i32) {
    let duration = Duration::from_secs(((hours * 3600) + (minutes * 60) + seconds) as u64);
    #[allow(static_mut_refs)]
    let mut timers = unsafe { TIMERS.write().unwrap() };
    timers.push(duration);
    crate::APP_HANDLE.get().unwrap().upgrade().unwrap().set_timers(slint::ModelRc::from(timers.iter().map(|d| d.as_secs() as i32).collect::<Vec<i32>>().as_slice()));
}

pub(crate) fn close() {
    std::process::exit(0);
}