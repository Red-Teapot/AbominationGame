use std::f32::consts::PI;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_ecs_tilemap::{Map, MapQuery, MapTileError, Tile, TilePos};
use rand::random;
use gameplay::{TILE_CORE, TILE_CONNECTOR};
use crate::assets::GameplayAssets;
use crate::{gameplay, GameState};
use crate::gameplay::components::*;
use crate::gameplay::resources::{MonsterSpawnCooldown, Wallet};
use crate::gameplay::{TILE_CANNON, TILE_NONE};

pub fn core_spinner(mut query: Query<&mut Transform, With<CoreSpinner>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(-5.375 * time.delta_seconds()));
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
                        transform: Transform {
                            translation: vec3(ghost_transform.translation.x, ghost_transform.translation.y, 0.3),
                            ..default()
                        },
                        ..default()
                    }).insert(Cannon {
                        species: Species::Red,
                        cooldown: 0.0,
                    }).id();

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
                        transform: Transform {
                            translation: vec3(ghost_transform.translation.x, ghost_transform.translation.y, 0.3),
                            ..default()
                        },
                        ..default()
                    }).insert(Cannon {
                        species: Species::Green,
                        cooldown: 0.0,
                    }).id();

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
                        transform: Transform {
                            translation: vec3(ghost_transform.translation.x, ghost_transform.translation.y, 0.3),
                            ..default()
                        },
                        ..default()
                    }).insert(Cannon {
                        species: Species::Blue,
                        cooldown: 0.0,
                    }).id();

                    commands.entity(cannon_entity).insert(CannonBase(cannon_head));
                }

                _ => (),
            }

            map_query.notify_chunk_for_tile(tile_pos, 0u16, 0u16);
            commands.entity(ghost_entity).despawn();
        }
    }
}

pub fn update_cannons(mut cannon_query: Query<(&mut Transform, &GlobalTransform, &mut Cannon), Without<Monster>>,
                      mut monster_query: Query<(&GlobalTransform, &Monster)>,
                      time: Res<Time>,
                      mut commands: Commands,
                      game_assets: Res<GameplayAssets>)
{
    for (mut cannon_transform, cannon_glob_transform, mut cannon) in cannon_query.iter_mut() {
        let mut distance = f32::MAX;

        for (monster_transform, monster) in monster_query.iter() {
            let cur_distance = monster_transform.translation.distance(cannon_glob_transform.translation);
            if cannon.species == monster.0 && cur_distance < distance {
                distance = cur_distance;

                let offset: Vec2 = (monster_transform.translation - cannon_glob_transform.translation).truncate();
                cannon_transform.rotation = Quat::from_rotation_z(-offset.angle_between(Vec2::X));

                cannon.cooldown -= time.delta_seconds();
                if cannon.cooldown <= 0.0 {
                    let image = match cannon.species {
                        Species::Red => game_assets.bullet_red.clone(),
                        Species::Green => game_assets.bullet_green.clone(),
                        Species::Blue => game_assets.bullet_blue.clone(),
                    };

                    let velocity: f32 = match cannon.species {
                        Species::Red => 12.0,
                        Species::Green => 18.0,
                        Species::Blue => 29.0,
                    };

                    let velocity = Quat::from_rotation_z(-offset.angle_between(Vec2::X)) * Vec3::X * velocity;

                    commands.spawn_bundle(SpriteBundle {
                        texture: image,
                        transform: Transform {
                            translation: cannon_transform.translation.truncate().extend(5.0),
                            rotation: cannon_transform.rotation,
                            ..default()
                        },
                        ..default()
                    }).insert(Bullet {
                        velocity: velocity.truncate(),
                        species: cannon.species,
                    });

                    cannon.cooldown = match cannon.species {
                        Species::Red => 0.7,
                        Species::Green => 0.5,
                        Species::Blue => 0.25,
                    }
                }
            }
        }

        if distance == f32::MAX {
            cannon_transform.rotate(Quat::from_rotation_z(3.0 * time.delta_seconds()));
        }
    }
}

pub fn spawn_monsters(mut commands: Commands,
                      mut cooldown: ResMut<MonsterSpawnCooldown>,
                      game_assets: Res<GameplayAssets>,
                      time: Res<Time>)
{
    cooldown.0 -= time.delta_seconds();

    if cooldown.0 <= 0.0 {
        cooldown.0 = 5.0 + random::<f32>() * 10.0;

        let distance = 24.0 * 10.0;
        let angle = random::<f32>() * PI * 2.0;
        let kind = random::<f32>();
        let mut species = Species::Red;
        if kind >= 1.0 / 3.0 && kind < 2.0 / 3.0 {
            species = Species::Green;
        }
        if kind >= 2.0 / 3.0 {
            species = Species::Blue;
        }

        let image = match species {
            Species::Red => game_assets.monster_red.clone(),
            Species::Green => game_assets.monster_green.clone(),
            Species::Blue => game_assets.monster_blue.clone(),
        };

        let position = Quat::from_rotation_z(angle) * vec3(distance, 0.0, 3.7);

        commands.spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        }).insert(Monster(species))
            .insert(Health(cooldown.1));

        cooldown.1 += 1;
    }
}

pub fn move_monsters(mut monsters: Query<(&mut Transform, &Monster)>,
                     time: Res<Time>,
                     mut state: ResMut<State<GameState>>)
{
    for (mut transform, monster) in monsters.iter_mut() {
        let dir = -transform.translation.normalize().truncate().extend(0.0);
        transform.translation += dir * match monster.0 {
            Species::Red => 27.0 * time.delta_seconds(),
            Species::Green => 14.0 * time.delta_seconds(),
            Species::Blue => 9.0 * time.delta_seconds(),
        };

        if transform.translation.length() <= 24.0 {
            state.set(GameState::Lose).unwrap();
        }
    }
}

pub fn move_bullets(mut commands: Commands,
                    mut bullets: Query<(Entity, &mut Transform, &Bullet)>,
                    mut monsters: Query<(Entity, &mut Transform, &Monster, &mut Health), Without<Bullet>>,
                    mut wallet: ResMut<Wallet>)
{
    for (bullet_entity, mut bullet_transform, bullet) in bullets.iter_mut() {
        bullet_transform.translation += bullet.velocity.extend(0.0);

        for (monster_entity, monster_transform, monster, mut health) in monsters.iter_mut() {
            if monster.0 == bullet.species && monster_transform.translation.distance(bullet_transform.translation) < 30.0 {
                commands.entity(bullet_entity).despawn();

                health.0 -= match monster.0 {
                    Species::Red => 3,
                    Species::Green => 2,
                    Species::Blue => 1,
                };

                if health.0 <= 0 {
                    commands.entity(monster_entity).despawn();

                    match monster.0 {
                        Species::Red => wallet.red_squares += 5,
                        Species::Green => wallet.green_triangles += 5,
                        Species::Blue => wallet.blue_circles += 5,
                    };
                }
            }
        }
    }
}