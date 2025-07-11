# Simple metrics

The library allows to render Prometheus metrics in a more flexible way
compared to other implementations.

## Do you need this library?

Simple metrics library provides a very opinionated way on how things
should be or can be done. It comes from author's experience with
Prometheus client libraries and various exporters.

Goals:
- **provide a modular way to render and compose metrics**: You can
  have multiple workers, each of them should be able to produce
  independent set of metrics.
- **provide data-driven approach to metrics**: The metrics can be
  constructed and rendered based on exporter's internal state. There's
  no stateful metrics registry, and it's your responsibility to update
  state when needed.

Non goals:
- re-implement Prometheus metrics (Counter, Gauge, Histogram) and
  their behavior. It's your responsibility to keep the internal state
  consistent, and correctly derive metrics from it.
   - Side note: I'm currently thinking on having those types here, but
     it will take some time to implement them properly.
- have stateful metrics registry.
- manage global state for metrics.

You might find this library useful if you run multiple independent
workers that should serve different sets of metrics. The workers in
this case can have domain specific metrics, and are not limited to a
single format.

Because there's no global registry, and because the metrics are
data-driven, it becomes relatively easy to test your application. See
unit tests in the code, [examples](examples), or keep reading for a
simple demo.

## Usage

In order to render metrics, you need to have an `enum`, every variant
of which represents a separate metric group.

```rust
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ServiceMetric {
    WorkerHealth,
    ServiceHeight,
}

impl ToMetricDef for ServiceMetric {
    fn to_metric_def(&self) -> MetricDef {
        match self {
            ServiceMetric::WorkerHealth => {
                MetricDef::new("worker_health", "worker health", MetricType::Gauge).unwrap()
            }
            ServiceMetric::ServiceHeight => {
                MetricDef::new("service_height", "service height", MetricType::Gauge).unwrap()
            }
        }
    }
}
```

Since the workers can produce metrics on their own, it's easy to
aggregate the result in order to serve all the metrics via an
endpoint.

A simple example on how to convert a list of states into metrics:

```rust
pub struct State {
    name: String,
    health: bool,
    height: i64,
}

let states = vec![
    SimpleState {
        name: "a".into(),
        health: true,
        height: 100,
    },
    SimpleState {
        name: "b".into(),
        health: false,
        height: 200,
    },
];

let mut static_labels = LabelsBuilder::new()
    .with("process", "simple-metrics")
    .build()
    .expect("invalid label names");

let mut store: MetricStore<ServiceMetric> =
    MetricStore::new().with_static_labels(static_labels);

for s in states {
    let mut common = LabelsBuilder::new()
        .with("name", s.name)
        .build()
        .expect("invalid label names");

    store.add_sample(
        ServiceMetric::WorkerHealth,
        Sample::new(&common, s.health)
    );

    store.add_value(ServiceMetric::ServiceHeight, &common, s.height);
}

let actual = store.render_into_metrics(Some("namespace"));
println!("{}", actual);

let expected = r#"# HELP worker_health worker health
# TYPE worker_health gauge
namespace_worker_health{name="a",process="simple-metrics"} 1
namespace_worker_health{name="b",process="simple-metrics"} 0

# HELP service_height service height
# TYPE service_height gauge
namespace_service_height{name="a",process="simple-metrics"} 100
namespace_service_height{name="b",process="simple-metrics"} 200
"#;

assert_eq!(actual, expected);
```

## Alternatives

- Official Rust client - [prometheus/client_rust](https://github.com/prometheus/client_rust/)
- TiKV's implementation - [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus)

## License

MIT or Apache-2, at your choice.

## Author

Sergei Mishin
