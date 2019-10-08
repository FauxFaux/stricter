#[test]
fn load_first() -> Result<(), failure::Error> {
    stricter::model::load(&include_bytes!("model/first.toml")[..])?;
    Ok(())
}
