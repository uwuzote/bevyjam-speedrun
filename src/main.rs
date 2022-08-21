#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textures;
mod spawn;

use bevy::{prelude::*, window::WindowMode, render::texture::ImageSettings};
use textures::*;
use spawn::*;

const CLEAR_COLOR: ClearColor = ClearColor(Color::rgb(0.4, 0.0, 0.0));

fn main() {
    #[cfg(not(target_family = "wasm"))]
    std::env::set_var("WGPU_BACKEND", "Vulkan");

    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
           title: "<Koci4 moment>".into(),
           mode: WindowMode::BorderlessFullscreen,
           ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<TileSheet>()
        .init_resource::<ItemSheet>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_koci4)
        // .add_system(animate_demons)
        .run();
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn spawn_koci4(mut cmd: Commands, tiles: Res<TileSheet>) {
    cmd.spawn_bundle(DemonBundle::new([0.0, 0.0], 0, &tiles));
    cmd.spawn_bundle(TileBundle::new([1.0, 1.0], 1, &tiles));
    cmd.spawn_bundle(TileBundle::new([0.0, 1.0], 2, &tiles));
    cmd.spawn_bundle(TileBundle::new([1.0, 0.0], 2, &tiles));
}

// fn animate_demons(mut query: Query<&mut Transform, With<Demon>>, time: Res<Time>) {
//     for mut demon in &mut query {
//         demon.translation += anim;
//     };
// }

// fn animate_sprite(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
//         }
//     }
// }
