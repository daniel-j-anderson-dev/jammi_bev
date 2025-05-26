use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Hitbox(Rect);
