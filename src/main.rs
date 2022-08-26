#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity)]

pub mod assets;
pub mod comps;
pub mod consts;
pub mod items;
pub mod spawn;
pub mod systems;

use crate::systems::*;
pub use crate::{assets::*, comps::*, consts::*, items::*, spawn::*};
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
pub enum SpecialStartupStage {
    Resources,
    Spawns,
}

#[derive(Component)]
#[allow(clippy::upper_case_acronyms)]
struct DEBUG(&'static str);

#[allow(non_snake_case, dead_code)]
fn DEBUG_SYSTEM(demons: Res<DemonSheet>) {
    eprintln!("{:?}", demons);
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    std::env::set_var("WGPU_BACKEND", "Vulkan");

    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        // .add_system(DEBUG_SYSTEM)
        // Settings
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
            title: "Koci4 combinations".into(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Startup
        .add_state(GameState::Game)
        .add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_assets.exclusive_system().at_end(),
        )
        .add_startup_stage_after(
            StartupStage::PreStartup,
            SpecialStartupStage::Resources,
            SystemStage::single(load_assets.exclusive_system()),
        )
        .add_startup_stage_after(
            SpecialStartupStage::Resources,
            SpecialStartupStage::Spawns,
            SystemStage::single(startup_spawns.exclusive_system()),
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
