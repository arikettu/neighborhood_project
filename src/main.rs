mod config;
mod hk;
mod timer;
mod app;

use std::io::Write;
use std::sync::OnceLock;
use std::time::Duration;
use slint::{ComponentHandle, ModelRc};

slint::include_modules!();

// slint::Weak being sync is a fucking lie
pub(crate) static APP_HANDLE: OnceLock<slint::Weak<App>> = OnceLock::new();

fn create_timer() {
    print!("timer length (seconds): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u64>() {
        Ok(n) => {
            timer::start(Duration::from_secs(n), || {
                println!("timer finished");
            });
        },
        Err(_) => { println!("timer not started"); }
    };
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let app = App::new()?;
    match APP_HANDLE.set(app.as_weak()) {
        Ok(_) => {},
        Err(_) => { return Err("what".into()); }
    }
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F11".into()).unwrap(), || APP_HANDLE.get().unwrap().upgrade().unwrap().invoke_open())?;
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