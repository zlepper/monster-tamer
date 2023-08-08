use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::monsters::MonsterType;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum MoveCategory {
    Physical,
    Magical,
}

#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "f7f576dd-71f3-40fb-988f-8e676048cbf6"]
pub struct RawMonsterMove {
    pub def_name: String,
    pub move_type_def_name: String,
    pub base_mp_usage: f32,
    pub base_damage: f32,
    pub base_accuracy: f32,
    pub base_crit_chance: f32,
    pub base_crit_multiplier: f32,
    pub post_move_speed: f32,
    pub category: MoveCategory,
}

impl RawMonsterMove {
    pub fn to_definition(&self, db: &DefDatabase<MonsterType>) -> Result<MonsterMove> {
        Ok(MonsterMove {
            def_name: self.def_name.clone(),
            move_type: db.get_def_id(&self.move_type_def_name).ok_or_else(|| anyhow!("Monster type '{}' not found", self.move_type_def_name))?,
            base_mp_usage: self.base_mp_usage,
            base_damage: self.base_damage,
            base_accuracy: self.base_accuracy,
            base_crit_chance: self.base_crit_chance,
            base_crit_multiplier: self.base_crit_multiplier,
            category: self.category,
        })
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MonsterMove {
    pub def_name: String,
    pub move_type: DefId<MonsterType>,
    pub base_mp_usage: f32,
    pub base_damage: f32,
    pub base_accuracy: f32,
    pub base_crit_chance: f32,
    pub base_crit_multiplier: f32,
    pub category: MoveCategory,
}

impl Definition for MonsterMove {
    fn get_def_name(&self) -> &str {
        &self.def_name
    }
}
