use bevy::prelude::*;

use crate::components::Coordinates;

#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent {
    pub coordinates: Coordinates,
}
