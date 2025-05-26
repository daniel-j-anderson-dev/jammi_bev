use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);
impl Velocity {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
    pub const ZERO: Self = Self(Vec3::ZERO);
    pub const ONE: Self = Self(Vec3::ONE);
    pub const X: Self = Self(Vec3::X);
    pub const Y: Self = Self(Vec3::Y);
    pub const Z: Self = Self(Vec3::Z);
}
