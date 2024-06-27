use std::collections::HashMap;

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

pub trait RenderIntoMetrics {
    fn render_into_metrics(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetricDef {
    name: String,
    help: String,
    metric_type: MetricType,
}

/// should not fail
pub trait ToMetricDef {
    fn to_metric_def(&self) -> MetricDef;
}

pub enum Error {
    MetricsConvertError(()),
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                CosmosMetric::WorkerHealth => MetricDef {
                    name: "worker_health".to_string(),
                    help: "worker health".to_string(),
                    metric_type: MetricType::Gauge,
                },
                CosmosMetric::ChainHeight => MetricDef {
                    name: "cosmos_height".to_string(),
                    help: "cosmos height".to_string(),
                    metric_type: MetricType::Gauge,
                },
                CosmosMetric::SomeDeltaShit => MetricDef {
                    name: "cosmos_delta".to_string(),
                    help: "cosmos delta".to_string(),
                    metric_type: MetricType::Gauge,
                },
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
