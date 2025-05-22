// from KnownFolders.h

/*
#if defined(INITGUID) || defined(INITKNOWNFOLDERS)
#define DEFINE_KNOWN_FOLDER(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
        EXTERN_C const GUID DECLSPEC_SELECTANY name \
                = { l, w1, w2, { b1, b2,  b3,  b4,  b5,  b6,  b7,  b8 } }
#else
#define DEFINE_KNOWN_FOLDER(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
        EXTERN_C const GUID name
#endif // INITGUID || INITKNOWNFOLDERS
*/

// and

/*
// {F1B32785-6FBA-4FCF-9D55-7B8E7F157091}
DEFINE_KNOWN_FOLDER(FOLDERID_LocalAppData,        0xF1B32785, 0x6FBA, 0x4FCF, 0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91);
*/

// what the fuck is that

use std::io::Read;
use serde::{Deserialize, Serialize};
use std::os::windows::ffi::OsStringExt;

#[repr(C)]
struct GUID {
    pub l: u32,
    pub w1: u16,
    pub w2: u16,
    pub b: [u8; 8],
}

#[allow(non_snake_case)]
#[link(name = "shell32")]
unsafe extern "system" {
    fn SHGetKnownFolderPath(
        rfid: *const GUID,
        dwFlags: u32,
        hToken: isize,
        ppszPath: *mut *mut u16
    ) -> i32;
}

#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub title: String,
    pub length: u64
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub presets: Vec<Preset>
}

pub fn config() -> Result<Config, Box<dyn core::error::Error>> {
    #[allow(non_snake_case)]
    let FOLDERID_LocalAppData = GUID {
        l: 0xF1B32785,
        w1: 0x6FBA,
        w2: 0x4FCF,
        b: [0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91]
    };

    let path = unsafe {
        let mut raw_path: *mut u16 = std::ptr::null_mut();
        if SHGetKnownFolderPath(&FOLDERID_LocalAppData, 0, 0, &mut raw_path) != 0 { return Err("could not find config directory (localappdata)".into()); }
        let mut len = 0;
        while *raw_path.add(len) != 0 { len += 1; }
        let slice = std::slice::from_raw_parts(raw_path, len);
        match std::ffi::OsString::from_wide(slice).into_string() {
            Ok(path) => { format!("{}\\neighborhood", path) },
            Err(_) => { return Err("invalid utf-8 in config directory (localappdata) path".into()); }
        }
    };

    std::fs::DirBuilder::new().recursive(true).create(&path)?;  // ensure %LOCALAPPDATA%\neighborhood exists
    let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(format!("{}\\config.json", path))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(serde_json::from_str::<Config>(&*buffer)?)
}