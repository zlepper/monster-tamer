use crate::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait Definition: Send + Sync + 'static {
    fn get_def_name(&self) -> &str;
}

#[derive(Resource)]
pub struct DefDatabase<T>
where
    T: Definition,
{
    by_name_map: HashMap<String, u64>,
    by_id_map: HashMap<u64, DefInfo<T>>,

    next_id: u64,
}

pub struct DefInfo<T>
where
    T: Definition,
{
    pub id: DefId<T>,
    pub definition: T,
}

impl<T> Definition for DefInfo<T>
where
    T: Definition,
{
    fn get_def_name(&self) -> &str {
        self.definition.get_def_name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefId<TDef> {
    id: u64,
    _phantom: PhantomData<TDef>,
}

impl<TDef> DefId<TDef> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
}

impl<TDef> PartialEq<Self> for DefId<TDef> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<TDef> Eq for DefId<TDef> {}

impl<TDef> Hash for DefId<TDef> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> DefDatabase<T>
where
    T: Definition,
{
    pub fn new() -> Self {
        Self {
            by_name_map: HashMap::new(),
            by_id_map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn get_by_id(&self, id: &DefId<T>) -> Option<&T> {
        let def = self.by_id_map.get(&id.id)?;
        Some(&def.definition)
    }

    pub fn get_def_id(&self, name: &str) -> Option<DefId<T>> {
        self.by_name_map.get(name).map(|id| DefId::new(*id))
    }

    pub fn insert(&mut self, value: T) {
        self.next_id += 1;
        let key = value.get_def_name().to_string();
        let def_info = DefInfo {
            id: DefId {
                id: self.next_id,
                _phantom: PhantomData,
            },
            definition: value,
        };
        self.by_name_map.insert(key, def_info.id.id);
        self.by_id_map.insert(def_info.id.id, def_info);
    }

    pub fn replace(&mut self, value: T) {
        let existing_id = self
            .by_name_map
            .get(value.get_def_name())
            .unwrap_or_else(|| panic!("Def {} not found for replacement", value.get_def_name()));

        self.by_id_map.get_mut(existing_id).unwrap().definition = value;
    }

    pub fn len(&self) -> usize {
        self.by_id_map.len()
    }
}

impl<T: Definition> FromIterator<T> for DefDatabase<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut db = Self::new();
        for item in iter {
            db.insert(item);
        }
        db
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SimpleTestDef {
        name: String,
        value: u32,
    }

    impl Definition for SimpleTestDef {
        fn get_def_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn insert_and_get() {
        let mut db = DefDatabase::<SimpleTestDef>::new();
        db.insert(SimpleTestDef { name: "test".to_string(), value: 42 });
        let id = db.get_def_id("test").unwrap();
        let def = db.get_by_id(&id).unwrap();
        assert_eq!(def.name, "test");
        assert_eq!(def.value, 42);
    }

    #[test]
    fn replace() {
        let mut db = DefDatabase::<SimpleTestDef>::new();
        db.insert(SimpleTestDef { name: "test".to_string(), value: 42 });
        let id = db.get_def_id("test").unwrap();
        let def = db.get_by_id(&id).unwrap();
        assert_eq!(def.name, "test");
        assert_eq!(def.value, 42);
        db.replace(SimpleTestDef { name: "test".to_string(), value: 43 });
        let def = db.get_by_id(&id).unwrap();
        assert_eq!(def.name, "test");
        assert_eq!(def.value, 43);
    }
}
