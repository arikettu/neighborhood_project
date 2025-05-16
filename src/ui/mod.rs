mod app;

slint::include_modules!();

pub fn init() -> Result<App, Box<dyn core::error::Error>> {
    let app = App::new()?;
    app.on_close(app::close);
    Ok(app)
}