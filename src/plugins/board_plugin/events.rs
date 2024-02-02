use bevy::prelude::*;

use crate::components::Coordinates;

#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent {
    pub coordinates: Coordinates,
}

#[derive(Debug, Copy, Clone, Event)]
pub struct BoardCompletedEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinates);

#[derive(Debug, Copy, Clone, Event)]
pub struct MineExplosionEvent;
