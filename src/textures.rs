use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct TextureSheet(pub Handle<TextureAtlas>);

impl FromWorld for TextureSheet {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let texture_handle = asset_server.load("textures.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // commands.spawn_bundle(Camera2dBundle::default());
        // commands
        //     .spawn_bundle(SpriteSheetBundle {
        //         texture_atlas: texture_atlas_handle,
        //         transform: Transform::from_scale(Vec3::splat(6.0)),
        //         ..default()
        //     });

        TextureSheet(texture_atlas_handle)
    }
}