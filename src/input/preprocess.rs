pub fn preprocess_html(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                result.push('\n');
                if let Some(&'\n') = chars.peek() {
                    chars.next();
                }
            }
            '\0' => result.push('\u{FFFD}'),
            _ => result.push(c),
        }
    }
    result
}
