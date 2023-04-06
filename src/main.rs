use crate::ground::GroundPlugin;
use crate::json_asset_definition::{output_json_schema, DefPlugin};
use crate::jumping::*;
use crate::monsters::MonsterPlugin;
use crate::player::PlayerPlugin;
use crate::prelude::*;

mod def_database;
mod def_types;
mod ground;
mod json_asset_definition;
mod jumping;
mod monsters;
mod player;
mod prelude;
mod shared;
mod world;

fn main() {
    output_json_schema();

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
        .add_plugin(DefPlugin)
        .run();
}
