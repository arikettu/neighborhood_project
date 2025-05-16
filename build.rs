fn main() -> Result<(), Box<dyn core::error::Error>> {
    slint_build::compile("ui/app.slint")?;
    Ok(())
}