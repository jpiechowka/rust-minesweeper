use bevy::prelude::Component;
#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Mine;
