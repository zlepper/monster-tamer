use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    LoadingFromDisk,
    AddingToDatabase,
    Playing,
}
