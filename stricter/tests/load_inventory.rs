use failure::Error;

#[test]
fn load_first() -> Result<(), Error> {
    stricter::inventory::load()
}
