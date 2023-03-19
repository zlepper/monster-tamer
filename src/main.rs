use crate::ground::GroundPlugin;
use crate::jumping::*;
use crate::monsters::MonsterPlugin;
use crate::player::PlayerPlugin;
use crate::prelude::*;

mod jumping;
mod ground;
mod shared;
mod player;
mod prelude;
mod monsters;
mod def_database;
mod json_asset_definition;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<GameState>()
        .add_plugin(GroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(JumpingPlugin)
        .add_plugin(MonsterPlugin)
        .run();
}
