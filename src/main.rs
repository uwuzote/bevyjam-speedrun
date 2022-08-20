#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textures;

use bevy::{prelude::*, window::WindowMode, render::texture::ImageSettings};
use textures::TextureSheet;

const SCALE: f32 = 2.0;
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
           ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<TextureSheet>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_koci4)
        .run();
}

fn spawn_koci4(mut cmd: Commands, textures: Res<TextureSheet>) {
    cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: textures.0.clone_weak(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(SCALE, SCALE, 1.0)),
        ..default()
    });
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

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
