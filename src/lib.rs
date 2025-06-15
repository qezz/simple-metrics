pub mod labels;
pub mod store;

pub use labels::Labels;
pub use store::MetricStore;

/// Internal trait for rendering a collection of metrics into a
/// string.
pub trait RenderIntoMetrics {
    fn render_into_metrics(&self, namespace: Option<&str>) -> String;
}

#[derive(Debug, Clone)]
pub enum MetricValue {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    F64(f64),
    Bool(bool),
}

impl MetricValue {
    pub fn render(&self) -> String {
        match self {
            MetricValue::I32(v) => v.to_string(),
            MetricValue::I64(v) => v.to_string(),
            MetricValue::I128(v) => v.to_string(),
            MetricValue::U32(v) => v.to_string(),
            MetricValue::U64(v) => v.to_string(),
            MetricValue::U128(v) => v.to_string(),
            MetricValue::F64(v) => format!("{}", v),
            MetricValue::Bool(v) => {
                if *v {
                    1_i64.to_string()
                } else {
                    0_i64.to_string()
                }
            }
        }
    }
}

impl From<i32> for MetricValue {
    fn from(v: i32) -> Self {
        MetricValue::I32(v)
    }
}

impl From<i64> for MetricValue {
    fn from(v: i64) -> Self {
        MetricValue::I64(v)
    }
}

impl From<i128> for MetricValue {
    fn from(v: i128) -> Self {
        MetricValue::I128(v)
    }
}

impl From<u32> for MetricValue {
    fn from(v: u32) -> Self {
        MetricValue::U32(v)
    }
}

impl From<u64> for MetricValue {
    fn from(v: u64) -> Self {
        MetricValue::U64(v)
    }
}

impl From<u128> for MetricValue {
    fn from(v: u128) -> Self {
        MetricValue::U128(v)
    }
}

impl From<f64> for MetricValue {
    fn from(v: f64) -> Self {
        MetricValue::F64(v)
    }
}

impl From<bool> for MetricValue {
    fn from(v: bool) -> Self {
        MetricValue::Bool(v)
    }
}

/// Sample holds a single measurement of metrics
#[derive(Debug, Clone)]
pub struct Sample {
    labels: Labels,
    value: MetricValue,
}

impl Sample {
    pub fn new<T: Into<MetricValue>>(labels: &Labels, value: T) -> Result<Self, Error> {
        labels::check_labels(labels)?;

        Ok(Self {
            labels: labels.clone(),
            value: value.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    /// InvalidMetricName means the metric name doesn't comply with
    /// the Prometheus data model
    ///
    /// For more details, see
    /// https://prometheus.io/docs/concepts/data_model/
    InvalidMetricName(String),

    /// InvalidLabelName means the label name doesn't comply with the
    /// Prometheus data model.
    ///
    /// For more details, see
    /// https://prometheus.io/docs/concepts/data_model/
    InvalidLabelName(String),
}

/// Metric type
///
/// This library doesn't distinguish between a counter, gauge and
/// histogram metric types. It's your responsibility to ensure that
/// the counter only increases and histogram provides consistent data.
#[derive(Clone, Debug)]
pub enum MetricType {
    /// A counter is a cumulative metric that represents a single
    /// monotonically increasing counter whose value can only increase
    /// or be reset to zero on restart.
    // TODO: Make counter work only with Option<u64>?
    Counter,

    /// A gauge is a metric that represents a single numerical value
    /// that can arbitrarily go up and down.
    Gauge,

    /// A histogram samples observations (usually things like request
    /// durations or response sizes) and counts them in configurable
    /// buckets.
    Histogram,
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
        }
    }
}

#[allow(clippy::manual_is_ascii_check)]
#[inline(always)]
pub fn is_valid_metric_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if let Some(&first) = bytes.first() {
        if !((b'A'..=b'Z').contains(&first)
            || (b'a'..=b'z').contains(&first)
            || first == b'_'
            || first == b':')
        {
            return false;
        }

        bytes.iter().skip(1).all(|&b| {
            (b'A'..=b'Z').contains(&b)
                || (b'a'..=b'z').contains(&b)
                || (b'0'..=b'9').contains(&b)
                || b == b'_'
                || b == b':'
        })
    } else {
        false
    }
}

/// MetricDef represents a metric definition.
///
/// A metric definition consists of a name, a help string, and a
/// metric type. The samples are dynaically added on the rendering
/// phase.
#[derive(Clone, Debug)]
pub struct MetricDef {
    name: String,
    help: String,
    metric_type: MetricType,
}

