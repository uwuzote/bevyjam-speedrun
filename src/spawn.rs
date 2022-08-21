use bevy::prelude::*;
use crate::textures::*;

#[derive(Default, Component)]
pub struct Tile;

pub const TILE_SCALE: f32 = 3.0;
pub const TILE_OFF: f32 = TILE_SCALE * 24.0;

// #[derive(Component)]
// pub struct Storage...

#[derive(Default, Component)]
pub struct Demon;

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    #[bundle]
    spritesheet: SpriteSheetBundle
}

impl TileBundle {
    pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
        TileBundle {
            tile: Tile,
            spritesheet: SpriteSheetBundle {
                texture_atlas: atlas.0.clone_weak(),
                transform: Transform::from_xyz(TILE_OFF * x, TILE_OFF * y, 0.0).with_scale(Vec3::new(TILE_SCALE, TILE_SCALE, 1.0)),
                sprite: TextureAtlasSprite {
                    index: idx,
                    ..default()
                },
                ..default()
            }
        }
    }
}

#[derive(Bundle)]
pub struct DemonBundle {
    demon: Demon,
    #[bundle]
    spritesheet: SpriteSheetBundle
}

impl DemonBundle {
    pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
        DemonBundle {
            demon: Demon,
            spritesheet: SpriteSheetBundle {
                texture_atlas: atlas.0.clone_weak(),
                transform: Transform::from_xyz(TILE_OFF * x, TILE_OFF * y, 1.0).with_scale(Vec3::new(TILE_SCALE, TILE_SCALE, 1.0)),
                sprite: TextureAtlasSprite {
                    index: idx,
                    ..default()
                },
                ..default()
            }
        }
    }
}