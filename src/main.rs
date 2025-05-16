mod config;
mod hk;
mod timer;
mod ui;

use std::io::Write;
use std::time::Duration;
use slint::ComponentHandle;

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
    hk::init()?;
    hk::add_shortcut(hk::KeyboardState::parse("LCONTROL F11".into()).unwrap(), create_timer)?;
    let _cfg = match config::config() {
        Ok(cfg) => { Some(cfg) },
        Err(e) => {
            println!("could not load config");
            None
        }
    };
    let app = ui::init()?;
    app.run()?;
    Ok(())
}