use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct StringInterner {
    set: Arc<Mute<HashSet<Arc<String>>>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            set: Rc::new(HashSet::new()),
        }
    }

    pub fn intern(&mut self, s: &str) -> Arc<String> {
        let val = Arc::new(s.to_string());

        if let Some(existing) = self.set.get(&val) {
            existing.clone()
        } else {
            self.set.insert(val.clone());
            val
        }
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}
