use crate::ground::GroundPlugin;
use crate::jumping::*;
use crate::player::PlayerPlugin;
use crate::prelude::*;

mod jumping;
mod ground;
mod shared;
mod player;
mod prelude;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_plugin(GroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(JumpingPlugin)
        .run();
}
