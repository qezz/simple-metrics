use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use simple_metrics::{
    matchers::{
        NaiveLabelNameChecker, NaiveLabelNameCheckerV2, NaiveLabelNameCheckerV3,
        RegexLabelNameChecker,
    },
    Labels,
};

const SIZES: &[usize] = &[1000, 5000, 10000];

fn gen_tuples(size: usize) -> Vec<(String, String)> {
    let mut tuples = Vec::with_capacity(size);

    for i in 0..size {
        let label = format!("info_label{}", i);
        let value = format!("some_value{}", i);

        tuples.push((label, value));
    }

    tuples
}

fn bench_label_names(c: &mut Criterion) {
    let mut group = c.benchmark_group("label_names_comparison");

    for case in SIZES {
        let orig = gen_tuples(*case);
        let tuples: Vec<(&str, &str)> = orig.iter().map(|x| (x.0.as_str(), x.1.as_str())).collect();

        {
            let labels_regex = Labels::<RegexLabelNameChecker>::from(tuples.as_slice());
            group.bench_with_input(BenchmarkId::new("regex", case), case, |b, _| {
                b.iter(|| black_box(labels_regex.check_names()))
            });
        }

        {
            let labels_naive = Labels::<NaiveLabelNameChecker>::from(tuples.as_slice());
            group.bench_with_input(BenchmarkId::new("naive", case), case, |b, _| {
                b.iter(|| black_box(labels_naive.check_names()))
            });
        }

        {
            let labels_naive_v2 = Labels::<NaiveLabelNameCheckerV2>::from(tuples.as_slice());
            group.bench_with_input(BenchmarkId::new("naive_v2", case), case, |b, _| {
                b.iter(|| black_box(labels_naive_v2.check_names()))
            });
        }

        {
            let labels = Labels::<NaiveLabelNameCheckerV3>::from(tuples.as_slice());
            group.bench_with_input(BenchmarkId::new("naive_v3", case), case, |b, _| {
                b.iter(|| black_box(labels.check_names()))
            });
        }
    }

    group.finish();
}

fn regression_label_names(c: &mut Criterion) {
    let mut group = c.benchmark_group("label_names_regression");

    for case in SIZES {
        let orig = gen_tuples(*case);
        let tuples: Vec<(&str, &str)> = orig.iter().map(|x| (x.0.as_str(), x.1.as_str())).collect();

        let labels: Labels = Labels::from(tuples.as_slice());

        group.bench_with_input(BenchmarkId::new("labels_default", case), case, |b, _| {
            b.iter(|| black_box(labels.check_names()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_label_names, regression_label_names);
criterion_main!(benches);
