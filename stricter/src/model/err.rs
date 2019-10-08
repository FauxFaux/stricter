use std::borrow;

pub fn source_context(from: &[u8], e: &toml::de::Error, target_line: usize, col: usize) -> String {
    let doc = from.split(|&c| c == b'\n');
    let skipped = target_line.saturating_sub(4);
    let mut doc = doc.skip(skipped);
    let mut msg = String::with_capacity(128);
    for no in 0..7 {
        let this_line = skipped + no;
        let value = doc.next().map(String::from_utf8_lossy);
        msg.push_str(&format!(
            "{:3} | {}\n",
            this_line + 1,
            value.unwrap_or(borrow::Cow::Borrowed(""))
        ));
        if this_line == target_line {
            // TODO: not Debug formatting of `e`
            msg.push_str(&format!("     {} ^ {:?}\n", repeated_space(col), e));
        }
    }

    msg
}

fn repeated_space(col: usize) -> String {
    vec![' '; col].into_iter().collect::<String>()
}
