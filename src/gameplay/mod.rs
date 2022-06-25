use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::Anchor;
use bevy::ui::FocusPolicy;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::InspectorPlugin;

use crate::{GameState, palette, PreloadedAssets};
use crate::assets::*;
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use crate::gameplay::systems::*;

mod components;
mod bundles;
mod resources;
mod systems;

pub const TILE_NONE: u16 = 0;
pub const TILE_CORE: u16 = 1;
pub const TILE_CONNECTOR: u16 = 2;
pub const TILE_CANNON: u16 = 3;

pub fn register_systems(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(gameplay_enter));

    app.add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(core_spinner)
        .with_system(wallet_display)
        .with_system(buy_item)
        .with_system(drag_ghost)
        .with_system(update_cannons)
        .with_system(spawn_monsters)
        .with_system(move_monsters)
        .with_system(move_bullets));

    #[cfg(debug_assertions)]
    app.add_plugin(InspectorPlugin::<Wallet>::new());
}

pub fn gameplay_enter(mut commands: Commands,
                      game_assets: Res<GameplayAssets>,
                      ui_assets: Res<UIAssets>,
                      pre_assets: Res<PreloadedAssets>,
                      mut map_query: MapQuery) {
    commands.insert_resource(Wallet {
        red_squares: 20,
        green_triangles: 20,
        blue_circles: 20,
    });

    commands.insert_resource(MonsterSpawnCooldown(10.0, DEFAULT_HEALTH));

    let mut world_camera = OrthographicCameraBundle::new_2d();
    world_camera.orthographic_projection.scale = 1.0 / 3.0;
    commands.spawn_bundle(world_camera)
        .insert(MainCamera);

    commands.spawn_bundle(UiCameraBundle::default());

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0, map_entity);

    let mut layer_settings = LayerSettings::new(
        MapSize(4, 4),
        ChunkSize(8, 8),
        TileSize(24.0, 24.0),
        TextureSize(49.0, 124.0),
    );
    layer_settings.tile_spacing = vec2(1.0, 1.0);
    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        layer_settings,
        0,
        0,
    );
    layer_builder.set_all(TileBundle {
        tile: Tile {
            texture_index: TILE_NONE,
            ..default()
        },
        ..default()
    });

    layer_builder.set_tile(TilePos(16, 16), TileBundle {
        tile: Tile {
            texture_index: TILE_CORE,
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();

    map_query.build_layer(&mut commands, layer_builder, game_assets.tiles.clone());

    map.add_layer(&mut commands, 0, layer_entity);

    commands.entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-24.0 * 8.0 * 2.0 - 12.0, -24.0 * 8.0 * 2.0 - 12.0, 0.1))
        .insert(GlobalTransform::default());

    commands.spawn_bundle(SpriteBundle {
        texture: game_assets.core_spinner.clone(),
        sprite: Sprite {
            anchor: Anchor::Center,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.2),
        ..Default::default()
    }).insert(CoreSpinner)
        .insert(Health(DEFAULT_HEALTH));

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|panel| {
            insert_wallet_info(panel, Species::Red, &ui_assets, &pre_assets);
            insert_wallet_info(panel, Species::Green, &ui_assets, &pre_assets);
            insert_wallet_info(panel, Species::Blue, &ui_assets, &pre_assets);
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|panel| {
            insert_cost_info(panel, Money::new(Species::Red, 5), Item::Connector, &game_assets, &pre_assets, &ui_assets);

            insert_cost_info(panel, Money::new(Species::Red, 10), Item::RedCannon, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Green, 10), Item::GreenCannon, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Blue, 10), Item::BlueCannon, &game_assets, &pre_assets, &ui_assets);

            /*insert_cost_info(panel, Money::new(Species::Red, 10), Item::ConverterRedGreen, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Red, 10), Item::ConverterRedBlue, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Green, 10), Item::ConverterGreenRed, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Green, 10), Item::ConverterGreenBlue, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Blue, 10), Item::ConverterBlueRed, &game_assets, &pre_assets, &ui_assets);
            insert_cost_info(panel, Money::new(Species::Blue, 10), Item::ConverterBlueGreen, &game_assets, &pre_assets, &ui_assets);*/
        });
    });
}

