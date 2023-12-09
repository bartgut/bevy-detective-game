use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use thiserror::Error;
use crate::quests::loader::format::QuestBundle;

#[derive(Default)]
pub struct QuestBundleLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum QuestBundleLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for QuestBundleLoader {
    type Asset = QuestBundle;
    type Settings = ();
    type Error = QuestBundleLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let quest = ron::de::from_bytes::<QuestBundle>(&bytes)?;
            Ok(quest)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["quest"]
    }
}
