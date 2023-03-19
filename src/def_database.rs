use crate::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct DefDatabase<T>
where
    T: Resource,
{
    map: HashMap<String, T>,
}

impl<T> DefDatabase<T>
where
    T: Resource,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.map.insert(key, value);
    }
}

impl<T> Default for DefDatabase<T>
where
    T: Resource,
{
    fn default() -> Self {
        Self::new()
    }
}
