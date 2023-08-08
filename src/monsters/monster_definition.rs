use crate::def_database::Definition;
use crate::def_database::{DefDatabase, DefId};
use crate::prelude::*;
use crate::world::BiomeDefinition;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::monsters::{MonsterMove, MonsterType};



#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "d89a0fa2-ad80-4c99-a62a-e134ce201bee"]
pub struct MonsterRawDefinition {
    pub def_name: String,
    pub model_path: String,
    pub spawn_locations: Vec<MonsterRawSpawnLocation>,
    pub move_learn_set: Vec<RawLearnedMove>,
    pub types: Vec<String>,
}


#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum RawMoveLearnCondition {
    Level(u32),
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RawLearnedMove {
    pub monster_move_def_name: String,
    pub conditions: Vec<RawMoveLearnCondition>,
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
        moves: &DefDatabase<MonsterMove>,
        types: &DefDatabase<MonsterType>,
    ) -> Result<MonsterDefinition> {
        let model = asset_server.load(&self.model_path);

        let mut spawn_locations = Vec::new();

        for location in self.spawn_locations.iter() {
            let biome_def = biomes
                .get_def_id(&location.biome_def)
                .ok_or_else(|| anyhow!("Biome definition '{}' not found", location.biome_def))?;
            spawn_locations.push(MonsterSpawnLocation { biome_def });
        }

        let mut move_learn_set = Vec::new();

        for learned_move in self.move_learn_set.iter() {
            let monster_move = moves
                .get_def_id(&learned_move.monster_move_def_name)
                .ok_or_else(|| anyhow!("Monster move '{}' not found", learned_move.monster_move_def_name))?;
            let mut conditions = Vec::new();
            for condition in learned_move.conditions.iter() {
                match condition {
                    RawMoveLearnCondition::Level(level) => conditions.push(MoveLearnCondition::Level(*level)),
                }
            }
            move_learn_set.push(LearnedMove {
                monster_move,
                conditions,
            });
        }

        let mut monster_types = Vec::new();
        for monster_type in self.types.iter() {
            let monster_type = types
                .get_def_id(&monster_type)
                .ok_or_else(|| anyhow!("Monster type '{}' not found", monster_type))?;
            monster_types.push(monster_type);
        }

        Ok(MonsterDefinition {
            def_name: self.def_name.clone(),
            model,
            spawn_locations,
            move_learn_set,
            types: monster_types,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveLearnCondition {
    Level(u32),
}

#[derive(Debug, Clone)]
pub struct LearnedMove {
    pub monster_move: DefId<MonsterMove>,
    pub conditions: Vec<MoveLearnCondition>,
}

#[derive(Debug, Clone, Resource)]
pub struct MonsterDefinition {
    pub def_name: String,
    pub model: Handle<Scene>,
    pub spawn_locations: Vec<MonsterSpawnLocation>,
    pub move_learn_set: Vec<LearnedMove>,
    pub types: Vec<DefId<MonsterType>>,
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
