# Simple metrics

The library provides a way to render Prometheus metrics in a more
flexible way compared to other implementations.

## Do you need this library?

Simple metrics library provides a very opinionated way on how things
should be or can be done. It comes from author's experience with
Prometheus client libraries and various exporters.

Goals:
- provide a modular way to render and compose metrics
- provide a functional (a.k.a pure) approach to metrics

Non goals:
- re-implement Prometheus metrics (Counter, Gauge, Histogram) and
  their behavior. It's your responsibility to keep the internal state
  consistent, and correctly derive metrics from it.
- have stateful metrics registry.
- manage global state for metrics.

You might find this library useful if you run multiple independent
workers that should serve different sets of metrics. The workers in
this case can have domain specific metrics, and are not limited to a
single format.

## Usage

In order to render metrics from a data struct, you need to implement
`IntoMetric<T>` for your type. `T` is the metric value type,
e.g. `IntoMetric<i64>`, `IntoMetric<f64>` etc.

If you run multiple similar workers, and their state does fit into the
same data structure, it's enough to implement the trait once, and
reuse it.

Since the workers can produce metrics on their own, it's easy to
aggregate the resulting metrics in order to serve them via an
endpoint.

## Example

A simple example on how to convert a struct into metrics:

```rust
struct Url(String);

struct Data {
    name: String,
    chain_id: String,
    samples: Vec<(Url, i64)>,
}

impl IntoMetric<i64> for Data {
    fn into_metric_group(self) -> Result<MetricGroup<i64>, Error> {
        let mut common_labels = HashMap::new();
        common_labels.insert("name".to_string(), self.name);
        common_labels.insert("chain_id".to_string(), self.chain_id);

        let metrics: Vec<Sample<i64>> = self
            .samples
            .into_iter()
            .map(|(url, value)| {
                let mut labels: HashMap<String, String> = HashMap::new();
                labels.insert("url".into(), url.0);

                labels.extend(common_labels.clone());

                Sample { labels, value }
            })
            .collect();

        let maybe_mg =
            MetricGroup::new("data_height", "this data height test", MetricType::Gauge)?;

        let with_metrics = maybe_mg.with_samples(metrics)?;

        Ok(with_metrics)
    }
}

let data = Data {
    name: "test".into(),
    chain_id: "pacific-1".into(),
    samples: vec![
        (Url("http://127.0.0.1:5000/".into()), 100),
        (Url("http://127.0.0.2:5000/".into()), 900),
    ],
};

let mg = data.into_metric_group().unwrap();

let expected = "\
# HELP data_height this data height test
# TYPE data_height gauge
data_height{chain_id=\"pacific-1\",name=\"test\",url=\"http://127.0.0.1:5000/\"} 100
data_height{chain_id=\"pacific-1\",name=\"test\",url=\"http://127.0.0.2:5000/\"} 900
";

assert_eq!(mg.render_into_metrics(), expected.to_string())

```

## Alternatives

- Official Rust client - [prometheus/client_rust](https://github.com/prometheus/client_rust/)
- TiKV's implementation - [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus)

## License

MIT or Apache-2, at your choice.

## Author

Sergei Mishin
