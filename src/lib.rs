use std::collections::HashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

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
    labels: HashMap<String, String>,
    value: Box<dyn RenderableValue>,
}

impl Sample {
    pub fn new<T: RenderableValue + 'static>(labels: &HashMap<String, String>, value: T) -> Self {
        Self {
            labels: labels.clone(),
            value: Box::new(value),
        }
    }
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut labels = self
            .labels
            .iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v))
            .collect::<Vec<String>>();

        // NOTE: Sorting labels may be expensive, but it provides
        // consistency.
        labels.sort();

        let labels = labels.join(",");

        write!(f, "{{{}}} {}", labels, self.value.render())
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

#[derive(Clone, Debug)]
pub enum MetricType {
    /// A counter is a cumulative metric that represents a single
    /// monotonically increasing counter whose value can only increase
    /// or be reset to zero on restart.
    ///
    /// This library doesn't distinguish between a counter and other
    /// metric types. It's your responsibility to ensure that the
    /// counter only increases.
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

impl<K: ToMetricDef> RenderIntoMetrics for HashMap<K, Vec<Sample>> {
    fn render_into_metrics(&self) -> String {
        let mut all_metrics: Vec<String> = Vec::with_capacity(self.keys().len());

        for (m, samples) in self {
            let metric_def = m.to_metric_def();

            let mut metrics = vec![];

            for s in samples {
                metrics.push(format!("{}{}", metric_def.name, s))
            }

            // TODO make sure no same labels exist?
            // TODO: match labels with the regexp
            // TODO: make sure there are no control characters in the labels values

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

    #[derive(Eq, Hash, PartialEq)]
    pub enum CosmosMetric {
        WorkerHealth,
        ChainHeight,
        SomeDeltaShit,
    }

    impl ToMetricDef for CosmosMetric {
        fn to_metric_def(&self) -> MetricDef {
            match self {
                CosmosMetric::WorkerHealth => MetricDef::new(
                    "worker_health",
                    "worker health",
                    MetricType::Gauge,
                ).unwrap(),
                CosmosMetric::ChainHeight => MetricDef::new(
                    "cosmos_height",
                    "cosmos height",
                    MetricType::Gauge,
                ).unwrap(),
                CosmosMetric::SomeDeltaShit => MetricDef::new(
                    "cosmos_delta",
                    "cosmos delta",
                    MetricType::Gauge,
                ).unwrap(),
            }
        }
    }

    #[test]
    fn simple() {
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

        let mut samples: HashMap<CosmosMetric, Vec<Sample>> = HashMap::new();
        samples.insert(CosmosMetric::ChainHeight, vec![]);
        samples.insert(CosmosMetric::WorkerHealth, vec![]);
        samples.insert(CosmosMetric::SomeDeltaShit, vec![]);

        for s in states {
            let mut common = HashMap::new();
            common.insert("name".to_string(), s.name);

            if let Some(v) = samples.get_mut(&CosmosMetric::WorkerHealth) {
                v.push(Sample::new(&common, if s.health { 1 } else { 0 }))
            }

            if let Some(v) = samples.get_mut(&CosmosMetric::ChainHeight) {
                let mut lbs = HashMap::new();
                lbs.insert("client".to_string(), s.client.clone());
                lbs.extend(common.clone());

                v.push(Sample::new(&lbs, s.height))
            }

            if let Some(v) = samples.get_mut(&CosmosMetric::SomeDeltaShit) {
                let mut lbs = HashMap::new();
                lbs.insert("client".to_string(), s.client);
                lbs.extend(common);

                v.push(Sample::new(&lbs, s.delta));
                v.push(Sample::new(&lbs, 1.0 - s.delta));
            }
        }

        let actual = samples.render_into_metrics();

        println!("{}", actual);

        let expected = "";
        assert_eq!(actual, expected);
    }
}
