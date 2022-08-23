use crate::consts::{ITEM_SIZE, TILE_SIZE};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct TileSheet(pub Handle<TextureAtlas>);

#[derive(Clone, Eq, PartialEq)]
pub struct ItemSheet(pub Handle<TextureAtlas>);

#[inline]
fn textures_loader(
    filename: &'static str,
    [sizex, sizey]: [f32; 2],
    [countx, county]: [usize; 2],
    world: &mut World,
) -> Handle<TextureAtlas> {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let texture_handle = asset_server.load(filename);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(sizex, sizey), countx, county);

    let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    texture_atlas_handle
}

impl FromWorld for TileSheet {
    #[inline]
    fn from_world(world: &mut World) -> Self {
        TileSheet(textures_loader(
            "tiles.png",
            [TILE_SIZE, TILE_SIZE],
            [8, 4],
            world,
        ))
    }
}

impl FromWorld for ItemSheet {
    #[inline]
    fn from_world(world: &mut World) -> Self {
        ItemSheet(textures_loader(
            "items.png",
            [ITEM_SIZE, ITEM_SIZE],
            [8, 4],
            world,
        ))
    }
}
