mod config;
mod hk;
mod app;

use std::sync::{OnceLock, RwLock};
use slint::{ComponentHandle, ModelRc};

slint::include_modules!();

// slint::Weak being sync is a fucking lie
pub(crate) static APP_HANDLE: OnceLock<slint::Weak<App>> = OnceLock::new();
pub(crate) static OPEN: RwLock<bool> = RwLock::new(false);

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let app = App::new()?;
    match APP_HANDLE.set(app.as_weak()) {
        Ok(_) => {},
        Err(_) => { return Err("??".into()); }
    }
    app.on_create(app::create);
    app.on_show(app::show);
    app.on_hide(app::hide);
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F11".into()).unwrap(), || {
        let mut open = OPEN.write().unwrap();
        match *open {
            true => { APP_HANDLE.get().unwrap().upgrade().unwrap().invoke_hide(); },
            false => { APP_HANDLE.get().unwrap().upgrade().unwrap().invoke_show(); }
        }
        *open = !*open;
    })?;
    match config::config() {
        Ok(cfg) => {
            app.set_presets(ModelRc::from(
                cfg.presets
                    .iter()
                    .map(|preset| preset.title.clone().into())
                    .collect::<Vec<slint::SharedString>>()
                    .as_slice()
            ));
        },
        Err(e) => {
            println!("error loading config: {}", e);
            println!("default config path is %LOCALAPPDATA%\\neighborhood\\config.json");
        }
    };
    slint::run_event_loop_until_quit()?;
    Ok(())
}