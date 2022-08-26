use crate::*;
use bevy::prelude::*;

pub fn tile_sprite_bundle([x, y]: [f32; 2], texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        transform: Transform::identity()
            .with_translation(Vec3::new(STEP_SIZE * x, STEP_SIZE * y, TILE_Z_POS))
            .with_scale(Vec3::new(TILE_SCALE, TILE_SCALE, 1.0)),
        texture,
        ..default()
    }
}

pub fn demon_sprite_bundle([x, y]: [f32; 2], texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        transform: Transform::identity()
            .with_translation(Vec3::new(STEP_SIZE * x, STEP_SIZE * y, DEMON_Z_POS))
            .with_scale(Vec3::new(DEMON_SCALE, DEMON_SCALE, 1.0)),
        texture,
        ..default()
    }
}

pub fn highlighter_sprite_bundle([x, y]: [f32; 2], texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        transform: Transform::identity()
            .with_translation(Vec3::new(STEP_SIZE * x, STEP_SIZE * y, HIGHLIGHTER_Z_POS))
            .with_scale(Vec3::new(DEMON_SCALE, DEMON_SCALE, 1.0)),
        texture,
        ..default()
    }
}

#[derive(Clone, Bundle)]
pub struct DemonBundle {
    pub demon: Demon,
    pub inventory: DemonInventory,
    pub highlighter: HighlighterTexture,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl DemonBundle {
    pub fn new(
        coords: [f32; 2],
        texture: Handle<Image>,
        high_texture: Handle<Image>,
        inventory: DemonInventory,
    ) -> Self {
        DemonBundle {
            sprite: demon_sprite_bundle(coords, texture),
            highlighter: HighlighterTexture(high_texture),
            demon: Demon,
            inventory,
        }
    }
}

#[derive(Clone, Bundle)]
pub struct HighlighterBundle {
    pub high: Highlighter,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl HighlighterBundle {
    pub fn new((coords, texture): ([f32; 2], Handle<Image>)) -> Self {
        HighlighterBundle {
            sprite: highlighter_sprite_bundle(coords, texture),
            high: Highlighter,
        }
    }
}
