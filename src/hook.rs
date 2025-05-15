#[allow(non_snake_case)]
#[repr(C)]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

pub type HHOOK = isize;
pub type HINSTANCE = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HOOKPROC = extern "C" fn(i32, WPARAM, LPARAM) -> LRESULT;

#[link(name = "user32")]
unsafe extern "system" {
    pub fn SetWindowsHookExW(
        idHook: i32,
        lpfn: HOOKPROC,
        hmod: HINSTANCE,
        dwThreadId: u32
    ) -> HHOOK;

    pub fn CallNextHookEx(
        hhk: HHOOK,
        nCode: i32,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> LRESULT;

    pub fn GetLastError() -> i32;
}