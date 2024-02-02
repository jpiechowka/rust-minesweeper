use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use crate::components::{Coordinates, Mine, MineNeighbor, Uncover};
use crate::plugins::{
    BoardCompletedEvent, Bounds2, MineExplosionEvent, TileMarkEvent, TileTriggerEvent,
};
use crate::resources::{Board, BoardAssets, BoardOptions, BoardPosition, Tile, TileMap, TileSize};
use crate::systems::{handle_mouse_input, mark_tiles, trigger_event_handler, uncover_tiles};
use crate::AppState;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.running_state.clone()), Self::create_board);

        app.add_systems(
            Update,
            (
                handle_mouse_input,
                trigger_event_handler,
                uncover_tiles,
                mark_tiles,
            )
                .run_if(in_state(AppState::InGame)),
        );

        app.add_systems(OnExit(self.running_state.clone()), Self::cleanup_board);

        app.add_event::<TileTriggerEvent>();
        app.add_event::<TileMarkEvent>();
        app.add_event::<MineExplosionEvent>();
        app.add_event::<BoardCompletedEvent>();

        info!("Loaded Board Plugin");
    }
}

impl<T: States> BoardPlugin<T> {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        board_assets: Res<BoardAssets>,
    ) {
        let window = window_query.single();

        let options = match board_options {
            Some(o) => *o,
            None => BoardOptions::default(),
        };

        let (map_size_x, map_size_y) = options.map_size;
        let mut tile_map = TileMap::new_empty(map_size_x, map_size_y);
        tile_map.set_mines(options.mine_count);

        #[cfg(feature = "debug")]
        info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::WindowAdaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        info!("Board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2f32), -(board_size.y / 2f32), 0f32) + offset
            }
            BoardPosition::CustomPosition(pos) => pos,
        };

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        let mut safe_start = None;

        info!("Spawning board");
        let board_entity = commands
            .spawn((
                Name::new("Board"),
                SpatialBundle {
                    transform: Transform::from_translation(board_position),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(
                            board_size.x / 2f32,
                            board_size.y / 2f32,
                            0f32,
                        ),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();

        if options.safe_start_enabled {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
            marked_tiles: Vec::new(),
        });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let mut commands = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..default()
                    },
                    texture: board_assets.tile_material.texture.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2f32),
                        (y as f32 * tile_size) + (tile_size / 2f32),
                        1f32,
                    ),
                    ..default()
                });

                commands
                    .insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinates);

                commands.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                color: board_assets.covered_tile_material.color,
                                ..default()
                            },
                            texture: board_assets.covered_tile_material.texture.clone(),
                            transform: Transform::from_xyz(0f32, 0f32, 2f32),
                            ..default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);

                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Mine => {
                        commands.insert(Mine);
                        commands.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0f32, 0f32, 1f32),
                                texture: board_assets.mine_material.texture.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::MineNeighbor(mine_count) => {
                        commands.insert(MineNeighbor { count: *mine_count });
                        commands.with_children(|parent| {
                            parent.spawn(Self::mine_count_text_bundle(
                                *mine_count,
                                board_assets,
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => {}
                }
            }
        }
    }

    fn mine_count_text_bundle(
        count: u8,
        board_assets: &BoardAssets,
        font_size: f32,
    ) -> Text2dBundle {
        let color = board_assets.mine_counter_color(count);

        let text_style = TextStyle {
            color,
            font: board_assets.mine_counter_font.clone(),
            font_size,
        };

        let text =
            Text::from_section(count.to_string(), text_style).with_alignment(TextAlignment::Center);

        Text2dBundle {
            text,
            transform: Transform::from_xyz(0f32, 0f32, 1f32),
            ..default()
        }
    }

    fn adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.resolution.width() / width as f32;
        let max_height = window.resolution.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        info!("Performing recursive despawn of entities");
        commands.entity(board.entity).despawn_recursive();
    }
}
