use crate::prelude::*;
use bevy::reflect::TypeUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, TypeUuid, JsonSchema, Resource)]
#[uuid = "ceeae331-a37a-428c-aa39-0dd85152d090"]
pub struct BiomeDefinition {
    pub def_name: String,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub min_humidity: Option<f32>,
    pub max_humidity: Option<f32>,
    pub min_temperature: Option<f32>,
    pub max_temperature: Option<f32>,
}

impl Definition for BiomeDefinition {
    fn get_def_name(&self) -> &str {
        &self.def_name
    }
}
