use bevy::prelude::*;

pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect {
        min: vec2(x, y),
        max: vec2(x + w, y + h),
    }
}
