mod hk;
mod hook;

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

// end temporary message loop things

fn test_print() {
    println!("hi");
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F2".into()).unwrap(), test_print)?;

    // this block is also a temporary message loop thing
    unsafe {
        let mut msg: MSG = core::mem::zeroed();
        loop {
            if GetMessageA(&mut msg, core::ptr::null_mut(), 0, 0) == 0 { break; }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }

    Ok(())
}