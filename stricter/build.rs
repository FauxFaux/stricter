use vergen::generate_cargo_keys;
use vergen::ConstantsFlags;

fn main() {
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate cargo keys!");
}
