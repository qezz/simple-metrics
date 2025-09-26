#[macro_export]
macro_rules! metric_def {
    ($name:literal, $help:literal, $metric_type:expr) => {{
        const _: () = {
            if !$crate::metric_def::validate_metric_name($name) {
                panic!(concat!(
                    "Invalid metric name: '",
                    $name,
                    "'. Should match: [a-zA-Z_:][a-zA-Z0-9_:]*"
                ));
            }
        };
        $crate::MetricDef::new_unchecked($name, $help.to_string(), $metric_type)
    }};
}

#[macro_export]
macro_rules! gauge {
    ($name:literal, $help:literal) => {{
        $crate::metric_def!($name, $help, $crate::MetricType::Gauge)
    }};
}

#[cfg(test)]
mod tests {
    #[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub enum Metric {
        Health,
        Gauge,
    }

    impl crate::ToMetricDef for Metric {
        fn to_metric_def(&self) -> crate::MetricDef {
            match self {
                Metric::Health => {
                    metric_def!(
                        "worker_health:beep_BOOP",
                        "worker health",
                        crate::MetricType::Gauge
                    )
                }
                Metric::Gauge => {
                    gauge!("worker_health", "worker health")
                }
            }
        }
    }

    #[test]
    fn smoke() {
        let mut store = crate::MetricStore::new();
        store.add_value(Metric::Health, &crate::Labels::new(), true);
        store.add_value(Metric::Gauge, &crate::Labels::new(), true);
    }
}
