use failure::Error;

#[test]
fn load_first() -> Result<(), Error> {
    println!(
        "{:?}",
        stricter::inventory::load("register({'hello': [5, 5.5]})")?
    );
    Ok(())
}
