use std::sync::Mutex;
use std::collections::HashMap;
use crate::hook::*;

lazy_static::lazy_static! {
    static ref CALLBACKS: Mutex<HashMap<String, fn()>> = Mutex::new(HashMap::new());
}

static LWIN: Mutex<bool> = Mutex::new(false);
static RWIN: Mutex<bool> = Mutex::new(false);
static LCTRL: Mutex<bool> = Mutex::new(false);
static RCTRL: Mutex<bool> = Mutex::new(false);
static LALT: Mutex<bool> = Mutex::new(false);
static RALT: Mutex<bool> = Mutex::new(false);
static LSHIFT: Mutex<bool> = Mutex::new(false);
static RSHIFT: Mutex<bool> = Mutex::new(false);

#[allow(non_snake_case)]
extern "C" fn low_level_keyboard_proc(
    nCode: i32,
    wParam: WPARAM,
    lParam: LPARAM
) -> LRESULT {
    if nCode < 0 { return unsafe { CallNextHookEx(0, nCode, wParam, lParam) }; }
    let key = unsafe { &*(lParam as *const KBDLLHOOKSTRUCT) };

    let down =  key.flags & 0x80 == 0;

    // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    match key.vkCode {
        0x5B => { *LWIN.lock().unwrap() = down; },
        0x5C => { *RWIN.lock().unwrap() = down; },
        0xA2 => { *LCTRL.lock().unwrap() = down; },
        0xA3 => { *RCTRL.lock().unwrap() = down; },
        0xA4 => { *LALT.lock().unwrap() = down; },
        0xA5 => { *RALT.lock().unwrap() = down; },
        0xA0 => { *LSHIFT.lock().unwrap() = down; },
        0xA1 => { *RSHIFT.lock().unwrap() = down; },
        _ => { }
    }
    
    0
}

pub fn init() -> Result<(), Box<dyn core::error::Error>> {
    unsafe {
        if SetWindowsHookExW(13, low_level_keyboard_proc, 0, 0) == 0 {
            return Err(format!("os error {}", GetLastError()).into());
        }
    }

    Ok(())
}

pub fn add_shortcut(shortcut: String, callback: fn()) -> Result<(), Box<dyn core::error::Error>> {
    CALLBACKS.lock()?.insert(shortcut, callback);
    Ok(())
}

pub fn remove_shortcut(shortcut: String) -> Result<(), Box<dyn core::error::Error>> {
    CALLBACKS.lock()?.remove(&shortcut);
    Ok(())
}