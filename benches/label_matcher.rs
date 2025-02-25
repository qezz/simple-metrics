use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use simple_metrics::{
    matchers::{NaiveLabelNameChecker, RegexLabelNameChecker},
    Labels,
};

const SIZES: &[usize] = &[1000, 5000, 10000];

fn gen_tuples(size: usize) -> Vec<(String, String)> {
    let mut tuples = Vec::with_capacity(size);

    for i in 0..size {
        let label = format!("info_label_{}", i);
        let value = format!("some_value{}", i);

        tuples.push((label, value));
    }

    tuples
}

fn bench_label_names(c: &mut Criterion) {
    let mut group = c.benchmark_group("label_names");

    for case in SIZES {
        let orig = gen_tuples(*case);
        let tuples: Vec<(&str, &str)> = orig.iter().map(|x| (x.0.as_str(), x.1.as_str())).collect();

        let labels_regex = Labels::<RegexLabelNameChecker>::from(tuples.as_slice());
        let labels_naive = Labels::<NaiveLabelNameChecker>::from(tuples.as_slice());

        group.bench_with_input(BenchmarkId::new("regex", case), case, |b, _| {
            b.iter(|| black_box(labels_regex.check_names()))
        });

        group.bench_with_input(BenchmarkId::new("naive", case), case, |b, _| {
            b.iter(|| black_box(labels_naive.check_names()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_label_names);
criterion_main!(benches);
