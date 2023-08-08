mod movement;
mod player_monster;
mod roaming_camera;
mod spawn_player;

use crate::player::movement::*;
use crate::player::roaming_camera::*;
use crate::player::spawn_player::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_collection_to_loading_state::<_, PlayerAssets>(GameState::LoadingFromDisk)
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((move_player.run_if(has_window_focus), player_jump.run_if(has_window_focus)))
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_system(pan_camera.run_if(has_window_focus))
            .add_system(rotate_player.run_if(has_window_focus))
            .add_system(grab_mouse);
    }
}
