#[macro_export]
macro_rules! metric_def {
    ($name:literal, $help:literal, $metric_type:expr) => {{
        const _: () = {
            if !$crate::metric_def::validate_metric_name($name) {
                panic!(concat!(
                    "Invalid metric name: ",
                    $name,
                    ". Should match: [a-zA-Z_:][a-zA-Z0-9_:]*"
                ));
            }
        };
        $crate::MetricDef::new_unchecked($name, $help.to_string(), $metric_type)
    }};
}

#[macro_export]
macro_rules! gauge {
    ($name:literal, $help:literal) => {{
        metric_def!($name, $help, $crate::MetricType::Gauge)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Labels, MetricDef, MetricStore, MetricType, ToMetricDef};

    #[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub enum Metric {
        Health,
        Gauge,
    }

    impl ToMetricDef for Metric {
        fn to_metric_def(&self) -> MetricDef {
            match self {
                Metric::Health => {
                    metric_def!("worker_health", "worker health", MetricType::Gauge)
                }
                Metric::Gauge => {
                    gauge!("worker_health", "worker health")
                }
            }
        }
    }

    #[test]
    fn smoke() {
        let mut store = MetricStore::new();
        store.add_value(Metric::Health, &Labels::new(), true);
        store.add_value(Metric::Gauge, &Labels::new(), true);
    }
}
