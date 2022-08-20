#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{*, window::WindowMode};

fn main() {
    std::env::set_var("WGPU_BACKEND", "Vulkan");

    App::new()
         .insert_resourse(WindowDescriptor {
            title: "Крутой заголовок окна".into(),
            // decorations: false,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
         })
         .add_plugins(DefaultPlugins)
         .run();
}
