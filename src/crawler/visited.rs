use std::{collections::HashSet, sync::{Arc, Mutex}};

#[derive(Clone, Default)]
pub struct Visited(Arc<Mutex<HashSet<String>>>);

impl Visited {
    pub fn insert_if_new(&self, url: &str) -> bool {
        let mut guard = self.0.lock().unwrap();
        guard.insert(url.to_owned())
    }
    pub fn all(&self) -> Vec<String> {
        self.0.lock().unwrap().iter().cloned().collect()
    }
    pub fn len(&self) -> usize { self.0.lock().unwrap().len() }
}
