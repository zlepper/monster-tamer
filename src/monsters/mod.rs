mod monster_definition;
pub use monster_definition::{MonsterDefinition, MonsterRawDefinition};

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Monster {
    experience: u64,
}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {}
}