/// Metric definition, including its name, help string, and metric type
impl MetricDef {
    pub fn new(name: &str, help: &str, metric_type: MetricType) -> Result<Self, Error> {
        if !is_valid_metric_name(name) {
            return Err(Error::InvalidMetricName(name.to_string()));
        }

        Ok(Self {
            name: name.to_string(),
            help: help.to_string(),
            metric_type,
        })
    }
}

// TODO: Replace with From<X> for MetricDef?
pub trait ToMetricDef: Clone {
    fn to_metric_def(&self) -> MetricDef;
}

#[cfg(test)]
mod tests {
    use crate::store::MetricStore;

    use super::*;

    pub struct State {
        name: String,
        client: String,
        health: bool,
        height: i64,
        delta: f64,
        maybe: Option<i64>,
    }

    #[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub enum ServiceMetric {
        WorkerHealth,
        ServiceHeight,
        ServiceDelta,
        Maybe,
        Maybe2,
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
                ServiceMetric::ServiceDelta => {
                    MetricDef::new("service_delta", "service delta", MetricType::Gauge).unwrap()
                }
                ServiceMetric::Maybe => {
                    MetricDef::new("service_maybe", "service maybe", MetricType::Gauge).unwrap()
                }
                ServiceMetric::Maybe2 => {
                    MetricDef::new("service_maybe2", "service maybe2", MetricType::Gauge).unwrap()
                }
            }
        }
    }

    #[test]
    fn complex() {
        let states = vec![
            State {
                name: "a".into(),
                client: "woot".into(),
                health: true,
                height: 100,
                delta: 1.0,
                maybe: Some(100),
            },
            State {
                name: "b".into(),
                client: "woot".into(),
                health: true,
                height: 200,
                delta: 2.2,
                maybe: Some(100),
            },
            State {
                name: "c".into(),
                client: "meh".into(),
                health: true,
                height: 300,
                delta: 3.0,
                maybe: Some(100),
            },
            State {
                name: "d".into(),
                client: "meh".into(),
                health: false,
                height: 0,
                delta: 291283791287391287391.123,
                maybe: None,
            },
        ];

        let static_labels = Labels::from([("process", "simple-metrics")]);

        let namespace = String::from("test_exporter");
        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common = Labels::from([("name", s.name)]);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, s.health).unwrap(),
            );

            let lbs = common.clone().with("client", s.client);

            store
                .add_value(ServiceMetric::ServiceHeight, &lbs, s.height)
                .expect("valid");

            if let Some(maybe) = s.maybe {
                store
                    .add_value(ServiceMetric::Maybe, &lbs, maybe)
                    .expect("valid");
            }

            store
                .maybe_add_value(ServiceMetric::Maybe2, &lbs, s.maybe)
                .expect("valid");

            let lbs_p = lbs.clone().with("type", "pos");
            store
                .add_value(ServiceMetric::ServiceDelta, &lbs_p, s.delta)
                .expect("valid");

            let lbs_n = lbs.clone().with("type", "neg");
            store
                .add_value(ServiceMetric::ServiceDelta, &lbs_n, -s.delta)
                .expect("valid");
        }

        let actual = store.render_into_metrics(Some(&namespace));
        let expected = r#"# HELP test_exporter_worker_health worker health
# TYPE test_exporter_worker_health gauge
test_exporter_worker_health{name="a",process="simple-metrics"} 1
test_exporter_worker_health{name="b",process="simple-metrics"} 1
test_exporter_worker_health{name="c",process="simple-metrics"} 1
test_exporter_worker_health{name="d",process="simple-metrics"} 0

# HELP test_exporter_service_height service height
# TYPE test_exporter_service_height gauge
test_exporter_service_height{client="woot",name="a",process="simple-metrics"} 100
test_exporter_service_height{client="woot",name="b",process="simple-metrics"} 200
test_exporter_service_height{client="meh",name="c",process="simple-metrics"} 300
test_exporter_service_height{client="meh",name="d",process="simple-metrics"} 0

