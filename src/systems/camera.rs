use bevy::prelude::*;

#[derive(Component)]
pub struct Minesweeper2dCamera;

pub fn setup_2d_camera(mut commands: Commands) {
    info!("Setting up 2D camera");
    commands.spawn((Camera2dBundle::default(), Minesweeper2dCamera));
}
