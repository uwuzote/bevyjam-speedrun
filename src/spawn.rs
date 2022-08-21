use bevy::prelude::*;
use crate::textures::*;
use crate::comps::{Demon, Tile};
use crate::consts::{TILE_SCALE, PIXEL_SCALE};

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    #[bundle]
    spritesheet: SpriteSheetBundle
}

#[derive(Bundle)]
pub struct DemonBundle {
    demon: Demon,
    #[bundle]
    spritesheet: SpriteSheetBundle
}

impl TileBundle {
    pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
        TileBundle {
            tile: Tile,
            spritesheet: SpriteSheetBundle {
                texture_atlas: atlas.0.clone_weak(),
                transform: Transform::from_xyz(
                    TILE_SCALE * x, TILE_SCALE * y, 0.0
                ).with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
                sprite: TextureAtlasSprite {
                    index: idx,
                    ..default()
                },
                ..default()
            }
        }
    }
}

impl DemonBundle {
    pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
        DemonBundle {
            demon: Demon,
            spritesheet: SpriteSheetBundle {
                texture_atlas: atlas.0.clone_weak(),
                transform: Transform::from_xyz(
                    TILE_SCALE * x, TILE_SCALE * y, 1.0
                ).with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
                sprite: TextureAtlasSprite {
                    index: idx,
                    ..default()
                },
                ..default()
            }
        }
    }
}