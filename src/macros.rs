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
        MetricDef::new_unchecked($name, $help.to_string(), $metric_type)
    }};
}
