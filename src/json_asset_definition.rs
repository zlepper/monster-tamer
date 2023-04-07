use crate::def_database::DefDatabase;
use crate::def_types::{DefTypes, DefsRoot};
use crate::monsters::MonsterDefinition;
use crate::prelude::*;
use crate::world::BiomeDefinition;
use bevy::asset::Error;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};

struct DefsLoader;

impl AssetLoader for DefsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let raw_definition: DefsRoot = serde_json::from_slice(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(raw_definition));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum DefDatabaseState {
    #[default]
    LoadingFromDisk,
    AddingToDatabase,
    Ready,
}

#[derive(Debug, Resource)]
struct LoadingQueue(Vec<HandleUntyped>);

pub struct DefPlugin;

impl Plugin for DefPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<DefsRoot>()
            .add_state::<DefDatabaseState>()
            .add_asset_loader(DefsLoader)
            .add_system(
                add_defs_to_database.in_schedule(OnEnter(DefDatabaseState::AddingToDatabase)),
            )
            .add_system(set_database_ready.in_schedule(OnEnter(DefDatabaseState::Ready)))
            .insert_resource(LoadingQueue(Vec::new()))
            .add_system(
                (move |asset_server: Res<AssetServer>, mut queue: ResMut<LoadingQueue>| {
                    let assets = asset_server
                        .load_folder("defs")
                        .expect("Failed to load asset folder");

                    queue.0.extend(assets);
                })
                .in_schedule(OnEnter(DefDatabaseState::LoadingFromDisk)),
            )
            .add_system(
                wait_for_loading_assets.in_set(OnUpdate(DefDatabaseState::LoadingFromDisk)),
            );
    }
}

fn wait_for_loading_assets(
    mut state: ResMut<NextState<DefDatabaseState>>,
    queue: Res<LoadingQueue>,
    asset_server: Res<AssetServer>,
) {
    use bevy::asset::LoadState;

    match asset_server.get_group_load_state(queue.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            for asset in queue.0.iter() {
                if let LoadState::Failed = asset_server.get_load_state(asset.id()) {
                    error!("Failed to load asset {:?}", asset);
                }
            }
        }
        LoadState::Loaded => {
            info!("All assets loaded");
            state.set(DefDatabaseState::AddingToDatabase);
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

macro_rules! try_unpack {
    ($variant:path, $value:expr) => {
        if let $variant(x) = $value {
            Some(x)
        } else {
            None
        }
    };
}

fn add_defs_to_database(
    asset_server: Res<AssetServer>,
    raw_definitions: Res<Assets<DefsRoot>>,
    mut commands: Commands,
    mut state: ResMut<NextState<DefDatabaseState>>,
) {
    let mut errors = Vec::new();

    let all_definitions: Vec<&DefTypes> = raw_definitions.iter().flat_map(|d| &d.1.defs).collect();

    let biomes: DefDatabase<BiomeDefinition> = all_definitions
        .iter()
        .filter_map(|d| try_unpack!(DefTypes::Biome, d))
        .cloned()
        .collect();

    info!("Loaded {} biomes", biomes.len());

    let monsters: DefDatabase<MonsterDefinition> = all_definitions
        .iter()
        .filter_map(|d| {
            try_unpack!(DefTypes::Monster, d).and_then(|m| {
                match m.to_definition(&asset_server, &biomes) {
                    Ok(def) => Some(def),
                    Err(err) => {
                        errors.push(err);
                        None
                    }
                }
            })
        })
        .collect();

    info!("Loaded {} monsters", monsters.len());

    commands.insert_resource(biomes);
    commands.insert_resource(monsters);

    if !errors.is_empty() {
        error!("Failed to load some definitions: {:?}", errors);
    } else {
        info!("All definitions loaded without errors");
    }

    state.set(DefDatabaseState::Ready);
}

fn set_database_ready(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}

pub fn output_json_schema() {
    let schema = schemars::schema_for!(DefsRoot);
    let schema_file = std::fs::File::create("assets/def_schema.json")
        .expect("Failed to create schema json file on disk");
    serde_json::to_writer_pretty(schema_file, &schema).expect("Failed to schema to json file");
}
