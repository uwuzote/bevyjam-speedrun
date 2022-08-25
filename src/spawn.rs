use crate::comps::Demon;
use crate::consts::*;
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
            .with_translation(Vec3::new(STEP_SIZE * x, STEP_SIZE * y, TILE_Z_POS))
            .with_scale(Vec3::new(TILE_SCALE, TILE_SCALE, 1.0)),
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
    atlas: &Res<DemonSheet>,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas: atlas.0.clone_weak(),
        transform: Transform::identity()
            .with_translation(Vec3::new(STEP_SIZE * x, STEP_SIZE * y, DEMON_Z_POS))
            .with_scale(Vec3::new(DEMON_SCALE, DEMON_SCALE, 1.0)),
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
        (coords, idx, atlas): ([f32; 2], usize, &Res<DemonSheet>),
        inventory: DemonInventory,
    ) -> Self {
        DemonBundle {
            sprite: demon_sprite_bundle(coords, idx, atlas),
            demon: Demon,
            inventory,
        }
    }
}
