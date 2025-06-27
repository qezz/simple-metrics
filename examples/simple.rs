use simple_metrics::{LabelsBuilder, MetricDef, MetricStore, RenderIntoMetrics, ToMetricDef};

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
}

// `ToMetricDef` should be implemented to comply with `MetricStore<T>`
// requirements.
//
// It's recommended to use exhaustive matching so the compiler catches
// a bug if you forget to add a new variant to the `Metric` enum.
impl ToMetricDef for Metric {
    fn to_metric_def(&self) -> simple_metrics::MetricDef {
        match self {
            Metric::ServiceHealth => {
                MetricDef::gauge("service_health", "service health (1 or 0)").unwrap()
            }
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

        store
    }
}

fn main() {
    let state = AppState::new();

    // Due to internal structure of MetricStore, the order of rendered
    // metrics is always consistent.
    println!(
        "{}",
        state
            .build_metric_store()
            .render_into_metrics(Some("namespace"))
    );
}
