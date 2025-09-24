use simple_metrics::{metric_def, LabelsBuilder, MetricDef, MetricStore, MetricType, RenderIntoMetrics, ToMetricDef};

// Define your metrics as enum variants. The variants are later used
// in `MetricStore` to reference existing metrics.
//
// General idea is that the metrics enum is very lightweight and is
// only used as a marker to add samples to.
//
// Required traits:
//   - for internal structure of MetricStore: Eq, Hash, Ord, PartialEq, PartialOrd
//   - for `ToMetricDef`: Clone
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metric {
    ServiceHealth,
    InternalData,
}

// `ToMetricDef` should be implemented to comply with `MetricStore<T>`
// requirements.
//
// It's recommended to use exhaustive matching so the compiler catches
// a bug if you forget to add a new variant to the `Metric` enum.
impl ToMetricDef for Metric {
    fn to_metric_def(&self) -> simple_metrics::MetricDef {
        // Create an exhaustive match on `self`.
        //
        // For each arm you have a couple of options:
        //
        // 1. Dynamic metric definition - provide any string to its
        //    constructor. It returns a `Result` because not all characters
        //    are allowed in the metric names.
        //
        // 2. Metric definition checked in compile-time - use macros
        //    which will fail in compile time, but the metric name
        //    should be a static str, and a valid one.
        //
        // There is a compile time penalty to using macros, but it's less
        // than `.unwrap()`-ing each Result value.
        //
        // Consider using macros if you need a static set of metrics.
        //
        // Consider using dynamic if the list of metric names are not
        // known at compile time (e.g. derived metrics or any type of user input)
        match self {
            Metric::ServiceHealth => {
                MetricDef::gauge("service_health", "service health (1 or 0)").unwrap()
            },
            Metric::InternalData => metric_def!("internal_data", "some exported value", MetricType::Gauge)
        }
    }
}

// This is the internal state of the app.
//
// It's recommended to build the metric store based on this state.
//
// That said, it allows to separate data processing (i.e. updating the
// app state) and metrics collection (i.e. building the `MetricStore`).
struct AppState {
    service_name: String,
    service_is_healthy: bool,
    service_data: u64,
}

impl AppState {
    // Initial state of the app.
    //
    // It's recommended to have the initial state representing working
    // being unhealthy, so you can catch possible issues in case of a
    // restart.
    fn new() -> Self {
        Self {
            service_name: "service01".into(),
            service_is_healthy: false,
            service_data: 0,
        }
    }

    // This function is up to you to implement.
    //
    // It's recommended to pay extra attention to collection of
    // values. If you iterate over a collection, make sure that labels
    // don't overlap from one metric sample to another.
    fn build_metric_store(&self) -> MetricStore<Metric> {
        let mut store: MetricStore<Metric> = MetricStore::new();

        // Labels can fail to `.build()` because there's a set of
        // characters that are allowed in the label names. You can
        // gracefully handle the error, and set the worker health to
        // 0.
        //
        // Prometheus v3 allows any UTF-8 sequences in the label
        // names, but this library doesn't support it.
        let labels = LabelsBuilder::new()
            .with("service_name", &self.service_name)
            .build()
            .expect("can't build service labels");

        // Store's `.add_value()` supports certain types as a value,
        // in this case a boolean value.
        //
        // See `MetricValue` type for more details.
        store.add_value(Metric::ServiceHealth, &labels, self.service_is_healthy);
        store.add_value(Metric::InternalData, &labels, self.service_data);

        store
    }
}

fn main() {
    let mut state = AppState::new();

    state.service_data += 100;

    // Due to internal structure of MetricStore, the order of rendered
    // metrics is always consistent.
    println!(
        "{}",
        state
            .build_metric_store()
            .render_into_metrics(Some("namespace"))
    );
}
