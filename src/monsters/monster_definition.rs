use crate::json_asset_definition::{Definition, RawDefinition, ToDefinition};
use crate::prelude::*;
use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "d89a0fa2-ad80-4c99-a62a-e134ce201bee"]
pub struct MonsterRawDefinition {
    pub def_name: String,
    pub model_path: String,
}

impl ToDefinition<MonsterDefinition> for MonsterRawDefinition {
    fn to_definition(&self, asset_server: &Res<AssetServer>) -> MonsterDefinition {
        MonsterDefinition {
            def_name: self.def_name.clone(),
            model: asset_server.load(&self.model_path),
        }
    }
}

impl RawDefinition<MonsterDefinition> for MonsterRawDefinition {}


#[derive(Debug, Resource)]
pub struct MonsterDefinition {
    pub def_name: String,
    pub model: Handle<Scene>,
}

impl Definition for MonsterDefinition {
    fn get_def_name(&self) -> &str {
        &self.def_name
    }
}
