use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use std::collections::HashMap;

fn labels_render_naive(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!(r#"{}="{}""#, k, v.replace('"', r#"\""#)))
        .collect::<Vec<String>>()
        .join(",")
}

fn labels_render_manual(map: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut first = true;

    for (k, v) in map.iter() {
        if !first {
            result.push(',');
        } else {
            first = false;
        }

        result.push_str(k);
        result.push_str("=\"");

        for c in v.chars() {
            if c == '"' {
                result.push_str("\\\"");
            } else {
                result.push(c);
            }
        }

        result.push('"');
    }

    result
}

const SIZES: &[usize] = &[100, 1000, 10000];

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    // group.measurement_time(std::time::Duration::from_secs(15));
    group.plot_config(plot_config);

    for size in SIZES.iter() {
        let mut map = HashMap::new();
        for i in 0..*size {
            let key = format!("a_kinda_long_label_that_i_want_to_use_here{}", i);
            let value = format!(
                "a_kinda_long_label_value_that_i_want_to_use_here{} also with \"quotes\"",
                i
            );
            map.insert(key, value);
        }

        group.bench_with_input(BenchmarkId::new("naive", size), &map, |b, map| {
            b.iter(|| labels_render_naive(black_box(map)))
        });

        group.bench_with_input(BenchmarkId::new("manual", size), &map, |b, map| {
            b.iter(|| labels_render_manual(black_box(map)))
        });
    }

    group.finish();
}

fn criterion_benchmark_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_short");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    // group.measurement_time(std::time::Duration::from_secs(10));
    group.plot_config(plot_config);

    for size in SIZES.iter() {
        let mut map = HashMap::new();
        for i in 0..*size {
            let key = format!("short_label{}", i);
            let value = format!("short_label_value{} \"quotes\"", i);
            map.insert(key, value);
        }

        group.bench_with_input(BenchmarkId::new("naive", size), &map, |b, map| {
            b.iter(|| labels_render_naive(black_box(map)))
        });

        group.bench_with_input(BenchmarkId::new("manual", size), &map, |b, map| {
            b.iter(|| labels_render_manual(black_box(map)))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_short);
criterion_main!(benches);
