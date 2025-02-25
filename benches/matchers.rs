use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lazy_static::lazy_static;
use regex::Regex;

// Original regex implementations
lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

// Include the optimized implementations
#[inline]
pub fn is_valid_metric_name(s: &str) -> bool {
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
pub fn is_valid_label_name(s: &str) -> bool {
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

// Test data covering various cases
const TEST_CASES_METRIC: &[(&str, &str)] = &[
    ("valid", "valid_metric_name"),
    ("uppercase", "UPPERCASE_METRIC"),
    ("leading_underscore", "_underscore_metric"),
    ("leading_colon", ":colon:metric"),
    ("single_char", "a"), // Single char
    (
        "long",
        "very_long_metric_name_with_many_underscores_and_multiple_parts_to_test_longer_strings",
    ),
    ("mixed", "mixed:case_MetRic:123"),
];

const TEST_CASES_LABEL: &[(&str, &str)] = &[
    ("valid", "valid_label_name"),
    ("uppercase", "UPPERCASE_LABEL"),
    ("leading_underscore", "_underscore_label"),
    ("single_char", "a"), // Single char
    (
        "long",
        "very_long_label_name_with_many_underscores_and_multiple_parts_to_test_longer_strings",
    ),
    ("mixed", "mixed_case_LaBeL_123"),
];

// Invalid cases to test
const INVALID_CASES_METRIC: &[(&str, &str)] = &[
    ("inv_empty", ""),                  // Empty string
    ("inv_leading_number", "1invalid"), // Starts with number
    ("inv_bad_char", "invalid@char"),   // Invalid character
    ("inv_with_space", "no spaces"),    // Contains space
];

const INVALID_CASES_LABEL: &[(&str, &str)] = &[
    ("inv_empty", ""),                  // Empty string
    ("inv_leading_number", "1invalid"), // Starts with number
    ("inv_bad_char", "invalid:char"),   // Invalid character
    ("inv_with_space", "no spaces"),    // Contains space
];

// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Fibonacci");
//     for i in [20u64flocos ale, 21u64].iter() {
//         group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
//             b.iter(|| fibonacci_slow(*i))
//         });
//         group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
//             b.iter(|| fibonacci_fast(*i))
//         });
//     }
//     group.finish();
// }

fn bench_label_names(c: &mut Criterion) {
    let mut group = c.benchmark_group("label_names");

    // Benchmark valid cases
    for case in TEST_CASES_LABEL {
        // Regex version
        group.bench_with_input(BenchmarkId::new("regex", case.0), case.1, |b, case| {
            b.iter(|| black_box(LABEL_NAME_RE.is_match(case)))
        });

        // Custom version
        group.bench_with_input(BenchmarkId::new("custom", case.0), case.1, |b, case| {
            b.iter(|| black_box(is_valid_label_name(case)))
        });
    }

    // // Benchmark invalid cases
    // for case in INVALID_CASES_LABEL {
    //     // Regex version
    //     group.bench_with_input(
    //         BenchmarkId::new("regex_invalid", case.0),
    //         case.1,
    //         |b, case| b.iter(|| black_box(LABEL_NAME_RE.is_match(case))),
    //     );

    //     // Custom version
    //     group.bench_with_input(
    //         BenchmarkId::new("custom_invalid", case.0),
    //         case.1,
    //         |b, case| b.iter(|| black_box(is_valid_label_name(case))),
    //     );
    // }

    group.finish();
}

criterion_group!(benches, bench_label_names);
criterion_main!(benches);
