use slint::ComponentHandle;
use raw_window_handle::HasWindowHandle;

#[allow(non_snake_case)]
#[link(name = "user32")]
unsafe extern "C" {
    pub fn SetWindowLongPtrW(
        hWnd: isize,
        nIndex: i32,
        dwNewLong: i64,
    ) -> isize;

    pub fn SetMenu(
        hWnd: isize,
        hMenu: isize,
    ) -> bool;

    // pub fn DefWindowProcW(
    //     hWnd: isize,
    //     uMsg: u32,
    //     wParam: u64,
    //     lParam: isize,
    // ) -> isize;

    pub fn SetWindowPos(
        hWnd: isize,
        hWndInsertAfter: isize,
        X: i32,
        Y: i32,
        cx: i32,
        cy: i32,
        uFlags: u32,
    ) -> bool;
}

fn get_handle() -> isize {
    match crate::APP_HANDLE.get().unwrap().upgrade().unwrap().window().window_handle().window_handle().unwrap().as_raw() {
        raw_window_handle::RawWindowHandle::Win32(handle) => { isize::from(handle.hwnd) }
        _ => { unreachable!() }
    }
}

// #[allow(non_snake_case)]
// extern "C" fn wnd_proc(hWnd: isize, uMsg: u32, wParam: u64, lParam: isize) -> isize {
//     if uMsg == 0x10 { return 0; }
//     unsafe { DefWindowProcW(hWnd, uMsg, wParam, lParam) }
// }

pub(crate) fn style() { unsafe {
    let handle = get_handle();
    SetWindowLongPtrW(handle, -16, 0x80000000 | 0x10000000);
    SetWindowLongPtrW(handle, -20, 0x80);
    SetMenu(handle, 0);
    // SetWindowLongPtrW(handle, -4, wnd_proc as i64);
    SetWindowPos(handle, -1, 0, 0, 0, 0, 0x0001 | 0x0002 | 0x0008 | 0x0010);
} }