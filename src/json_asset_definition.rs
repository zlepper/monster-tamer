use crate::def_database::DefDatabase;
use crate::prelude::*;
use bevy::asset::Error;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};

pub struct JsonAssetLoader<TRawDefinition>
where
    TRawDefinition: for<'de> serde::Deserialize<'de> + Send + Sync,
{
    _phantom: std::marker::PhantomData<TRawDefinition>,

    extensions: &'static [&'static str],
}

impl<TRawDefinition> JsonAssetLoader<TRawDefinition>
where
    TRawDefinition: for<'de> serde::Deserialize<'de> + Send + Sync,
{
    pub fn new(extensions: &'static [&'static str]) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            extensions,
        }
    }
}

impl<TRawDefinition> AssetLoader for JsonAssetLoader<TRawDefinition>
where
    TRawDefinition: for<'de> serde::Deserialize<'de> + TypeUuid + Send + Sync + 'static,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let raw_definition: TRawDefinition = serde_json::from_slice(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(raw_definition));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

pub trait RawDefinition<TDefinition>:
    for<'de> serde::Deserialize<'de> + TypeUuid + Send + Sync + ToDefinition<TDefinition> + 'static
where
    TDefinition: Definition,
{
}

pub trait Definition: Resource {
    fn get_def_name(&self) -> &str;
}

pub trait JsonDefsAdder {
    fn add_json_defs<TRawDefinition, TDefinition>(
        &mut self,
        path: &'static str,
        extensions: &'static [&'static str],
    ) -> &mut Self
    where
        TRawDefinition: RawDefinition<TDefinition>,
        TDefinition: Definition;
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum DefDatabaseState {
    #[default]
    LoadingFromDisk,
    AddingToDatabase,
    Ready,
}

#[derive(Debug, Resource)]
pub struct LoadingQueue(Vec<HandleUntyped>);

impl JsonDefsAdder for App {
    fn add_json_defs<TRawDefinition, TDefinition>(
        &mut self,
        path: &'static str,
        extensions: &'static [&'static str],
    ) -> &mut Self
    where
        TRawDefinition: RawDefinition<TDefinition>,
        TDefinition: Definition,
    {
        self.add_asset::<TRawDefinition>()
            .add_state::<DefDatabaseState>()
            .add_asset_loader(JsonAssetLoader::<TRawDefinition>::new(extensions))
            .insert_resource(DefDatabase::<TDefinition>::new())
            .add_system(
                add_defs_to_database::<TDefinition, TRawDefinition>
                    .in_schedule(OnEnter(DefDatabaseState::AddingToDatabase)),
            )
            .add_system(set_database_ready.in_schedule(OnEnter(DefDatabaseState::Ready)))
            .insert_resource(LoadingQueue(Vec::new()))
            .add_system(
                (move |asset_server: Res<AssetServer>, mut queue: ResMut<LoadingQueue>| {
                    let assets = asset_server
                        .load_folder(path)
                        .expect("Failed to load asset folder");

                    queue.0.extend(assets);
                })
                .in_schedule(OnEnter(DefDatabaseState::LoadingFromDisk)),
            )
            .add_system(wait_for_loading_assets.in_set(OnUpdate(DefDatabaseState::LoadingFromDisk)))
    }
}

pub trait ToDefinition<TDefinition>
where
    TDefinition: Definition,
{
    fn to_definition(&self, asset_server: &Res<AssetServer>) -> TDefinition;
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

fn add_defs_to_database<TDefinition: Definition, TRawDefinition: RawDefinition<TDefinition>>(
    asset_server: Res<AssetServer>,
    raw_definitions: Res<Assets<TRawDefinition>>,
    mut def_database: ResMut<DefDatabase<TDefinition>>,
    mut state: ResMut<NextState<DefDatabaseState>>,
) {

    info!("Adding defs to database ({} defs)", raw_definitions.len());

    for (_, raw_def) in raw_definitions.iter() {
        let def = raw_def.to_definition(&asset_server);

        let def_name = def.get_def_name();

        info!("Def read for {:?}", def_name);

        def_database.insert(def_name.to_string(), def);
    }

    state.set(DefDatabaseState::Ready);
}

fn set_database_ready(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
