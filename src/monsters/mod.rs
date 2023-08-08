mod monster_definition;
mod monster_move;
mod monster_type;

pub use monster_definition::*;
pub use monster_type::*;
pub use monster_move::*;

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Monster {
    def: DefId<MonsterDefinition>,
}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {}
}
