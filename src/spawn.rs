use crate::comps::Demon;
use crate::consts::{DEMON_Z_POS, PIXEL_SCALE, TILE_SCALE, TILE_Z_POS};
use crate::items::DemonInventory;
use crate::textures::*;
use bevy::prelude::*;

pub fn tile_sprite_bundle(
    [x, y]: [f32; 2],
    idx: usize,
    atlas: &Res<TileSheet>,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas: atlas.0.clone_weak(),
        transform: Transform::identity()
            .with_translation(Vec3::new(TILE_SCALE * x, TILE_SCALE * y, TILE_Z_POS))
            .with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
        sprite: TextureAtlasSprite {
            index: idx,
            ..default()
        },
        ..default()
    }
}

pub fn demon_sprite_bundle(
    [x, y]: [f32; 2],
    idx: usize,
    atlas: &Res<TileSheet>,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas: atlas.0.clone_weak(),
        transform: Transform::identity()
            .with_translation(Vec3::new(TILE_SCALE * x, TILE_SCALE * y, DEMON_Z_POS))
            .with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
        sprite: TextureAtlasSprite {
            index: idx,
            ..default()
        },
        ..default()
    }
}

#[derive(Clone, Bundle)]
pub struct DemonBundle {
    pub demon: Demon,
    pub inventory: DemonInventory,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl DemonBundle {
    pub fn new(
        (atlas, idx, coords): (&Res<TileSheet>, usize, [f32; 2]),
        inventory: DemonInventory,
    ) -> Self {
        DemonBundle {
            sprite: demon_sprite_bundle(coords, idx, atlas),
            demon: Demon,
            inventory,
        }
    }
}

// #[derive(Bundle)]
// pub struct TileSpriteBundle {
//     tile: Tile,
//     #[bundle]
//     spritesheet: SpriteSheetBundle
// }

// #[derive(Bundle)]
// pub struct DemonSpriteBundle {
//     demon: Demon,
//     #[bundle]
//     spritesheet: SpriteSheetBundle
// }

// impl TileSpriteBundle {
//     pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
//         TileSpriteBundle {
//             tile: Tile,
//             spritesheet: SpriteSheetBundle {
//                 texture_atlas: atlas.0.clone_weak(),
//                 transform: Transform::from_xyz(
//                     TILE_SCALE * x, TILE_SCALE * y, 0.0
//                 ).with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
//                 sprite: TextureAtlasSprite {
//                     index: idx,
//                     ..default()
//                 },
//                 ..default()
//             }
//         }
//     }
// }

// impl DemonSpriteBundle {
//     pub fn new([x, y]: [f32; 2], idx: usize, atlas: &Res<TileSheet>) -> Self {
//         DemonSpriteBundle {
//             demon: Demon,
//             spritesheet: SpriteSheetBundle {
//                 texture_atlas: atlas.0.clone_weak(),
//                 transform: Transform::from_xyz(
//                     TILE_SCALE * x, TILE_SCALE * y, 1.0
//                 ).with_scale(Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0)),
//                 sprite: TextureAtlasSprite {
//                     index: idx,
//                     ..default()
//                 },
//                 ..default()
//             }
//         }
//     }
// }
