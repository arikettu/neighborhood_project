use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref CALLBACKS: Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>> = Mutex::new(HashMap::new());
}

/*
here's some C++ code I wrote for a stenography program
ignore the comment block, apparantly wParam can be used to check if the keypress was sent by the current process
oops

LRESULT CALLBACK LowLevelKeyboardProc(int nCode, WPARAM wParam, LPARAM lParam) {
    if (nCode < HC_ACTION || !steno) return CallNextHookEx(nullptr, nCode, wParam, lParam);
    auto key = reinterpret_cast<KBDLLHOOKSTRUCT *>(lParam);

    // can probably make some "compatability mode" to handle input through processes
//    if (!qskip.empty()) {
//        auto o = std::find(qskip.begin(), qskip.end(), key->vkCode);
//        if (o != qskip.end()) {
//            qskip.erase(o);
//            return 0;
//        }
//    }

//    if (key->flags & LLKHF_INJECTED) return 0;

    if (key->flags & LLKHF_UP) {
        --down;
        if (down <= 0) {
            down = 0;
            if (note) {
                std::cout << "\33[2K\r" << getspacedinput() << std::endl;
                clear();
                return 1;
            }
            std::cout << "\33[2K\r" << getspacedinput() << " ✓";
            handle(getinput());
            clear();
            qrem.clear();
            return 1;
        }
        qrem.push_back(key->vkCode);
        std::cout << "\33[2K\r" << getspacedinput();
        return 1;
    }

    ++down;
    if (!qrem.empty()) { for (int i : qrem) krem(i); }
    qrem.clear();
    kadd(key->vkCode);
    std::cout << "\33[2K\r" << getspacedinput();
    return 1;
}

int run(const std::vector<std::string>& dicts) {
    if (!note) parser_init(dicts);
    SetWindowsHookEx(WH_KEYBOARD_LL, LowLevelKeyboardProc, nullptr, 0);

    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}
*/

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
pub type HWND = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HOOKPROC = extern "C" fn(i32, WPARAM, LPARAM) -> LRESULT;

#[link(name = "user32")]
unsafe extern "system" {
    pub unsafe fn SetWindowsHookExW(
        idHook: i32,
        lpfn: HOOKPROC,
        hmod: HINSTANCE,
        dwThreadId: u32,
    ) -> HHOOK;

    pub unsafe fn CallNextHookEx(
        hhk: HHOOK,
        nCode: i32,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
}

#[allow(non_snake_case)]
extern "C" fn low_level_keyboard_proc(
    nCode: i32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    if nCode < 0 {
        return unsafe { CallNextHookEx(0, nCode, wParam, lParam) };
    }

    let key = unsafe { &*(lParam as *const KBDLLHOOKSTRUCT) };

    println!("Key pressed: {}", key.vkCode);

    0
}

pub fn init() -> Result<(), Box<dyn core::error::Error>> {
    let hook = unsafe {
        SetWindowsHookExW(
            14, // WH_KEYBOARD_LL
            low_level_keyboard_proc,
            0,
            0,
        )
    };

    if hook == 0 { return Err("Failed to set hook".into()); }

    println!("Hook set successfully");

    Ok(())
}