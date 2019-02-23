/// Escape `s` to be FinTS compliant.
fn escape_fints(s: &str) -> String {
    s.replace("?", "??")
        .replace("+", "?+")
        .replace(":", "?:")
        .replace("@", "?@")
}

/// Unescape `s` from a FinTS-escaped format.
fn unescape_fints(s: &str) -> String {
    s.replace("??", "?")
        .replace("?+", "+")
        .replace("?:", ":")
        .replace("?@", "@")
}
