use bevy::{asset::AssetServerError, prelude::*};
use std::convert::From;
use std::ops::{Deref, DerefMut};

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
    };
}

macro_rules! typed_vec {
    ($wrap:ident / $path:literal of $t:ty) => {
        impl $wrap {
            #[cfg(not(target_family = "wasm"))]
            pub fn load(asset_server: &AssetServer) -> Result<Self, AssetServerError> {
                Ok($wrap(
                    asset_server
                        .load_folder($path)?
                        .into_iter()
                        .map(HandleUntyped::typed::<$t>)
                        .collect::<Vec<_>>(),
                ))
            }
        }
    };

    ($wrap:ident / [ $($path:literal),+ ]) => {
        impl $wrap {
            #[cfg(target_family = "wasm")]
            pub fn load(asset_server: &AssetServer) -> Result<Self, ()> {
                Ok($wrap(
                    vec![$($path),+]
                    .into_iter()
                    .map(|path| asset_server.load(path))
                    .collect::<Vec<_>>()
                ))
            }
        }
    }
}

asset!(FontGameCommands(Handle<Font>): "font-game-commands.ttf");

wrap!(DemonSheet(Vec<Handle<Image>>));
typed_vec!(DemonSheet / "demons" of Image);
typed_vec!(DemonSheet / [
    "demons/00d-koci4.png",
    "demons/00s-koci4.png",
    "demons/01d-exban1.png",
    "demons/01s-exban1.png"
]);

wrap!(ItemSheet(Vec<Handle<Image>>));
typed_vec!(ItemSheet / "items" of Image);
typed_vec!(ItemSheet / [
    "items/00-hellfire.png",
    "items/01-infernoflames.png"
]);

wrap!(TileSheet(Vec<Handle<Image>>));
typed_vec!(TileSheet / "tiles" of Image);
typed_vec!(TileSheet / [
    "tiles/00-infernoflames.png",
    "tiles/01-chest.png"
]);

pub fn load_assets(world: &mut World) {
    let server = world
        .get_resource::<AssetServer>()
        .expect("No asset server")
        .clone();

    world.insert_resource(FontGameCommands::load(&server));
    world.insert_resource(DemonSheet::load(&server).expect("DemonSheet load"));
    world.insert_resource(ItemSheet::load(&server).expect("ItemSheet load"));
    world.insert_resource(TileSheet::load(&server).expect("TileSheet load"));
}
