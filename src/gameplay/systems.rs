use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_ecs_tilemap::{Map, MapQuery, MapTileError, Tile, TilePos};
use gameplay::{TILE_CORE, TILE_CONNECTOR};
use crate::assets::GameplayAssets;
use crate::gameplay;
use crate::gameplay::components::*;
use crate::gameplay::resources::Wallet;
use crate::gameplay::{TILE_CANNON, TILE_NONE};

pub fn core_spinner(mut query: Query<&mut Transform, With<CoreSpinner>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(5.375 * time.delta_seconds()));
    }
}

pub fn wallet_display(mut query: Query<(&mut Text, &WalletDisplay)>, wallet: Res<Wallet>) {
    for (mut text, wallet_display) in query.iter_mut() {
        use Species::*;

        let value = match wallet_display.0 {
            Red => wallet.red_squares,
            Green => wallet.green_triangles,
            Blue => wallet.blue_circles,
        };

        text.sections[0].value = value.to_string();
    }
}

pub fn buy_item(mut commands: Commands,
                query: Query<(&Interaction, &BuyItemButton), Changed<Interaction>>,
                ghosts: Query<(), With<ItemPlacementGhost>>,
                game_assets: Res<GameplayAssets>)
{
    for (interaction, buy_item_btn) in query.iter() {
        match interaction {
            Interaction::Clicked => if ghosts.get_single().is_err() {
                let item_image = match buy_item_btn.item {
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

                commands.spawn_bundle(SpriteBundle {
                    texture: item_image,
                    transform: Transform::from_xyz(0.0, 0.0, 100.0),
                    visibility: Visibility { is_visible: false },
                    ..default()
                }).insert(ItemPlacementGhost)
                    .insert(buy_item_btn.cost)
                    .insert(buy_item_btn.item);
            },
            _ => ()
        }
    }
}

pub fn drag_ghost(windows: Res<Windows>,
                  camera: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Map>)>,
                  mut ghost: Query<(Entity, &mut Transform, &mut Sprite, &mut Visibility, &Item, &Money), With<ItemPlacementGhost>>,
                  mut map_query: MapQuery,
                  tile_query: Query<&Tile>,
                  mouse_buttons: Res<Input<MouseButton>>,
                  mut commands: Commands,
                  mut wallet: ResMut<Wallet>,
                  game_assets: Res<GameplayAssets>)
{
    let (ghost_entity, mut ghost_transform, mut ghost_sprite, mut ghost_visibility, item, cost) = match ghost.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    ghost_visibility.is_visible = true;
    ghost_sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.5);

    let (camera, camera_transform) = camera.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        ghost_transform.translation.x = world_pos.x;
        ghost_transform.translation.y = world_pos.y;

        let map_offset = -24.0 * 8.0 * 2.0 - 12.0;
        let tile_pos = TilePos(((world_pos.x - map_offset) / 24.0) as u32, ((world_pos.y - map_offset) / 24.0) as u32);

        match map_query.get_tile_entity(tile_pos, 0, 0) {
            Ok(tile_entity) => {
                let tile = tile_query.get(tile_entity).unwrap();

                if tile.texture_index != TILE_NONE {
                    return;
                }
            },
            Err(err) => match err {
                MapTileError::OutOfBounds(_) => return,
                _ => (),
            }
        }

        let neighbors = map_query.get_tile_neighbors(tile_pos, 0, 0);

        let mut neighbor_dir = 10000;
        for i in 0..4 {
            let neighbor = neighbors[i];

            if let Ok(tile_entity) = neighbor {
                let tile = tile_query.get(tile_entity).unwrap();

                match tile.texture_index {
                    TILE_CORE | TILE_CONNECTOR => {
                        ghost_sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                        ghost_transform.translation.x = (ghost_transform.translation.x / 24.0 + 0.5).floor() * 24.0;
                        ghost_transform.translation.y = (ghost_transform.translation.y / 24.0 + 0.5).floor() * 24.0;
                        neighbor_dir = i;
                        break;
                    },
                    _ => continue,
                }
            }
        }

        if mouse_buttons.just_released(MouseButton::Right) {
            commands.entity(ghost_entity).despawn();
        }

        if mouse_buttons.just_released(MouseButton::Left) && neighbor_dir < 10 && wallet.try_spend(*cost) {
            let (flip_x, flip_y, flip_d) = match neighbor_dir {
                0 => (false, false, true), // N
                1 => (false, true, true), // S
                2 => (false, false, false), // W
                3 => (true, false, false), // E
                _ => unreachable!(),
            };

            match item {
                Item::Connector => {
                    let tile = Tile {
                        texture_index: TILE_CONNECTOR,
                        ..default()
                    };
                    map_query.set_tile(&mut commands, tile_pos, tile, 0, 0).unwrap();
                }

                Item::RedCannon => {
                    let tile = Tile {
                        texture_index: TILE_CANNON,
                        flip_x,
                        flip_y,
                        flip_d,
                        ..default()
                    };
                    let cannon_entity = map_query.set_tile(&mut commands, tile_pos, tile, 0, 0).unwrap();

                    let cannon_head = commands.spawn_bundle(SpriteBundle {
                        texture: game_assets.cannon_red.clone(),
                        transform: *ghost_transform,
                        ..default()
                    }).insert(Cannon(Species::Red)).id();

                    commands.entity(cannon_entity).insert(CannonBase(cannon_head));
                }

                Item::GreenCannon => {
                    let tile = Tile {
                        texture_index: TILE_CANNON,
                        flip_x,
                        flip_y,
                        flip_d,
                        ..default()
                    };
                    let cannon_entity = map_query.set_tile(&mut commands, tile_pos, tile, 0, 0).unwrap();

                    let cannon_head = commands.spawn_bundle(SpriteBundle {
                        texture: game_assets.cannon_green.clone(),
                        transform: *ghost_transform,
                        ..default()
                    }).insert(Cannon(Species::Green)).id();

                    commands.entity(cannon_entity).insert(CannonBase(cannon_head));
                }

                Item::BlueCannon => {
                    let tile = Tile {
                        texture_index: TILE_CANNON,
                        flip_x,
                        flip_y,
                        flip_d,
                        ..default()
                    };
                    let cannon_entity = map_query.set_tile(&mut commands, tile_pos, tile, 0, 0).unwrap();

                    let cannon_head = commands.spawn_bundle(SpriteBundle {
                        texture: game_assets.cannon_blue.clone(),
                        transform: *ghost_transform,
                        ..default()
                    }).insert(Cannon(Species::Blue)).id();

                    commands.entity(cannon_entity).insert(CannonBase(cannon_head));
                }

                _ => (),
            }

            map_query.notify_chunk_for_tile(tile_pos, 0u16, 0u16);
            commands.entity(ghost_entity).despawn();
        }
    }
}