fn insert_wallet_info(parent: &mut ChildBuilder, species: Species, ui_assets: &UIAssets, pre_assets: &PreloadedAssets) {
    fn color(species: Species) -> Color {
        match species {
            Species::Red => palette::RED,
            Species::Green => palette::GREEN,
            Species::Blue => palette::BLUE,
        }
    }

    let image = |species: Species| -> Handle<Image> {
        match species {
            Species::Red => ui_assets.red_square.clone(),
            Species::Green => ui_assets.green_triangle.clone(),
            Species::Blue => ui_assets.blue_circle.clone(),
        }
    };

    parent.spawn_bundle(NodeBundle {
        color: palette::BLACK.into(),
        ..default()
    }).with_children(|section| {
        section.spawn_bundle(NodeBundle {
            image: UiImage(image(species)),
            style: Style {
                size: Size::new(Val::Px(24.0 * 3.0), Val::Px(24.0 * 3.0)),
                ..default()
            },
            ..default()
        });
        section.spawn_bundle(TextBundle {
            text: Text::with_section("???", TextStyle {
                font: pre_assets.font.clone(),
                font_size: 56.0,
                color: color(species),
            }, TextAlignment::default()),
            transform: Transform {
                scale: vec3(0.25, 0.25, 1.0) * 3.0,
                ..default()
            },
            style: Style {
                position: Rect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(3.0),
                },
                size: Size {
                    width: Val::Px(36.0 * 3.0),
                    height: Val::Px(24.0 * 3.0),
                },
                ..default()
            },
            ..default()
        }).insert(WalletDisplay(species));
    });
}

fn insert_cost_info(parent: &mut ChildBuilder,
                    cost: Money,
                    item: Item,
                    game_assets: &GameplayAssets,
                    pre_assets: &PreloadedAssets,
                    ui_assets: &UIAssets) {

    let money_color = match cost.species {
        Species::Red => palette::RED,
        Species::Green => palette::GREEN,
        Species::Blue => palette::BLUE,
    };

    let money_image = match cost.species {
        Species::Red => (ui_assets.red_square_small.clone(), vec2(5.0, 5.0)),
        Species::Green => (ui_assets.green_triangle_small.clone(), vec2(7.0, 4.0)),
        Species::Blue => (ui_assets.blue_circle_small.clone(), vec2(6.0, 6.0)),
    };

    let item_image = match item {
        Item::Connector => game_assets.connector.clone(),
        Item::RedCannon => game_assets.cannon_red.clone(),
        Item::GreenCannon => game_assets.cannon_green.clone(),
        Item::BlueCannon => game_assets.cannon_blue.clone(),
        Item::ConverterRedGreen => game_assets.converter_red_green.clone(),
        Item::ConverterRedBlue => game_assets.converter_red_blue.clone(),
        Item::ConverterGreenRed => game_assets.converter_green_red.clone(),
        Item::ConverterGreenBlue => game_assets.converter_green_blue.clone(),
        Item::ConverterBlueRed => game_assets.converter_blue_red.clone(),
        Item::ConverterBlueGreen => game_assets.converter_blue_green.clone(),
    };

    parent.spawn_bundle(ButtonBundle {
        color: palette::BLACK.into(),
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|section| {
        section.spawn_bundle(NodeBundle {
            image: UiImage(item_image),
            style: Style {
                size: Size::new(Val::Px(24.0 * 3.0), Val::Px(24.0 * 3.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        });

        section.spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Px(32.0 * 3.0),
                    ..default()
                },
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        }).with_children(|line| {
            let (cur_img, cur_size) = money_image;

            line.spawn_bundle(NodeBundle {
                image: UiImage(cur_img),
                style: Style {
                    size: Size::new(Val::Px(cur_size.x * 3.0), Val::Px(cur_size.y * 3.0)),
                    ..default()
                },
                ..default()
            });

            line.spawn_bundle(TextBundle {
                text: Text::with_section(cost.amount.to_string(), TextStyle {
                    font: pre_assets.font.clone(),
                    font_size: 56.0,
                    color: money_color,
                }, default()),
                transform: Transform {
                    scale: vec3(0.25, 0.25, 1.0) * 3.0,
                    ..default()
                },
                style: Style {
                    position: Rect {
                        top: Val::Px(6.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Px(16.0 * 3.0),
                        height: Val::Px(12.0 * 3.0),
                    },
                    ..default()
                },
                ..default()
            });
        });
    }).insert(BuyItemButton {
        cost,
        item,
    });
}