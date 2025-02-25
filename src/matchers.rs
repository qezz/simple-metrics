use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

#[inline]
pub fn is_valid_metric_name(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    match bytes[0] {
        b'A'..=b'Z' | b'a'..=b'z' | b'_' | b':' => {}
        _ => return false,
    }

    bytes[1..].iter().all(|&b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b':'
        )
    })
}

#[inline]
pub fn is_valid_label_name(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    match bytes[0] {
        b'A'..=b'Z' | b'a'..=b'z' | b'_' => {}
        _ => return false,
    }

    bytes[1..].iter().all(|&b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_'
        )
    })
}

pub trait MetricNameChecker {
    fn is_valid(name: &str) -> bool;
}

pub struct RegexMetricNameChecker;

impl MetricNameChecker for RegexMetricNameChecker {
    fn is_valid(name: &str) -> bool {
        METRIC_NAME_RE.is_match(name)
    }
}

pub struct NaiveMetricNameChecker;

impl MetricNameChecker for NaiveMetricNameChecker {
    fn is_valid(name: &str) -> bool {
        is_valid_metric_name(name)
    }
}

pub trait LabelNameChecker {
    fn init() -> Self;
    fn is_valid(&self, name: &str) -> bool;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegexLabelNameChecker;

impl LabelNameChecker for RegexLabelNameChecker {
    fn init() -> Self {
        Self{}
    }

    fn is_valid(&self, name: &str) -> bool {
        LABEL_NAME_RE.is_match(name)
    }
}

pub struct NaiveLabelNameChecker;

impl LabelNameChecker for NaiveLabelNameChecker {
    fn init() -> Self {
        Self{}
    }

    fn is_valid(&self, name: &str) -> bool {
        is_valid_label_name(name)
    }
}
