use crate::json_asset_definition::Definition;
use crate::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn get_by_name(&self, name: &str) -> Option<&DefInfo<T>> {
        let id = self.by_name_map.get(name)?;
        self.by_id_map.get(id)
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
