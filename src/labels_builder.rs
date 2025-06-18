use std::collections::BTreeMap;

use crate::{labels::is_valid_label_name, Error, Labels};

/// Builder for [Labels] struct.
///
/// Label names should match the regex `[a-zA-Z_][a-zA-Z0-9_]*`, that's
/// why `.build()` returns a `Result`, and it's up to the user how to
/// handle it.
///
/// # Prometheus v3 compatibility note
///
/// Prometheus v3 allows metric names and label names to use any UTF-8
/// characters. **This feature is not currently supported in this
/// library.** For more details, refer to the official Prometheus
/// docs: <https://prometheus.io/docs/concepts/data_model/>
///
///
/// # Examples
/// ```
/// use simple_metrics::LabelsBuilder;
///
/// let builder_result = LabelsBuilder::new()
///     .with("hello", "world")
///     .with("simple", "metrics")
///     .build();
/// let _ = builder_result.expect("label names are invalid");
///
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelsBuilder {
    pub(crate) inner: BTreeMap<String, String>,
}

impl LabelsBuilder {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn from_labels(labels: &Labels) -> Self {
        Self {
            inner: labels.inner.clone(),
        }
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
        self
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
    }

    pub fn check_labels(&self) -> Result<(), Error> {
        for (name, _) in self.inner.iter() {
            if !is_valid_label_name(name) {
                return Err(Error::InvalidLabelName(name.to_string()));
            }
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Labels, Error> {
        self.check_labels()?;

        Ok(Labels::from_builder(self))
    }
}

impl Default for LabelsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> FromIterator<(T, U)> for LabelsBuilder
where
    T: Into<String>,
    U: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let mut map = BTreeMap::new();
        for (key, value) in iter {
            map.insert(key.into(), value.into());
        }
        Self { inner: map }
    }
}

impl<'a, T, U> From<&'a [(T, U)]> for LabelsBuilder
where
    T: Into<String> + AsRef<str> + 'a,
    U: Into<String> + AsRef<str> + 'a,
{
    fn from(tuples: &'a [(T, U)]) -> Self {
        tuples
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect()
    }
}

impl<K: Ord + Clone + Into<String>, V: Clone + Into<String>, const N: usize> From<[(K, V); N]>
    for LabelsBuilder
{
    fn from(value: [(K, V); N]) -> Self {
        value.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Labels};

    use super::LabelsBuilder;

    #[test]
    fn simple() {
        let builder = LabelsBuilder::new().with("hello", "world");
        let labels = builder.build().unwrap();

        let mut builder2 = LabelsBuilder::new();
        builder2.insert("hello", "world");
        let labels2 = builder2.build().unwrap();

        let expected = {
            let mut tmp = Labels::new();
            tmp.inner.insert("hello".into(), "world".into());
            tmp
        };

        assert_eq!(labels, expected);
        assert_eq!(labels2, expected);
    }

    #[test]
    fn labels_with() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let builder: LabelsBuilder = tuples.into_iter().collect();
        let labels = builder.build().unwrap();
        assert_eq!(3, labels.len());

        let new_labels = labels.clone().builder().with("four", "4").build().unwrap();

        let tuples2 = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4")];
        let builder2: LabelsBuilder = tuples2.into_iter().collect();
        let labels2 = builder2.build().unwrap();

        assert_eq!(new_labels, labels2);
    }

    #[test]
    fn labels_from_tuple_list() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let builder: LabelsBuilder = tuples.into_iter().collect();
        let labels = builder.build().unwrap();
        assert_eq!(3, labels.len());

        let builder_from = LabelsBuilder::from(&tuples[..]);
        let labels_from = builder_from.build().unwrap();
        assert_eq!(3, labels_from.len());

        assert_eq!(labels, labels_from);

        let labels_from_2 = LabelsBuilder::from(&[("one", "1"), ("two", "2"), ("three", "3")][..])
            .build()
            .unwrap();
        assert_eq!(labels, labels_from_2);

        let labels_from_3 = LabelsBuilder::from([("one", "1"), ("two", "2"), ("three", "3")])
            .build()
            .unwrap();
        assert_eq!(labels, labels_from_3);
    }

    #[test]
    fn invalid_labels() {
        let builder: LabelsBuilder = LabelsBuilder::new().with("!invalid", "some value");
        let res = builder.build();

        match res {
            Ok(_) => panic!("must fail"),
            Err(e) => {
                assert_eq!(e, Error::InvalidLabelName("!invalid".to_string()));
            }
        }
    }
}
