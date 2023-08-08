use crate::prelude::*;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, TypeUuid, JsonSchema)]
#[uuid = "2dc91960-9211-4596-906a-27c0f4d71d71"]
pub struct RawMonsterType {
    pub def_name: String,
    pub damage_scales: Vec<RawMonsterTypeDamageScale>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RawMonsterTypeDamageScale {
    pub damage_scale: f32,
    pub target_type_def_name: String,
}

impl RawMonsterType {
    pub fn to_definition(&self) -> MonsterType {
        MonsterType {
            def_name: self.def_name.clone(),
            damage_scales: vec![]
        }
    }
    
    pub fn link_definitions(&self, monster_types: &DefDatabase<MonsterType>) -> Result<MonsterType> {
        let mut damage_scales = Vec::new();
        
        for scale in self.damage_scales.iter() {
            let target_type = monster_types
                .get_def_id(&scale.target_type_def_name)
                .ok_or_else(|| anyhow!("Monster type '{}' not found", scale.target_type_def_name))?;
            damage_scales.push(MonsterTypeDamageScale {
                damage_scale: scale.damage_scale,
                target_type,
            });
        }
        
        Ok(MonsterType {
            def_name: self.def_name.clone(),
            damage_scales,
        })
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MonsterType {
    pub def_name: String,
    pub damage_scales: Vec<MonsterTypeDamageScale>,
}

impl Definition for MonsterType {
    fn get_def_name(&self) -> &str {
        &self.def_name
    }
}

#[derive(Debug, Clone)]
pub struct MonsterTypeDamageScale {
    pub damage_scale: f32,
    pub target_type: DefId<MonsterType>,
}
