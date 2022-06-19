use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;

pub fn run(app: &mut App) {
    app.insert_resource(WindowDescriptor {
        title: "Abomination".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin)
    .add_plugin(TweeningPlugin)
    .add_plugin(TilemapPlugin)
    .add_plugin(NinePatchPlugin::<()>::default());

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
