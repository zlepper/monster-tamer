mod monster_definition;

use crate::json_asset_definition::{JsonDefsAdder};
use crate::monsters::monster_definition::{MonsterDefinition, MonsterRawDefinition};
use crate::prelude::*;


#[derive(Component, Debug)]
pub struct Monster {
    experience: u64,
}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_json_defs::<MonsterRawDefinition, MonsterDefinition>("monsters", &["monster.json"]);
    }
}
