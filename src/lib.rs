use regex::Regex;
use std::{
    collections::{btree_map, BTreeMap},
    hash::Hash,
};

lazy_static::lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

/// Internal trait for rendering a collection of metrics into a
/// string.
pub trait RenderIntoMetrics {
    fn render_into_metrics(&self) -> String;
}

/// Internal representation of sample labels
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labels {
    inner: BTreeMap<String, String>,
}

impl Labels {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.insert(key, value);
        self
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
    }

    pub fn iter(&self) -> btree_map::Iter<String, String> {
        self.inner.iter()
    }

    pub fn keys(&self) -> btree_map::Keys<String, String> {
        self.inner.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Labels {
    type Item = (String, String);
    type IntoIter = btree_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Extend<(String, String)> for Labels {
    fn extend<T: IntoIterator<Item = (String, String)>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl<T, U> FromIterator<(T, U)> for Labels
where
    T: Into<String>,
    U: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let mut map = BTreeMap::new();
        for (key, value) in iter {
            map.insert(key.into(), value.into());
        }
        Labels { inner: map }
    }
}

impl<'a, T, U> From<&'a [(T, U)]> for Labels
where
    T: Into<String> + AsRef<str> + 'a,
    U: Into<String> + AsRef<str> + 'a,
{
    fn from(tuples: &'a [(T, U)]) -> Self {
        tuples
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect()
    }
}

impl RenderIntoMetrics for Labels {
    fn render_into_metrics(&self) -> String {
        self.iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v))
            .collect::<Vec<String>>()
            .join(",")
    }
}

