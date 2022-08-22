use bevy::prelude::*;

pub struct FontHandle(pub Handle<Font>);

impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        FontHandle(asset_server.load("font-game-commands.ttf"))
    }
}