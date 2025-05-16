mod hk;
mod config;

use std::io::Write;
use std::thread;
use std::time::Duration;

// temporary message loop things
// will be replaced with something less stupid later

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct MSG {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>
}

#[allow(non_snake_case)]
unsafe extern "system" {
    pub fn GetMessageA(
        lpMsg: *mut MSG,
        hWnd: *mut core::ffi::c_void,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32
    ) -> i32;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;
}

fn message_loop() { unsafe {
    let mut msg: MSG = core::mem::zeroed();
    loop {
        if GetMessageA(&mut msg, core::ptr::null_mut(), 0, 0) == 0 { break; }
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
} }

// end temporary message loop things

fn create_timer() {
    print!("timer length (seconds): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u64>() {
        Ok(n) => {
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(n));
                println!("{} second timer complete", n);
            });
        },
        Err(_) => { println!("timer not started"); }
    };
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F11".into()).unwrap(), create_timer)?;
    let cfg = match config::config() {
        Ok(cfg) => { Some(cfg) },
        Err(e) => {
            println!("could not load config");
            None
        }
    };

    message_loop();
    Ok(())
}