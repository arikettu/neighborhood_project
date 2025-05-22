mod config;
mod hk;
mod app;

use std::sync::OnceLock;
use slint::{ComponentHandle, ModelRc};

slint::include_modules!();

// slint::Weak being sync is a fucking lie
pub(crate) static APP_HANDLE: OnceLock<slint::Weak<App>> = OnceLock::new();

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let app = App::new()?;
    match APP_HANDLE.set(app.as_weak()) {
        Ok(_) => {},
        Err(_) => { return Err("what".into()); }
    }
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F11".into()).unwrap(), || APP_HANDLE.get().unwrap().upgrade().unwrap().invoke_create(1, 0, 0))?;
    app.on_create(app::create);
    app.on_close(app::close);
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
    app.run()?;
    Ok(())
}