use crate::monsters::MonsterRawDefinition;
use crate::world::BiomeDefinition;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum DefTypes {
    Monster(MonsterRawDefinition),
    Biome(BiomeDefinition),
}

#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "3423cab8-79ff-4cad-b9ee-0358c005fc1b"]
pub struct DefsRoot {
    pub defs: Vec<DefTypes>,
}
