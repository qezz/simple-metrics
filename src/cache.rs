use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use crate::Labels;

pub type InternedString = Rc<str>;
pub type LabelSetId = u32;

#[derive(Clone, Debug)]
pub struct Cache {
    label_sets: Vec<Labels>,

    /// Lookup label set if it already has an id
    cache: HashMap<Labels, LabelSetId>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            label_sets: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn intern(&mut self, labels: Labels) -> LabelSetId {
        if let Some(&id) = self.cache.get(&labels) {
            id
        } else {
            let id = self.label_sets.len() as LabelSetId;
            self.label_sets.push(labels.clone());
            self.cache.insert(labels, id);
            id
        }
    }

    pub fn get(&self, id: LabelSetId) -> Option<&Labels> {
        self.label_sets.get(id as usize)
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
