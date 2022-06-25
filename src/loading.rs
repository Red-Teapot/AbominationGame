use bevy::math::vec3;
use bevy::prelude::*;
use bevy::ui::Val::Percent;
use crate::{palette, PreloadedAssets};

pub fn loading_start(mut commands: Commands, assets: Res<PreloadedAssets>) {
    let mut camera = UiCameraBundle::default();
    commands.spawn_bundle(camera);

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section("Loading assets...", TextStyle {
                font: assets.font.clone(),
                color: palette::WHITE,
                font_size: 56.0,
            }, TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            transform: Transform {
                scale: vec3(0.25, 0.25, 1.0) * 3.0,
                ..default()
            },
            ..default()
        });
    });
}

pub fn loading_end(mut commands: Commands, mut entities: Query<Entity>) {
    for entity in entities.iter_mut() {
        commands.entity(entity).despawn();
    }
}