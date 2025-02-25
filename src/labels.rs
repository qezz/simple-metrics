use std::collections::{btree_map, BTreeMap};

use regex::Regex;

use crate::Error;

lazy_static::lazy_static! {
    static ref LABEL_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

/// Internal representation of sample labels
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labels {
    inner: BTreeMap<String, String>,
}

impl Labels {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.insert(key, value);
        self
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
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

impl<T, U> FromIterator<(T, U)> for Labels
where
    T: Into<String>,
    U: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let mut map = BTreeMap::new();
        for (key, value) in iter {
            map.insert(key.into(), value.into());
        }
        Labels { inner: map }
    }
}

impl<'a, T, U> From<&'a [(T, U)]> for Labels
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
    for Labels
{
    fn from(value: [(K, V); N]) -> Self {
        value.iter().cloned().collect()
    }
}

pub fn check_labels(labels: &Labels) -> Result<(), Error> {
    for name in labels.keys() {
        if !LABEL_NAME_RE.is_match(name) {
            return Err(Error::InvalidLabelName(name.to_string()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Labels;

    #[test]
    fn labels_new() {
        let mut labels_insert = Labels::new();
        labels_insert.insert("hello", "world");
        labels_insert.insert("woot", "meh");
        assert_eq!(2, labels_insert.len());

        let labels_with = Labels::new().with("hello", "world").with("woot", "meh");
        assert_eq!(2, labels_with.len());

        assert_eq!(labels_insert, labels_with);

        let labels_default = Labels::default();
        assert_eq!(0, labels_default.len());
    }

    #[test]
    fn labels_with() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let labels: Labels = tuples.into_iter().collect();
        assert_eq!(3, labels.len());

        let new_labels = labels.clone().with("four", "4");

        let tuples2 = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4")];
        let labels2: Labels = tuples2.into_iter().collect();

        assert_eq!(new_labels, labels2);
    }

    #[test]
    fn labels_from_tuple_list() {
        let tuples = [("one", "1"), ("two", "2"), ("three", "3")];
        let labels: Labels = tuples.into_iter().collect();
        assert_eq!(3, labels.len());

        let labels_from = Labels::from(&tuples[..]);
        assert_eq!(3, labels_from.len());

        assert_eq!(labels, labels_from);

        let labels_from_2 = Labels::from(&[("one", "1"), ("two", "2"), ("three", "3")][..]);
        assert_eq!(labels, labels_from_2);

        let labels_from_3 = Labels::from([("one", "1"), ("two", "2"), ("three", "3")]);
        assert_eq!(labels, labels_from_3);
    }
}
