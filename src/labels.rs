use std::collections::{btree_map, BTreeMap};

use crate::labels_builder::LabelsBuilder;

/// Opaque representation of sample labels
///
/// Use [LabelsBuilder] object to create a new set of labels.
///
/// # Examples
/// ```
/// use simple_metrics::LabelsBuilder;
///
/// let builder = LabelsBuilder::new().with("hello", "world");
/// let res = builder.build();
/// let labels = res.unwrap();
/// assert_eq!(labels.len(), 1);
///
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labels {
    pub(crate) inner: BTreeMap<String, String>,
}

impl Labels {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Creates a new builder from the existing set of labels.
    ///
    /// # Examples
    /// ```
    /// use simple_metrics::LabelsBuilder;
    ///
    /// let base = LabelsBuilder::new().with("a", "b").build().unwrap();
    /// let chained = base.builder().with("c", "d").build().unwrap();
    ///
    /// let final_keys: Vec<_> = chained.keys().collect();
    /// assert_eq!(vec!["a", "c"], final_keys);
    /// ```
    pub fn builder(&self) -> LabelsBuilder {
        LabelsBuilder::from_labels(self)
    }

    /// This method is internal-only.
    ///
    /// Use `LabelsBuilder::build` to create a `Labels` object.
    pub(crate) fn from_builder(builder: &LabelsBuilder) -> Self {
        let mut labels = Labels::new();

        for (name, value) in builder.inner.iter() {
            labels.inner.insert(name.into(), value.into());
        }

        labels
    }

    pub fn iter(&self) -> btree_map::Iter<String, String> {
        self.inner.iter()
    }

    pub fn keys(&self) -> btree_map::Keys<String, String> {
        self.inner.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn render(&self) -> String {
        self.iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v.replace('"', r#"\""#)))
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Labels {
    type Item = (String, String);
    type IntoIter = btree_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Extend<(String, String)> for Labels {
    fn extend<T: IntoIterator<Item = (String, String)>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

// Somehow the manual ascii check shows higher performance than using
// the set of `is_ascii_*()` methods.
//
// See https://github.com/qezz/simple-metrics/pull/5 for some details.
#[allow(clippy::manual_is_ascii_check)]
#[inline(always)]
pub fn is_valid_label_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if let Some(&first) = bytes.first() {
        if !((b'A'..=b'Z').contains(&first) || (b'a'..=b'z').contains(&first) || first == b'_') {
            return false;
        }

        bytes.iter().skip(1).all(|&b| {
            (b'A'..=b'Z').contains(&b)
                || (b'a'..=b'z').contains(&b)
                || (b'0'..=b'9').contains(&b)
                || b == b'_'
        })
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{labels_builder::LabelsBuilder, Labels};

    #[test]
    fn labels_new() {
        let mut labels_insert = Labels::new();
        labels_insert.inner.insert("hello".into(), "world".into());
        labels_insert.inner.insert("woot".into(), "meh".into());
        assert_eq!(2, labels_insert.len());

        let labels_with = LabelsBuilder::new()
            .with("hello", "world")
            .with("woot", "meh")
            .build()
            .expect("can't build labels");
        assert_eq!(2, labels_with.len());

        assert_eq!(labels_insert, labels_with);

        let labels_default = Labels::default();
        assert_eq!(0, labels_default.len());
    }
}
