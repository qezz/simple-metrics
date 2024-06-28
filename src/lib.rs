use regex::Regex;
use std::{collections::BTreeMap, hash::Hash};

lazy_static::lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

type Labels = BTreeMap<String, String>;

/// A trait for rendering a Prometheus metric value into a string.
pub trait RenderableValue {
    fn render(&self) -> String;
}

impl RenderableValue for i64 {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl RenderableValue for f64 {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

/// Sample holds a single measurement of metrics
pub struct Sample {
    labels: Labels,
    value: Box<dyn RenderableValue>,
}

impl Sample {
    pub fn new<T: RenderableValue + 'static>(labels: &Labels, value: T) -> Self {
        Self {
            labels: labels.clone(),
            value: Box::new(value),
        }
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

/// Internal trait for rendering a collection of metrics into a
/// string.
pub trait RenderIntoMetrics {
    fn render_into_metrics(&self) -> String;
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

pub trait ToMetricDef {
    fn to_metric_def(&self) -> MetricDef;
}

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

    pub fn with_static_labels(self, labels: Labels) -> Self {
        Self {
            static_labels: labels,
            ..self
        }
    }

    pub fn add_sample(&mut self, to_metric: K, sample: Sample) {
        let v = self.samples.entry(to_metric).or_default();
        v.push(sample);
    }

    pub fn add_value<V: RenderableValue + 'static>(
        &mut self,
        to_metric: K,
        labels: &Labels,
        value: V,
    ) {
        let sample = Sample::new(labels, value);

        self.add_sample(to_metric, sample);
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

                let r = format!(
                    "{}{{{}}} {}",
                    metric_def.name,
                    labels.render_into_metrics(),
                    s.value.render()
                );

                metrics.push(r)
            }

            // TODO make sure no same labels exist?
            // TODO match labels with the regexp
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
    }

    #[derive(Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub enum ServiceMetric {
        WorkerHealth,
        ChainHeight,
        SomeDeltaShit,
    }

    impl ToMetricDef for ServiceMetric {
        fn to_metric_def(&self) -> MetricDef {
            match self {
                ServiceMetric::WorkerHealth => {
                    MetricDef::new("worker_health", "worker health", MetricType::Gauge).unwrap()
                }
                ServiceMetric::ChainHeight => {
                    MetricDef::new("service_height", "service height", MetricType::Gauge).unwrap()
                }
                ServiceMetric::SomeDeltaShit => {
                    MetricDef::new("service_delta", "service delta", MetricType::Gauge).unwrap()
                }
            }
        }
    }

    impl From<ServiceMetric> for MetricDef {
        fn from(value: ServiceMetric) -> Self {
            match value {
                ServiceMetric::WorkerHealth => {
                    MetricDef::new("worker_health", "worker health", MetricType::Gauge).unwrap()
                }
                ServiceMetric::ChainHeight => {
                    MetricDef::new("service_height", "service height", MetricType::Gauge).unwrap()
                }
                ServiceMetric::SomeDeltaShit => {
                    MetricDef::new("service_delta", "service delta", MetricType::Gauge).unwrap()
                }
            }
        }
    }

    #[test]
    fn simple2() {
        let states = vec![
            State {
                name: "a".into(),
                client: "woot".into(),
                health: true,
                height: 100,
                delta: 1.0,
            },
            State {
                name: "b".into(),
                client: "woot".into(),
                health: true,
                height: 200,
                delta: 2.2,
            },
            State {
                name: "c".into(),
                client: "meh".into(),
                health: true,
                height: 300,
                delta: 3.0,
            },
            State {
                name: "d".into(),
                client: "meh".into(),
                health: false,
                height: 0,
                delta: 291283791287391287391.123,
            },
        ];

        let mut static_labels = BTreeMap::new();
        static_labels.insert("process".into(), "simple-metrics".into());

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let mut common = BTreeMap::new();
            common.insert("name".to_string(), s.name);

            store.add_sample(
                ServiceMetric::WorkerHealth,
                Sample::new(&common, if s.health { 1 } else { 0 }),
            );

            let mut lbs = BTreeMap::new();
            lbs.insert("client".to_string(), s.client.clone());
            lbs.extend(common.clone());

            store.add_value(ServiceMetric::ChainHeight, &lbs, s.height);

            let mut lbs_s = lbs.clone();
            lbs_s.insert("type".into(), "pos".into());
            store.add_value(ServiceMetric::SomeDeltaShit, &lbs_s, s.delta);

            let mut lbs_s = lbs.clone();
            lbs_s.insert("type".into(), "neg".into());
            store.add_value(ServiceMetric::SomeDeltaShit, &lbs_s, -s.delta);
        }

        let actual = store.render_into_metrics();

        println!("{}", actual);

        let expected = "";
        assert_eq!(actual, expected);
    }
}
