const GWL_STYLE: i32 = -16;
const WS_POPUP: i64 = 0x80000000;
const WS_VISIBLE: i64 = 0x10000000;
const GWL_EXSTYLE: i32 = -20;
const WS_EX_TOOLWINDOW: i64 = 0x00000080;
const GWLP_WNDPROC: i32 = -4;
const WM_CLOSE: u32 = 0x0010;

#[allow(non_snake_case)]
#[link(name = "user32")]
unsafe extern "C" {
    pub fn SetWindowLongPtrW(
        hWnd: isize,
        nIndex: i32,
        dwNewLong: isize,
    ) -> isize;

    pub fn SetMenu(
        hWnd: isize,
        hMenu: isize,
    ) -> bool;

    pub fn DefWindowProcW(
        hWnd: isize,
        uMsg: u32,
        wParam: u64,
        lParam: isize,
    ) -> isize;
}

extern "C" fn wnd_proc(hWnd: isize, uMsg: u32, wParam: u64, lParam: isize) -> isize {
    if uMsg == WM_CLOSE { return 0; }
    unsafe { DefWindowProcW(hWnd, uMsg, wParam, lParam) }
}

pub(crate) fn style(handle: isize) { unsafe {
    SetWindowLongPtrW(handle, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as isize);
    // SetWindowLongPtrW(handle, GWL_EXSTYLE, WS_EX_TOOLWINDOW as isize);
    // SetMenu(handle, 0);
    SetWindowLongPtrW(handle, GWLP_WNDPROC, wnd_proc as isize);
} }
