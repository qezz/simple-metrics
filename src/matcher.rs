use regex::Regex;

lazy_static::lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

#[inline]
pub fn naive_is_valid_metric_name(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Check first character
    match bytes[0] {
        b'A'..=b'Z' | b'a'..=b'z' | b'_' | b':' => {}
        _ => return false,
    }

    // Check remaining characters
    bytes[1..].iter().all(|&b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b':'
        )
    })
}

#[inline]
pub fn naive_is_valid_label_name(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Check first character
    match bytes[0] {
        b'A'..=b'Z' | b'a'..=b'z' | b'_' => {}
        _ => return false,
    }

    // Check remaining characters
    bytes[1..].iter().all(|&b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_'
        )
    })
}

pub fn regex_is_valid_metric_name(name: &str) -> bool {
    METRIC_NAME_RE.is_match(name)
}

pub fn regex_is_valid_label_name(name: &str) -> bool {
    LABEL_NAME_RE.is_match(name)
}