pub fn check_labels(labels: &Labels) -> Result<(), Error> {
    for name in labels.keys() {
        if !LABEL_NAME_RE.is_match(name) {
            return Err(Error::InvalidLabelName(name.to_string()));
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub enum MetricValue {
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}

impl MetricValue {
    pub fn render(&self) -> String {
        match self {
            MetricValue::I64(v) => v.to_string(),
            MetricValue::U64(v) => v.to_string(),
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

impl From<i64> for MetricValue {
    fn from(v: i64) -> Self {
        MetricValue::I64(v)
    }
}

impl From<u64> for MetricValue {
    fn from(v: u64) -> Self {
        MetricValue::U64(v)
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
        check_labels(labels)?;

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
        if !METRIC_NAME_RE.is_match(name) {
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

#[derive(Clone, Debug)]
pub struct MetricStore<K: ToMetricDef> {
    static_labels: Labels,
    samples: BTreeMap<K, Vec<Sample>>,
}

impl<K: ToMetricDef + Eq + PartialEq + Hash + Ord> Default for MetricStore<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: ToMetricDef + Eq + PartialEq + Hash + Ord> MetricStore<K> {
    pub fn new() -> Self {
        Self {
            static_labels: Labels::new(),
            samples: BTreeMap::new(),
        }
    }

    /// Add static labels to MetricStore
    ///
    /// These labels will be added to every sample.
    pub fn with_static_labels(self, labels: Labels) -> Self {
        Self {
            static_labels: labels,
            ..self
        }
    }

    /// Add Sample to metric
    pub fn add_sample(&mut self, to_metric: K, sample: Sample) {
        let v = self.samples.entry(to_metric).or_default();
        v.push(sample);
    }

    /// Add value to metric
    pub fn add_value<V: Into<MetricValue>>(
        &mut self,
        to_metric: K,
        labels: &Labels,
        value: V,
    ) -> Result<(), Error> {
        let sample = Sample::new(labels, value)?;
        self.add_sample(to_metric, sample);

        Ok(())
    }

    /// Add value to metric if it's `Some(..)`, skip if it's `None`
    pub fn maybe_add_value<V: Into<MetricValue>>(
        &mut self,
        to_metric: K,
        labels: &Labels,
        maybe_value: Option<V>,
    ) -> Result<(), Error> {
        if let Some(value) = maybe_value {
            self.add_value(to_metric, labels, value)?
        }

        Ok(())
    }

    /// Include static labels to all samples, and return the inner
    /// representation.
    ///
    /// Useful if you need to merge multiple MetricStore instances
    /// together.
    pub fn to_rich_samples(self) -> BTreeMap<K, Vec<Sample>> {
        let mut rich_data = BTreeMap::new();

        for (k, samples) in self.samples {
            let mut new_samples = vec![];

            for s in samples {
                let mut labels = s.labels.clone();
                labels.extend(self.static_labels.clone());

                new_samples.push(Sample {
                    labels,
                    value: s.value,
                });
            }

            rich_data.insert(k, new_samples);
        }

        rich_data
    }
}

impl<K: ToMetricDef> RenderIntoMetrics for MetricStore<K> {
    fn render_into_metrics(&self) -> String {
        let mut all_metrics: Vec<String> = Vec::with_capacity(self.samples.keys().len());

        for (m, samples) in &self.samples {
            let metric_def = m.to_metric_def();

            let mut metrics = vec![];

            for s in samples {
                let mut labels: Labels = s.labels.clone();
                labels.extend(self.static_labels.clone());

                metrics.push(format!(
                    "{}{{{}}} {}",
                    metric_def.name,
                    labels.render_into_metrics(),
                    s.value.render()
                ))
            }

            // TODO make sure no same labels exist?
            // TODO make sure there are no control characters in the labels values

            let rendered = format!(
                "# HELP {} {}\n# TYPE {} {}\n{}\n",
                metric_def.name,
                metric_def.help,
                metric_def.name,
                metric_def.metric_type,
                metrics.join("\n")
            );

            all_metrics.push(rendered);
        }

        all_metrics.join("\n")
    }
}

impl<K: ToMetricDef> RenderIntoMetrics for BTreeMap<K, Vec<Sample>> {
    fn render_into_metrics(&self) -> String {
        let len = self.len();
        let mut all_metrics: Vec<String> = Vec::with_capacity(len);

        for (m, samples) in self {
            let metric_def = m.to_metric_def();

            let mut metrics = vec![];

            for s in samples {
                metrics.push(format!(
                    "{}{{{}}} {}",
                    metric_def.name,
                    s.labels.render_into_metrics(),
                    s.value.render()
                ))
            }

            // TODO make sure no same labels exist?
            // TODO make sure there are no control characters in the labels values

            let rendered = format!(
                "# HELP {} {}\n# TYPE {} {}\n{}\n",
                metric_def.name,
                metric_def.help,
                metric_def.name,
                metric_def.metric_type,
                metrics.join("\n")
            );

            all_metrics.push(rendered);
        }

        all_metrics.join("\n")
    }
}

#[cfg(test)]
mod tests {
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

        let static_labels = Labels::from(&[("process", "simple-metrics")][..]);

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common = Labels::from(&[("name", s.name)][..]);

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

            let lbs_p = lbs.clone().with("type", "pos");
            store
                .add_value(ServiceMetric::ServiceDelta, &lbs_p, s.delta)
                .expect("valid");

            let lbs_n = lbs.clone().with("type", "neg");
            store
                .add_value(ServiceMetric::ServiceDelta, &lbs_n, -s.delta)
                .expect("valid");
        }

        let actual = store.render_into_metrics();
        let expected = r#"# HELP worker_health worker health
# TYPE worker_health gauge
worker_health{name="a",process="simple-metrics"} 1
worker_health{name="b",process="simple-metrics"} 1
worker_health{name="c",process="simple-metrics"} 1
worker_health{name="d",process="simple-metrics"} 0

# HELP service_height service height
# TYPE service_height gauge
service_height{client="woot",name="a",process="simple-metrics"} 100
service_height{client="woot",name="b",process="simple-metrics"} 200
service_height{client="meh",name="c",process="simple-metrics"} 300
service_height{client="meh",name="d",process="simple-metrics"} 0

# HELP service_delta service delta
# TYPE service_delta gauge
service_delta{client="woot",name="a",process="simple-metrics",type="pos"} 1
service_delta{client="woot",name="a",process="simple-metrics",type="neg"} -1
service_delta{client="woot",name="b",process="simple-metrics",type="pos"} 2.2
service_delta{client="woot",name="b",process="simple-metrics",type="neg"} -2.2
service_delta{client="meh",name="c",process="simple-metrics",type="pos"} 3
service_delta{client="meh",name="c",process="simple-metrics",type="neg"} -3
service_delta{client="meh",name="d",process="simple-metrics",type="pos"} 291283791287391300000
service_delta{client="meh",name="d",process="simple-metrics",type="neg"} -291283791287391300000

# HELP service_maybe service maybe
# TYPE service_maybe gauge
service_maybe{client="woot",name="a",process="simple-metrics"} 100
service_maybe{client="woot",name="b",process="simple-metrics"} 100
service_maybe{client="meh",name="c",process="simple-metrics"} 100
"#;
        assert_eq!(actual, expected);
    }

    pub struct SimpleState {
        name: String,
        health: bool,
        height: i64,
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
            let common = Labels::from(&[("name", s.name)][..]);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, s.health).unwrap(),
            );

            store
                .add_value(ServiceMetric::ServiceHeight, &common, s.height)
                .expect("valid");
        }

        let _cloned_store = store.clone();

        let actual = store.render_into_metrics();
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
    fn labels_new() {
        let mut labels_insert = Labels::new();
        labels_insert.insert("hello", "world");
        labels_insert.insert("woot", "meh");
        assert_eq!(2, labels_insert.len());

        let labels_with = Labels::new().with("hello", "world").with("woot", "meh");
        assert_eq!(2, labels_with.len());

        assert_eq!(labels_insert, labels_with);

        let labels_default = Labels::default();
        assert_eq!(0, labels_default.len());
    }

    #[test]
    fn labels_with() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let labels: Labels = tuples.into_iter().collect();
        assert_eq!(3, labels.len());

        let new_labels = labels.clone().with("four", "4");

        let tuples2 = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4")];
        let labels2: Labels = tuples2.into_iter().collect();

        assert_eq!(new_labels, labels2);
    }

    #[test]
    fn labels_from_tuple_list() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let labels: Labels = tuples.into_iter().collect();
        assert_eq!(3, labels.len());

        let labels_from = Labels::from(&tuples[..]);
        assert_eq!(3, labels_from.len());

        assert_eq!(labels, labels_from);

        let labels_from_2 = Labels::from(&[("one", "1"), ("two", "2"), ("three", "3")][..]);
        assert_eq!(labels, labels_from_2);
    }
}
