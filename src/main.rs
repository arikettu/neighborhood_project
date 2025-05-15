mod hk;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct MSG {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>
}

unsafe extern "system" {
    pub unsafe fn GetMessageA(
        lpMsg: *mut MSG,
        hWnd: *mut core::ffi::c_void,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;
    pub unsafe fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub unsafe fn DispatchMessageA(lpMsg: *const MSG) -> isize;
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    hk::init()?;

    let mut msg: MSG = unsafe { core::mem::zeroed() };

    loop {
        unsafe {
            if GetMessageA(&mut msg, core::ptr::null_mut(), 0, 0) == 0 {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }

    Ok(())
}