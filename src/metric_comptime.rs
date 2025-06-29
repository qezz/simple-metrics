use crate::{MetricDef, MetricName, MetricType};

impl MetricDef {
    pub const fn new_static(name: &'static str, help: String, metric_type: MetricType) -> Self {
        Self {
            name: MetricName::Static(name),
            help,
            metric_type,
        }
    }
}

/// Validates metric name at compile time
pub const fn validate_metric_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    let len = bytes.len();

    // Check length constraints
    if len == 0 || len > 255 {
        return false;
    }

    // First character must be letter or underscore
    if !matches!(bytes[0], b'a'..=b'z' | b'A'..=b'Z' | b'_') {
        return false;
    }

    // Rest must be alphanumeric or underscore
    let mut i = 1;
    while i < len {
        if !matches!(bytes[i], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') {
            return false;
        }
        i += 1;
    }

    true
}

#[macro_export]
macro_rules! metric_def {
    ($name:literal, $help:literal, $metric_type:expr) => {{
        const _: () = {
            if !$crate::metric_comptime::validate_metric_name($name) {
                panic!(concat!(
                    "Invalid metric name: ",
                    $name,
                    ". Should match: [a-zA-Z_:][a-zA-Z0-9_:]*"
                ));
            }
        };
        $crate::MetricDef::new_static($name, $help.to_string(), $metric_type)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Labels, MetricDef, MetricStore, MetricType, ToMetricDef};

    #[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub enum Metric {
        Health,
    }

    impl ToMetricDef for Metric {
        fn to_metric_def(&self) -> MetricDef {
            match self {
                Metric::Health => {
                    metric_def!("worker_health", "worker health", MetricType::Gauge)
                }
            }
        }
    }

    #[test]
    fn smoke() {
        let mut store = MetricStore::new();
        store.add_value(Metric::Health, &Labels::new(), true);
    }
}
