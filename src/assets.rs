use bevy::{prelude::*, asset::AssetServerError};
use std::ops::{Deref, DerefMut};
use std::convert::From;

macro_rules! wrap {
    ($wrap:ident($of:ty)) => {
        #[derive(Eq, PartialEq, Debug)]
        pub struct $wrap(pub $of);

        impl Deref for $wrap {
            type Target = $of;

            fn deref(&self) -> &<Self as Deref>::Target {
                &self.0
            }
        }

        impl DerefMut for $wrap {
            fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
                &mut self.0
            }
        }

        impl From<$of> for $wrap {
            fn from(inner: $of) -> Self {
                $wrap(inner)
            }
        }
    };
}

macro_rules! asset {
    ($wrap:ident, $path:literal) => {
        impl $wrap {
            #[inline]
            pub fn load(asset_server: &AssetServer) -> Self {
                $wrap(asset_server.load($path))
            }
        }
    };

    ($wrap:ident(many $path:literal) ) => {
        wrap!($wrap(Vec<HandleUntyped>));

        impl $wrap {
            #[inline]
            pub fn load(asset_server: &AssetServer) -> Result<Self, AssetServerError> {
                Ok($wrap(asset_server.load_folder($path)?))
            }
        }
    };

    ($wrap:ident($of:ty): $path:literal ) => {
        wrap!($wrap($of));
        asset!($wrap, $path);
    }
}

asset!(FontGameCommands(Handle<Font>): "font-game-commands.ttf");
wrap!(DemonSheet(Vec<Handle<Image>>));
impl DemonSheet {
    pub fn load(asset_server: &AssetServer) -> Result<Self, AssetServerError> {
        Ok(DemonSheet(
            asset_server.load_folder("demons")?
            .into_iter()
            .map(HandleUntyped::typed::<Image>)
            .collect::<Vec<_>>()
        ))
    }
}

pub fn load_assets(world: &mut World) {
    let server = world
        .get_resource::<AssetServer>()
        .expect("No asset server")
        .clone();

    world.insert_resource(FontGameCommands::load(&server));
    world.insert_resource(DemonSheet::load(&server).expect("DemonSheet load"));
}