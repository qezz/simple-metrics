use std::{collections::BTreeMap, hash::Hash};

use crate::{Labels, MetricValue, RenderIntoMetrics, Sample, ToMetricDef};

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
    pub fn add_value<V: Into<MetricValue>>(&mut self, to_metric: K, labels: &Labels, value: V) {
        let sample = Sample::new(labels, value);
        self.add_sample(to_metric, sample);
    }

    /// Add value to metric if it's `Some(..)`, skip if it's `None`
    pub fn maybe_add_value<V: Into<MetricValue>>(
        &mut self,
        to_metric: K,
        labels: &Labels,
        maybe_value: Option<V>,
    ) {
        if let Some(value) = maybe_value {
            self.add_value(to_metric, labels, value)
        }
    }

    /// Add values for multiple metrics with the same set of labels.
    ///
    /// The usage is a bit different from the usual `.add_value()`
    /// method. Since there are no heterogeneous collections in Rust,
    /// the type of the metric value should be the same for all
    /// metrics. A workaround is to call `.into()` on each value.
    ///
    /// # Examples
    /// ```rust
    /// use simple_metrics::{LabelsBuilder, MetricDef, MetricStore, MetricType, ToMetricDef};
    ///
    /// #[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
    /// pub enum Metric {
    ///     A,
    ///     B,
    ///     C,
    /// }
    ///
    /// impl ToMetricDef for Metric {
    ///     fn to_metric_def(&self) -> MetricDef {
    ///         match self {
    ///             Metric::A => MetricDef::new("stub_a", "a", MetricType::Gauge).unwrap(),
    ///             Metric::B => MetricDef::new("stub_b", "b", MetricType::Gauge).unwrap(),
    ///             Metric::C => MetricDef::new("stub_c", "c", MetricType::Gauge).unwrap(),
    ///         }
    ///     }
    /// }
    ///
    /// let mut store: MetricStore<Metric> = MetricStore::new();
    /// let common = LabelsBuilder::new().with("hello", "world").build().unwrap();
    ///
    /// store.add_with_common_labels(
    ///     &common,
    ///     &[
    ///         (Metric::A, true.into()),
    ///         (Metric::B, 1000.into()),
    ///         (Metric::C, (-123.456).into()),
    ///     ],
    /// );
    ///
    /// ```
    pub fn add_with_common_labels(&mut self, labels: &Labels, metrics: &[(K, MetricValue)]) {
        for (to_metric, value) in metrics {
            self.add_value(to_metric.clone(), labels, value.clone())
        }
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

    /// Include static labels to all samples.
    ///
    /// It is a destructive operation, the static labels will be
    /// emptied after this operation.
    ///
    /// Useful if you need to merge multiple MetricStore instances
    /// together.
    pub fn bake_static_labels(&mut self) {
        for samples in self.samples.values_mut() {
            for s in samples {
                s.labels.extend(self.static_labels.clone());
            }
        }

        self.static_labels = Labels::new();
    }

    pub fn extend_samples(&mut self, other: &MetricStore<K>) {
        self.samples.extend(other.clone().samples)
    }
}

impl<K: ToMetricDef> RenderIntoMetrics for MetricStore<K> {
    fn render_into_metrics(&self, namespace: Option<&str>) -> String {
        let mut all_metrics: Vec<String> = Vec::with_capacity(self.samples.keys().len());

        for (m, samples) in &self.samples {
            let metric_def = m.to_metric_def();

            let mut metrics = vec![];

            for s in samples {
                let mut labels: Labels = s.labels.clone();
                labels.extend(self.static_labels.clone());

                let rendered = match namespace {
                    Some(ref ns) => {
                        format!(
                            "{}_{}{{{}}} {}",
                            ns,
                            metric_def.name,
                            labels.render(),
                            s.value.render()
                        )
                    }
                    None => {
                        format!(
                            "{}{{{}}} {}",
                            metric_def.name,
                            labels.render(),
                            s.value.render()
                        )
                    }
                };

                metrics.push(rendered);
            }

            // TODO make sure no same labels exist?
            // TODO make sure there are no control characters in the labels values

            let rendered = match namespace {
                Some(ref ns) => {
                    format!(
                        "# HELP {}_{} {}\n# TYPE {}_{} {}\n{}\n",
                        ns,
                        metric_def.name,
                        metric_def.help,
                        ns,
                        metric_def.name,
                        metric_def.metric_type,
                        metrics.join("\n")
                    )
                }
                None => {
                    format!(
                        "# HELP {} {}\n# TYPE {} {}\n{}\n",
                        metric_def.name,
                        metric_def.help,
                        metric_def.name,
                        metric_def.metric_type,
                        metrics.join("\n")
                    )
                }
            };
            all_metrics.push(rendered);
        }

        all_metrics.join("\n")
    }
}

impl<K: ToMetricDef> RenderIntoMetrics for BTreeMap<K, Vec<Sample>> {
    fn render_into_metrics(&self, namespace: Option<&str>) -> String {
        let len = self.len();
        let mut all_metrics: Vec<String> = Vec::with_capacity(len);

        for (m, samples) in self {
            let metric_def = m.to_metric_def();

            let mut metrics = vec![];

            for s in samples {
                let rendered = match namespace {
                    Some(ref ns) => {
                        format!(
                            "{}_{}{{{}}} {}",
                            ns,
                            metric_def.name,
                            s.labels.render(),
                            s.value.render()
                        )
                    }
                    None => {
                        format!(
                            "{}{{{}}} {}",
                            metric_def.name,
                            s.labels.render(),
                            s.value.render()
                        )
                    }
                };

                metrics.push(rendered);
            }

            // TODO make sure no same labels exist?
            // TODO make sure there are no control characters in the labels values

            let rendered = match namespace {
                Some(ref ns) => {
                    format!(
                        "# HELP {}_{} {}\n# TYPE {}_{} {}\n{}\n",
                        ns,
                        metric_def.name,
                        metric_def.help,
                        ns,
                        metric_def.name,
                        metric_def.metric_type,
                        metrics.join("\n")
                    )
                }
                None => {
                    format!(
                        "# HELP {} {}\n# TYPE {} {}\n{}\n",
                        metric_def.name,
                        metric_def.help,
                        metric_def.name,
                        metric_def.metric_type,
                        metrics.join("\n")
                    )
                }
            };

            all_metrics.push(rendered);
        }

        all_metrics.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        labels_builder::LabelsBuilder,
        store::MetricStore,
        tests::{ServiceMetric, SimpleState},
        RenderIntoMetrics, Sample,
    };

    #[test]
    fn bake_static_labels() {
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

        let static_labels = LabelsBuilder::new()
            .with("process", "simple-metrics")
            .build()
            .unwrap();

        let mut store: MetricStore<ServiceMetric> =
            MetricStore::new().with_static_labels(static_labels);

        for s in states {
            let common_builder = LabelsBuilder::from([("name", s.name)]);
            let common = common_builder.build().expect("can't build labels");

            store.add_sample(ServiceMetric::WorkerHealth, Sample::new(&common, s.health));
            store.add_value(ServiceMetric::ServiceHeight, &common, s.height)
        }

        let expected = store.clone().render_into_metrics(None);

        assert!(!store.static_labels.is_empty());
        store.bake_static_labels();

        let actual = store.render_into_metrics(None);

        assert_eq!(actual, expected);
        assert!(store.static_labels.is_empty());

        // Calling `bake_static_labels()` again should not change the output.
        store.bake_static_labels();
        let actual = store.render_into_metrics(None);
        assert_eq!(actual, expected);
    }
}
