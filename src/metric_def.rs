use std::fmt::Display;

use crate::{Error, MetricType};

/// MetricName is part of implementation detail of how metric
/// definitions (`MetricDef`) are constructed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MetricName {
    Static(&'static str),
    Dynamic(String),
}

impl Display for MetricName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricName::Static(s) => write!(f, "{}", s),
            MetricName::Dynamic(s) => write!(f, "{}", s),
        }
    }
}

impl AsRef<str> for MetricName {
    fn as_ref(&self) -> &str {
        match self {
            MetricName::Static(s) => s,
            MetricName::Dynamic(s) => s,
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
    pub(crate) name: MetricName,
    pub(crate) help: String,
    pub(crate) metric_type: MetricType,
}

/// Metric definition, including its name, help string, and metric type
impl MetricDef {
    pub fn new(name: &str, help: &str, metric_type: MetricType) -> Result<Self, Error> {
        if !is_valid_metric_name(name) {
            return Err(Error::InvalidMetricName(name.to_string()));
        }

        Ok(Self {
            name: MetricName::Dynamic(name.to_string()),
            help: help.to_string(),
            metric_type,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn gauge(name: &str, help: &str) -> Result<Self, Error> {
        Self::new(name, help, MetricType::Gauge)
    }
}

// TODO: Replace with From<X> for MetricDef?
pub trait ToMetricDef: Clone {
    fn to_metric_def(&self) -> MetricDef;
}

impl MetricDef {
    /// Unchecked metric definition.
    ///
    /// <div class="warning">
    ///
    /// Do not use if you don't know the implications. The
    /// [`MetricStore`](crate::MetricStore) will happily render these
    /// values even if they are invalid Prometheus metric definitions.
    ///
    /// Use [`new()`](Self::new) to dynamically create a [`MetricDef`].
    ///
    /// See [`metric_def!()`](crate::metric_def!) macro docs for
    /// compile time checks.
    ///
    /// </div>
    pub const fn new_unchecked(name: &'static str, help: String, metric_type: MetricType) -> Self {
        Self {
            name: MetricName::Static(name),
            help,
            metric_type,
        }
    }
}

#[allow(clippy::manual_is_ascii_check)]
#[inline(always)]
pub fn is_valid_metric_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if let Some(&first) = bytes.first() {
        if !((b'A'..=b'Z').contains(&first)
            || (b'a'..=b'z').contains(&first)
            || first == b'_'
            || first == b':')
        {
            return false;
        }

        bytes.iter().skip(1).all(|&b| {
            (b'A'..=b'Z').contains(&b)
                || (b'a'..=b'z').contains(&b)
                || (b'0'..=b'9').contains(&b)
                || b == b'_'
                || b == b':'
        })
    } else {
        false
    }
}
