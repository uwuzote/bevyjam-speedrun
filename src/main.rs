#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod comps;
pub mod consts;
pub mod font_loader;
pub mod items;
pub mod spawn;
pub mod systems;
pub mod textures;

use crate::systems::*;
pub use crate::{comps::*, consts::*, font_loader::*, items::*, spawn::*, textures::*};
use bevy::{app::AppExit, prelude::*, render::texture::ImageSettings, window::WindowMode};

pub const FULL_SIZE: Size<Val> = Size {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    // MainMenu,
    Game,
    Items,
}

#[derive(Component)]
#[allow(clippy::upper_case_acronyms)]
struct DEBUG(&'static str);

#[allow(non_snake_case)]
fn DEBUG_SYSTEM(query: Query<(&DEBUG, &GlobalTransform, &ComputedVisibility)>) {
    for (DEBUG(name), trans, vis) in &query {
        eprintln!(
            "{:?}: vis = {} at {}",
            name,
            vis.is_visible(),
            trans.translation()
        );
    }
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    std::env::set_var("WGPU_BACKEND", "Vulkan");

    App::new()
        .add_system(DEBUG_SYSTEM)
        // Settings
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
            title: "<Koci4 moment>".into(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Setup
        .init_resource::<TileSheet>()
        .init_resource::<ItemSheet>()
        .init_resource::<DemonSheet>()
        .init_resource::<FontHandle>()
        .add_state(GameState::Game)
        .add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new()
                .with_system(spawn_animator)
                .with_system(draw_ui),
        )
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_koci4)
        // Core
        .add_system(toggle_menu)
        .add_system(animator_animation)
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(change_active_demon)
                .with_system(move_active_demon.after(change_active_demon)),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Items)
                .with_system(quit_game)
                .with_system(toggle_storage_menu),
        )
        // UI
        .add_system_set(
            SystemSet::on_enter(GameState::Items)
                .with_system(show_ui)
                .with_system(spawn_inventory),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(hide_ui)
                .with_system(despawn_inventory),
        )
        .run();
}
