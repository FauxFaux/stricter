use failure::Error;
use serde_json::json;

#[test]
fn load_first() -> Result<(), Error> {
    let output = stricter::inventory::load(
        r"
for i in range(2):
    register({f'hello{i}': [i, i + 0.5]})
",
    )?;

    assert_eq!(
        json!([
            json!({
                "hello0": [0, 0.5],
            }),
            json!({
                "hello1": [1, 1.5],
            }),
        ]),
        output
    );

    Ok(())
}

#[test]
fn type_error() {
    let output = stricter::inventory::load("register({f'hello': [str]})").unwrap_err();
    assert_eq!(
        r##"
Traceback (most recent call last):
  File "<virtual>", line 1, in <module>
TypeError: unexpected type "type": "<class \'str\'>"
"##
        .trim(),
        format!("{}", output.find_root_cause()).trim()
    );
}
