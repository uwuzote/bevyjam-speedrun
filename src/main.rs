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

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum SpawningStage {
    Animator,
    Koci4,
    Highlighter,
}

#[derive(Component)]
#[allow(clippy::upper_case_acronyms)]
struct DEBUG(&'static str);

#[allow(non_snake_case)]
fn DEBUG_SYSTEM() {}

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
        // Startup
        .add_startup_stage_before(
            StartupStage::Startup,
            SpawningStage::Animator,
            SystemStage::single(spawn_animator),
        )
        .add_startup_stage_after(
            SpawningStage::Animator,
            SpawningStage::Koci4,
            SystemStage::single(spawn_koci4),
        )
        .add_startup_stage_after(
            SpawningStage::Koci4,
            SpawningStage::Highlighter,
            SystemStage::single(spawn_highlighter),
        )
        .add_startup_system(spawn_camera)
        .add_startup_system(draw_ui)
        // Core
        .add_system(toggle_menu)
        .add_system(animator_animation)
        .add_system(highlighter_animation)
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(change_active_demon)
                .with_system(move_active_demon.after(change_active_demon))
                .with_system(update_highlighter.after(move_active_demon)),
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
                .with_system(spawn_inventory.before(show_ui)),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(hide_ui)
                .with_system(despawn_inventory.after(hide_ui)),
        )
        .run();
}
