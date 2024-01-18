use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use crate::components::{Coordinates, Mine, MineNeighbor};
use crate::plugins::{Bounds2, TileTriggerEvent};
use crate::resources::{Board, BoardOptions, BoardPosition, Tile, TileMap, TileSize};
use crate::systems::{handle_mouse_input, trigger_event_handler, uncover_tiles};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        app.add_systems(Update, handle_mouse_input);
        app.add_systems(Update, trigger_event_handler);
        app.add_systems(Update, uncover_tiles);
        app.add_event::<TileTriggerEvent>();
        info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = window_query.single();

        info!("Loading assets");
        let font: Handle<Font> = asset_server.load("fonts/symtext/Symtext.ttf");
        let mine_sprite: Handle<Image> = asset_server.load("sprites/Mine.png");

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

        info!("Spawning board");
        commands
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
                            color: Color::ANTIQUE_WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
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
                    Color::DARK_GRAY,
                    Color::GRAY,
                    &mut covered_tiles,
                    mine_sprite,
                    font,
                );
            });

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
        });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        background_color: Color,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        mine_sprite: Handle<Image>,
        font: Handle<Font>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let mut commands = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: background_color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..default()
                    },
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
                                color: covered_tile_color,
                                ..default()
                            },
                            transform: Transform::from_xyz(0f32, 0f32, 2f32),
                            ..default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
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
                                texture: mine_sprite.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::MineNeighbor(mine_count) => {
                        commands.insert(MineNeighbor { count: *mine_count });
                        commands.with_children(|parent| {
                            parent.spawn(Self::mine_count_text_bundle(
                                *mine_count,
                                font.clone(),
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => {}
                }
            }
        }
    }

    fn mine_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
        let color = match count {
            1 => Color::WHITE,
            2 => Color::GREEN,
            3 => Color::YELLOW,
            4 => Color::ORANGE,
            5 => Color::RED,
            _ => Color::PURPLE,
        };

        let text_style = TextStyle {
            color,
            font,
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
}
