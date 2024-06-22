use std::collections::HashMap;

use regex::Regex;

lazy_static::lazy_static! {
    static ref METRIC_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap();
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

/// IntoMetric should be implemented for the application state or part
/// of it, in order to produce MetricGroup, which will be later turned
/// into rendered Prometheus metrics.
pub trait IntoMetric<T> {
    fn into_metric_group(self) -> Result<MetricGroup<T>, Error>;
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

/// Prometheus metric type
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// RenderIntoMetrics is an internal trait, which is implemented for
/// MetricGroup to allow ad-hoc rendering of metrics.
pub trait RenderIntoMetrics {
    fn render_into_metrics(self) -> String;
}

/// RenderSample is an internal trait, which is implemented for
/// Sample to allow ad-hoc rendering of metrics.
pub trait RenderSample {
    fn render_sample(&self) -> String;
}

/// Sample holds a single measurement of metrics
#[derive(Debug, PartialEq, Eq)]
pub struct Sample<T> {
    pub labels: HashMap<String, String>,
    pub value: T,
}

impl<T: std::fmt::Display> RenderSample for Sample<T> {
    fn render_sample(&self) -> String {
        let mut labels = self
            .labels
            .iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v))
            .collect::<Vec<String>>();

        // NOTE: Sorting labels may be expensive, but it provides
        // consistency.
        labels.sort();

        let labels = labels.join(",");

        format!("{{{}}} {}", labels, self.value)
    }
}

pub fn check_labels(labels: &HashMap<String, String>) -> Result<(), Error> {
    for name in labels.keys() {
        if !LABEL_NAME_RE.is_match(name) {
            return Err(Error::InvalidLabelName(name.to_string()));
        }
    }

    Ok(())
}

/// MetricGroup is the internal representation of Prometheus metrics.
/// An implementation exists to turn MetricGroup into String.
///
/// Type parameter `T` repsesents the sample's value.
#[derive(Debug, PartialEq, Eq)]
pub struct MetricGroup<T> {
    name: String,
    help: String,
    metric_type: MetricType,
    samples: Vec<Sample<T>>,
}

impl<T> MetricGroup<T> {
    /// Creates a MetricGroup without samples. To add samples, use
    /// either `with_samples()` or `add()`
    ///
    /// User has to check for error when constructing a new MetricGroup.
    ///
    /// Returns `Error::InvalidMetricName` when the metric name is invalid.
    pub fn new(name: &str, help: &str, typ: MetricType) -> Result<Self, Error> {
        if !METRIC_NAME_RE.is_match(name) {
            return Err(Error::InvalidMetricName(name.to_string()));
        }

        Ok(Self {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: typ,
            samples: vec![],
        })
    }

    /// Consumes `self` and `samples`, returns MetricGroup with the
    /// provided samples.
    ///
    /// Returns `Error::InvalidLabelName` when any of the labels is invalid.
    pub fn with_samples(self, samples: Vec<Sample<T>>) -> Result<Self, Error> {
        for metric in &samples {
            check_labels(&metric.labels)?;
        }

        Ok(Self { samples, ..self })
    }

    /// Mutates `self`, returns `()` if a new sample was added successful.
    ///
    /// Returns `Error::InvalidLabelName` when any of the labels is invalid.
    pub fn add(&mut self, labels: HashMap<String, String>, value: T) -> Result<(), Error> {
        check_labels(&labels)?;

        self.samples.push(Sample { labels, value });

        Ok(())
    }
}

impl RenderIntoMetrics for MetricGroup<i64> {
    fn render_into_metrics(self) -> String {
        let mut rendered_samples = self
            .samples
            .into_iter()
            .map(|v| format!("{}{}", self.name, v.render_sample()))
            .collect::<Vec<String>>();

        // NOTE: Sorting samples may be expensive, but it provides
        // consistency.
        rendered_samples.sort();

        let metrics = rendered_samples.join("\n");

        format!(
            r#"# HELP {} {}
# TYPE {} {}
{}
"#,
            self.name, self.help, self.name, self.metric_type, metrics
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl IntoMetric<i64> for HashMap<String, i64> {
        fn into_metric_group(self) -> Result<MetricGroup<i64>, Error> {
            let mut common_labels = HashMap::new();
            common_labels.insert("common".to_string(), "label".to_string());

            let metrics: Vec<Sample<i64>> = self
                .into_iter()
                .map(|(k, v)| {
                    let mut labels = HashMap::new();
                    labels.insert("name".into(), k);

                    labels.extend(common_labels.clone());

                    Sample { labels, value: v }
                })
                .collect();

            let maybe_mg =
                MetricGroup::new("hashmap_test", "this is a hashmap test", MetricType::Gauge)?;

            let with_metrics = maybe_mg.with_samples(metrics)?;

            Ok(with_metrics)
        }
    }

    #[test]
    fn test_hashmap_into_metrics() {
        let mut hm: HashMap<String, i64> = HashMap::new();
        hm.insert("test1".into(), 100);
        hm.insert("test2".into(), 200);

        let mg = hm.into_metric_group().unwrap();

        let expected = "\
# HELP hashmap_test this is a hashmap test
# TYPE hashmap_test gauge
hashmap_test{common=\"label\",name=\"test1\"} 100
hashmap_test{common=\"label\",name=\"test2\"} 200
";

        assert_eq!(mg.render_into_metrics(), expected.to_string())
    }

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

    #[test]
    fn test_data_into_metrics() {
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
    }

    #[test]
    fn test_data_empty_samples() {
        let data = Data {
            name: "test".into(),
            chain_id: "pacific-1".into(),
            samples: vec![],
        };

        let mg = data.into_metric_group().unwrap();
        assert!(mg.samples.is_empty());

        let expected = "\
# HELP data_height this data height test
# TYPE data_height gauge

";

        assert_eq!(mg.render_into_metrics(), expected.to_string())
    }
}
