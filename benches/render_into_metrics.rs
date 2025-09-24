use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{borrow::Cow, collections::HashMap};

use simple_metrics::*;

// #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
// enum TestMetricKey {
//     Cat,
//     Dog,
// }

// impl ToMetricDef for TestMetricKey {
//     fn to_metric_def(&self) -> MetricDef {
//         match self {
//             TestMetricKey::Cat => gauge!("cat", "cat"),
//             TestMetricKey::Dog => gauge!("dog", "dog"),
//         }
//     }
// }

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TestMetricKey<'a> {
    name: Cow<'a, str>,
}

impl TestMetricKey<'_> {
    fn new(idx: usize) -> Self {
        Self {
            name: format!("metric_{}", idx).into(),
        }
    }
}

impl ToMetricDef for TestMetricKey<'_> {
    fn to_metric_def(&self) -> MetricDef {
        // match self {
        //     TestMetricKey::Cat => gauge!("cat", "cat"),
        //     TestMetricKey::Dog => gauge!("dog", "dog"),
        // }
        MetricDef::gauge(&self.name, &self.name).unwrap()
    }
}

fn create_test_store<'a>(
    num_metrics: usize,
    samples_per_metric: usize,
) -> MetricStore<TestMetricKey<'a>> {
    // Add some static labels
    // let mut static_labels = LabelsBuilder::new();
    // static_labels.insert("instance".to_string(), "server-01".to_string());
    // static_labels.insert("region".to_string(), "us-west-2".to_string());
    // store = store.with_static_labels(static_labels.build().unwrap());

    let static_labels = LabelsBuilder::new()
        .with("instance", "server-01")
        .with("region", "eu-north-1")
        .build()
        .unwrap();

    let mut store = MetricStore::new().with_static_labels(static_labels);

    for metric_idx in 0..num_metrics {
        // let key = TestMetricKey::new(format!("metric_{}", metric_idx));
        // let key = MetricDef::new_unchecked(
        //     &format!("metric_{}", metric_idx),
        //     "_".to_string(),
        //     MetricType::Gauge,
        // );

        // let key = TestMetricKey::Cat;
        let key = TestMetricKey::new(metric_idx);

        for sample_idx in 0..samples_per_metric {
            let mut labels = LabelsBuilder::new()
                .with("method", format!("GET_{}", sample_idx % 5))
                .with("status".to_string(), format!("{}", 200 + (sample_idx % 5)));

            let sample = Sample::new(&labels.build().unwrap(), 42.0 + sample_idx as f64);

            store.add_sample(key.clone(), sample);
        }
    }

    store
}

fn bench_render_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_metrics");

    // Test different scales
    let configs = [
        (10, 5),    // 10 metrics, 5 samples each
        (100, 10),  // 100 metrics, 10 samples each
        (1000, 20), // 1000 metrics, 20 samples each
    ];

    for (num_metrics, samples_per_metric) in configs {
        let store = create_test_store(num_metrics, samples_per_metric);

        group.bench_with_input(
            BenchmarkId::new(
                "with_namespace",
                format!("{}x{}", num_metrics, samples_per_metric),
            ),
            &store,
            |b, store| b.iter(|| black_box(store.render_into_metrics(Some("myapp")))),
        );

        group.bench_with_input(
            BenchmarkId::new(
                "without_namespace",
                format!("{}x{}", num_metrics, samples_per_metric),
            ),
            &store,
            |b, store| b.iter(|| black_box(store.render_into_metrics(None))),
        );
    }

    group.finish();
}

// fn bench_compare_implementations(c: &mut Criterion) {
//     let store = create_test_store(100, 10);

//     let mut group = c.benchmark_group("implementation_comparison");

//     group.bench_function("render_into_metrics", |b| {
//         b.iter(|| black_box(store.render_into_metrics(Some("myapp"))))
//     });

//     // group.bench_function("optimized", |b| {
//     //     b.iter(|| black_box(store.render_into_metrics(Some("myapp"))))
//     // });

//     // group.bench_function("manual_assembly", |b| {
//     //     b.iter(|| black_box(store.render_into_metrics_manual(Some("myapp"))))
//     // });

//     group.finish();
// }

criterion_group!(
    benches,
    bench_render_metrics, // bench_compare_implementations
);
criterion_main!(benches);
