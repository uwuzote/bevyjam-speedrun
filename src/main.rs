#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textures;
mod spawn;

use bevy::{prelude::*, app::AppExit, window::WindowMode, render::texture::ImageSettings};
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
        .add_state(GameState::Game)
        .add_plugins(DefaultPlugins)
        .init_resource::<TileSheet>()
        .init_resource::<ItemSheet>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_koci4)
        .add_system(toggle_menu)
        .add_system_set(
            SystemSet::on_update(GameState::Game)
            .with_system(move_active_demon)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Items)
            .with_system(quit_game)
        )
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    // MainMenu,
    Game,
    Items,
}

#[derive(Component)]
struct ActiveDemon;

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn quit_game(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn toggle_menu(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>
) {
    if keys.just_pressed(KeyCode::E) {
        let newstate = match state.current() {
            GameState::Game => GameState::Items,
            GameState::Items => GameState::Game
        };

        state.set(newstate).unwrap();
    };
}

fn spawn_koci4(mut cmd: Commands, tiles: Res<TileSheet>) {
    cmd.spawn_bundle(DemonBundle::new([0.0, 0.0], 0, &tiles)).insert(ActiveDemon);
    cmd.spawn_bundle(TileBundle::new([1.0, 1.0], 1, &tiles));
    cmd.spawn_bundle(TileBundle::new([0.0, 1.0], 2, &tiles));
    cmd.spawn_bundle(TileBundle::new([1.0, 0.0], 2, &tiles));
}

fn move_active_demon(
    mut query: Query<&mut Transform, With<ActiveDemon>>,
    keys: Res<Input<KeyCode>>
) {
    let mut active = query.single_mut();

    if keys.just_pressed(KeyCode::D) {
        active.translation += Vec3::new(TILE_OFF, 0.0, 0.0);
    };

    if keys.just_pressed(KeyCode::A) {
        active.translation += Vec3::new(-TILE_OFF, 0.0, 0.0);
    };

    if keys.just_pressed(KeyCode::W) {
        active.translation += Vec3::new(0.0, TILE_OFF, 0.0);
    };

    if keys.just_pressed(KeyCode::S) {
        active.translation += Vec3::new(0.0, -TILE_OFF, 0.0);
    };
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
