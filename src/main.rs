use bevy_editor_pls::EditorWindowPlacement;
use crate::ground::GroundPlugin;
use crate::json_asset_definition::{output_json_schema, DefPlugin};
use crate::jumping::*;
use crate::monsters::MonsterPlugin;
use crate::player::PlayerPlugin;
use crate::prelude::*;
use bevy_editor_pls::prelude::*;

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
        .add_plugin(EditorPlugin {
            window: EditorWindowPlacement::New(Window {
                title: "Editor".to_string(),
                ..Default::default()
            }),
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            mode: DebugRenderMode::default()|DebugRenderMode::CONTACTS,
            ..default()
        })
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::LoadingFromDisk).continue_to_state(GameState::AddingToDatabase))
        .add_plugin(GroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(JumpingPlugin)
        .add_plugin(MonsterPlugin)
        .add_plugin(DefPlugin)
        .run();
}
