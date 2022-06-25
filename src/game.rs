use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;
use crate::assets::{GameplayAssets, UIAssets};
use crate::{gameplay, palette};
use crate::loading::{loading_end, loading_start};

pub fn run(app: &mut App) {
    app.insert_resource(WindowDescriptor {
        title: "Abomination".to_string(),
        resizable: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(palette::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin)
    .add_plugin(TweeningPlugin)
    .add_plugin(TilemapPlugin)
    .add_plugin(NinePatchPlugin::<()>::default());

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    AssetLoader::new(GameState::LoadingAssets)
        .continue_to_state(GameState::Gameplay)
        .with_collection::<GameplayAssets>()
        .with_collection::<UIAssets>()
        .build(app);

    app.add_state(GameState::LoadingAssets)
        .add_startup_system(preload_assets)
        .add_system_set(SystemSet::on_enter(GameState::LoadingAssets).with_system(loading_start))
        .add_system_set(SystemSet::on_exit(GameState::LoadingAssets).with_system(loading_end));

    gameplay::register_systems(app);

    app.run();
}

const FONT: &[u8] = include_bytes!("../assets/dpcomic.ttf");

fn preload_assets(mut commands: Commands, mut font_assets: ResMut<Assets<Font>>) {
    let font = Font::try_from_bytes(FONT.to_owned()).unwrap();

    let font_handle = font_assets.add(font);

    commands.insert_resource(PreloadedAssets {
        font: font_handle,
    });
}

pub struct PreloadedAssets {
    pub font: Handle<Font>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    LoadingAssets,
    Gameplay,
}
