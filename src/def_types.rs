use crate::monsters::{MonsterRawDefinition, RawMonsterMove, RawMonsterType};
use crate::world::BiomeDefinition;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum DefTypes {
    Monster(MonsterRawDefinition),
    Biome(BiomeDefinition),
    MonsterType(RawMonsterType),
    MonsterMove(RawMonsterMove),
}

#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "3423cab8-79ff-4cad-b9ee-0358c005fc1b"]
pub struct DefsRoot {
    pub defs: Vec<DefTypes>,
}
