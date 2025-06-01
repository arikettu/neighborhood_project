use std::sync::{Arc, LazyLock, RwLock};
use std::time::{Duration, Instant};
use slint::ComponentHandle;

pub(crate) static TIMERS: LazyLock<Arc<RwLock<Vec<(Instant, Duration, Instant)>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

pub(crate) fn show() { slint::invoke_from_event_loop(|| {
    crate::APP_HANDLE.get().unwrap().upgrade().unwrap().show().unwrap();
    crate::window::style();
}).unwrap(); }

pub(crate) fn hide() { slint::invoke_from_event_loop(|| crate::APP_HANDLE.get().unwrap().upgrade().unwrap().hide().unwrap()).unwrap(); }

pub(crate) fn create(hours: i32, minutes: i32, seconds: i32) {
    let now = Instant::now();
    let dur = Duration::from_secs(((hours * 3600) + (minutes * 60) + seconds) as u64);
    let end = now + dur;
    let mut timers = TIMERS.write().unwrap();  // just do this instead of swearing on the holy baguette
    timers.push((now, dur, end));
    drop(timers);
    update();
}

pub(crate) fn update() {
    let now = Instant::now();
    let mut timers = TIMERS.write().unwrap();
    let mut needs_update = false;
    let mut ix = -1;
    let mut remove_queue = Vec::new();
    for i in timers.iter_mut() {
        ix += 1;
        if i.2 <= now {
            remove_queue.push(ix);
            needs_update = true;
            continue
        }
        let real_diff = i.2.duration_since(now);
        if real_diff.as_secs() != i.1.as_secs() {
            needs_update = true;
            i.1 = real_diff;
        }
    }
    for i in remove_queue.iter() { timers.remove(*i as usize); }
    if needs_update { crate::APP_HANDLE.get().unwrap().upgrade_in_event_loop(|x| x.set_timers(slint::ModelRc::from(TIMERS.read().unwrap().iter().map(|times| (times.1.as_secs() as i32, times.2.duration_since(times.0).as_secs() as i32)).collect::<Vec<(i32, i32)>>().as_slice()))).unwrap() };
}