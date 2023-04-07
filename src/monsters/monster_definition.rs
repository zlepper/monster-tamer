use crate::def_database::Definition;
use crate::def_database::{DefDatabase, DefId};
use crate::prelude::*;
use crate::world::BiomeDefinition;
use anyhow::anyhow;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "d89a0fa2-ad80-4c99-a62a-e134ce201bee"]
pub struct MonsterRawDefinition {
    pub def_name: String,
    pub model_path: String,
    pub spawn_locations: Vec<MonsterRawSpawnLocation>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MonsterRawSpawnLocation {
    pub biome_def: String,
}

impl MonsterRawDefinition {
    pub fn to_definition(
        &self,
        asset_server: &Res<AssetServer>,
        biomes: &DefDatabase<BiomeDefinition>,
    ) -> Result<MonsterDefinition> {
        let model = asset_server.load(&self.model_path);

        let mut spawn_locations = Vec::new();

        for location in self.spawn_locations.iter() {
            let biome_def = biomes
                .get_def_id(&location.biome_def)
                .ok_or_else(|| anyhow!("Biome definition '{}' not found", location.biome_def))?;
            spawn_locations.push(MonsterSpawnLocation { biome_def });
        }

        Ok(MonsterDefinition {
            def_name: self.def_name.clone(),
            model,
            spawn_locations,
        })
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MonsterDefinition {
    pub def_name: String,
    pub model: Handle<Scene>,
    pub spawn_locations: Vec<MonsterSpawnLocation>,
}

#[derive(Debug, Clone)]
pub struct MonsterSpawnLocation {
    pub biome_def: DefId<BiomeDefinition>,
}

impl Definition for MonsterDefinition {
    fn get_def_name(&self) -> &str {
        &self.def_name
    }
}
