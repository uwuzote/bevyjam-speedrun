use bevy::prelude::*;

pub const PIXEL_SCALE: f32 = 3.0;

pub const TILE_SIZE: f32 = 24.0;
pub const ITEM_SIZE: f32 = 16.0;

pub const TILE_SCALE: f32 = PIXEL_SCALE * TILE_SIZE;
pub const ITEM_SCALE: f32 = PIXEL_SCALE * ITEM_SIZE;

pub const TILE_Z_POS: f32 = 0.0;
pub const DEMON_Z_POS: f32 = 1.0;

pub const CLEAR_COLOR: ClearColor = ClearColor(Color::rgb(0.4, 0.0, 0.0));