# HELP test_exporter_service_delta service delta
# TYPE test_exporter_service_delta gauge
test_exporter_service_delta{client="woot",name="a",process="simple-metrics",type="pos"} 1
test_exporter_service_delta{client="woot",name="a",process="simple-metrics",type="neg"} -1
test_exporter_service_delta{client="woot",name="b",process="simple-metrics",type="pos"} 2.2
test_exporter_service_delta{client="woot",name="b",process="simple-metrics",type="neg"} -2.2
test_exporter_service_delta{client="meh",name="c",process="simple-metrics",type="pos"} 3
test_exporter_service_delta{client="meh",name="c",process="simple-metrics",type="neg"} -3
test_exporter_service_delta{client="meh",name="d",process="simple-metrics",type="pos"} 291283791287391300000
test_exporter_service_delta{client="meh",name="d",process="simple-metrics",type="neg"} -291283791287391300000

# HELP test_exporter_service_maybe service maybe
# TYPE test_exporter_service_maybe gauge
test_exporter_service_maybe{client="woot",name="a",process="simple-metrics"} 100
test_exporter_service_maybe{client="woot",name="b",process="simple-metrics"} 100
test_exporter_service_maybe{client="meh",name="c",process="simple-metrics"} 100

# HELP test_exporter_service_maybe2 service maybe2
# TYPE test_exporter_service_maybe2 gauge
test_exporter_service_maybe2{client="woot",name="a",process="simple-metrics"} 100
test_exporter_service_maybe2{client="woot",name="b",process="simple-metrics"} 100
test_exporter_service_maybe2{client="meh",name="c",process="simple-metrics"} 100
"#;
        assert_eq!(actual, expected);
    }

    pub struct SimpleState {
        pub name: String,
        pub health: bool,
        pub height: i64,
    }

    #[test]
    fn simple() {
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

        let mut static_labels = Labels::new();
        static_labels.insert("process", "simple-metrics");

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common = Labels::from([("name", s.name)]);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, s.health).unwrap(),
            );

            store
                .add_value(ServiceMetric::ServiceHeight, &common, s.height)
                .expect("valid");
        }

        let _cloned_store = store.clone();

        let actual = store.render_into_metrics(None);
        println!("{}", actual);

        let expected = r#"# HELP worker_health worker health
# TYPE worker_health gauge
worker_health{name="a",process="simple-metrics"} 1
worker_health{name="b",process="simple-metrics"} 0

# HELP service_height service height
# TYPE service_height gauge
service_height{name="a",process="simple-metrics"} 100
service_height{name="b",process="simple-metrics"} 200
"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn simple_with_namespace() {
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

        let mut static_labels = Labels::new();
        static_labels.insert("process", "simple-metrics");

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common = Labels::from([("name", s.name)]);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, s.health).unwrap(),
            );

            store
                .add_value(ServiceMetric::ServiceHeight, &common, s.height)
                .expect("valid");
        }

        let _cloned_store = store.clone();

        let actual = store.render_into_metrics(Some("namespace"));
        println!("{}", actual);

        let expected = r#"# HELP namespace_worker_health worker health
# TYPE namespace_worker_health gauge
namespace_worker_health{name="a",process="simple-metrics"} 1
namespace_worker_health{name="b",process="simple-metrics"} 0

# HELP namespace_service_height service height
# TYPE namespace_service_height gauge
namespace_service_height{name="a",process="simple-metrics"} 100
namespace_service_height{name="b",process="simple-metrics"} 200
"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn simple_escape_label_values() {
        let states = vec![
            SimpleState {
                name: r#""a""#.into(),
                health: true,
                height: 100,
            },
            SimpleState {
                name: r#""b""#.into(),
                health: false,
                height: 200,
            },
        ];

        let mut static_labels = Labels::new();
        static_labels.insert("process", "simple-metrics");

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common = Labels::from([("name", s.name)]);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, s.health).unwrap(),
            );
        }

        let _cloned_store = store.clone();

        let actual = store.render_into_metrics(Some("namespace"));
        println!("{}", actual);

        let expected = r#"# HELP namespace_worker_health worker health
# TYPE namespace_worker_health gauge
namespace_worker_health{name="\"a\"",process="simple-metrics"} 1
namespace_worker_health{name="\"b\"",process="simple-metrics"} 0
"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_metric_name() {
        let starting_with_a_digit = MetricDef::new(
            "1starting_with_a_digit",
            "starting with a digit",
            MetricType::Gauge,
        );
        assert!(starting_with_a_digit.is_err());

        let has_spaces = MetricDef::new(
            "service health",
            "has spaces in the metric name",
            MetricType::Gauge,
        );
        assert!(has_spaces.is_err());

        let has_weird_chars =
            MetricDef::new("dobrý_den", "has non ascii characters", MetricType::Gauge);
        assert!(has_weird_chars.is_err());
    }
}
