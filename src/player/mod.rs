mod spawn_player;
mod movement;

use crate::prelude::*;
use crate::player::movement::*;
use crate::player::spawn_player::*;

#[derive(Component)]
pub struct Player;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((move_player, player_jump))
            .add_plugin(InputManagerPlugin::<PlayerAction>::default());
    }
}
