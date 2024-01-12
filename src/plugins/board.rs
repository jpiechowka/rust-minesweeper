use bevy::prelude::*;

use crate::components::Coordinates;
use crate::resources::{BoardOptions, BoardPosition, TileMap, TileSize};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window>,
    ) {
        let window = windows.single();

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

                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _) in line.iter().enumerate() {
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::DARK_GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2f32),
                                    (y as f32 * tile_size) + (tile_size / 2f32),
                                    1f32,
                                ),
                                ..default()
                            })
                            .insert(Name::new(format!("Tile: ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
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
