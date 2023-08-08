use crate::def_database::DefDatabase;
use crate::def_types::{DefTypes, DefsRoot};
use crate::monsters::{MonsterDefinition, MonsterMove, MonsterType};
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

pub struct DefPlugin;

impl Plugin for DefPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<DefsRoot>()
            .add_asset_loader(DefsLoader)
            .add_collection_to_loading_state::<_, DefAssets>(GameState::LoadingFromDisk)
            .add_system(
                add_defs_to_database.in_schedule(OnEnter(GameState::AddingToDatabase)),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct DefAssets {
    #[asset(path = "defs", collection(typed))]
    pub defs: Vec<Handle<DefsRoot>>,
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
    mut state: ResMut<NextState<GameState>>,
) {
    let mut errors = Vec::new();

    let all_definitions: Vec<&DefTypes> = raw_definitions.iter().flat_map(|d| &d.1.defs).collect();

    let biomes = create_biome_defs(&all_definitions);

    info!("Loaded {} biomes", biomes.len());

    let monster_types = create_monster_type_defs(&all_definitions, &mut errors);

    info!("Loaded {} monster types", monster_types.len());

    let monster_moves = create_monster_move_defs(&all_definitions, &monster_types, &mut errors);

    info!("Loaded {} monster moves", monster_moves.len());

    let monsters = create_monster_defs(&asset_server, &all_definitions, &biomes, &monster_moves, &monster_types, &mut errors);

    info!("Loaded {} monsters", monsters.len());

    commands.insert_resource(biomes);
    commands.insert_resource(monsters);
    commands.insert_resource(monster_moves);
    commands.insert_resource(monster_types);

    if !errors.is_empty() {
        error!("Failed to load some definitions: {:?}", errors);
    } else {
        info!("All definitions loaded without errors");
    }

    state.set(GameState::Playing);
}

fn create_biome_defs(all_definitions: &[&DefTypes]) -> DefDatabase<BiomeDefinition> {
    all_definitions
        .iter()
        .filter_map(|d| try_unpack!(DefTypes::Biome, d))
        .cloned()
        .collect()
}

fn create_monster_type_defs(
    all_definitions: &[&DefTypes],
    errors: &mut Vec<Error>,
) -> DefDatabase<MonsterType> {
    let mut monster_types = all_definitions
        .iter()
        .filter_map(|d| try_unpack!(DefTypes::MonsterType, d).and_then(|t| Some(t.to_definition())))
        .collect();

    for d in all_definitions.iter() {
        if let DefTypes::MonsterType(t) = d {
            match t.link_definitions(&monster_types) {
                Ok(def) => {
                    monster_types.replace(def);
                },
                Err(err) => {
                    errors.push(err);
                }
            }
        }
    }

    monster_types
}

fn create_monster_move_defs(
    all_definitions: &[&DefTypes],
    move_types: &DefDatabase<MonsterType>,
    errors: &mut Vec<Error>,
) -> DefDatabase<MonsterMove> {
    all_definitions
        .iter()
        .filter_map(|d| {
            try_unpack!(DefTypes::MonsterMove, d).and_then(|t| {
                match t.to_definition(move_types) {
                    Ok(def) => Some(def),
                    Err(err) => {
                        errors.push(err);
                        None
                    }
                }
            })
        })
        .collect()
}

fn create_monster_defs(
    asset_server: &Res<AssetServer>,
    all_definitions: &[&DefTypes],
    biomes: &DefDatabase<BiomeDefinition>,
    moves: &DefDatabase<MonsterMove>,
    types: &DefDatabase<MonsterType>,
    errors: &mut Vec<Error>,
) -> DefDatabase<MonsterDefinition> {
    all_definitions
        .iter()
        .filter_map(|d| {
            try_unpack!(DefTypes::Monster, d).and_then(|m| {
                match m.to_definition(asset_server, biomes, moves, types) {
                    Ok(def) => Some(def),
                    Err(err) => {
                        errors.push(err);
                        None
                    }
                }
            })
        })
        .collect()
}

pub fn output_json_schema() {
    let schema = schemars::schema_for!(DefsRoot);
    let schema_file = std::fs::File::create("assets/def_schema.json")
        .expect("Failed to create schema json file on disk");
    serde_json::to_writer_pretty(schema_file, &schema).expect("Failed to schema to json file");
}
