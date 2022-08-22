#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod comps;
pub mod consts;
pub mod textures;
pub mod spawn;
pub mod font_loader;

use bevy::{
    prelude::*,
    app::AppExit,
    window::WindowMode,
    render::{
        texture::ImageSettings,
    },
};
use crate::{
    comps::*,
    consts::*,
    textures::*,
    spawn::*,
    font_loader::*
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    // MainMenu,
    Game,
    Items,
}

#[derive(Component)]
struct DEBUG(&'static str);

#[allow(non_snake_case)]
fn DEBUG_SYSTEM(query: Query<(&DEBUG, &GlobalTransform, &ComputedVisibility)>) {
    for (DEBUG(name), trans, vis) in &query {
        eprintln!(
            "{:?}: vis = {} at {}",
            name, vis.is_visible(), trans.translation()
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
        .init_resource::<FontHandle>()
        .add_state(GameState::Game)
        .add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new()
            .with_system(spawn_animator)
            .with_system(draw_ui)
        )
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_koci4)
        // Core
        .add_system(toggle_menu)
        .add_system(animator_animation)
        .add_system_set(
            SystemSet::on_update(GameState::Game)
            .with_system(change_active_demon)
            .with_system(move_active_demon.after(change_active_demon))
        )
        .add_system_set(
            SystemSet::on_update(GameState::Items)
            .with_system(quit_game)
        )
        // UI
        // .add_startup_system(draw_ui) Moved to PreStartup Stage
        .add_system_set(
            SystemSet::on_enter(GameState::Items)
            .with_system(show_ui)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
            .with_system(hide_ui)
        )

        .run();
}

fn spawn_animator(mut cmd: Commands) {
    cmd.spawn_bundle(SpatialBundle::visible_identity()).insert(Animator);
}

fn animator_animation(
    mut query: Query<&mut Transform, With<Animator>>,
    time: Res<Time>
) {
    let mut anim = query.single_mut();
    let smth = (time.seconds_since_startup() as f32).sin();

    anim.translation = Vec3::new(0.0, smth * TILE_SCALE / 6.0, 0.0);
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn draw_ui(mut cmd: Commands, font: Res<FontHandle>) {
    cmd.spawn_bundle(NodeBundle {
        color: Color::NONE.into(),
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
        .insert(ItemsMenu)
        .with_children(|cmd| {
            cmd
                .spawn_bundle(
                    // Create a TextBundle that has a Text with a single section.
                    TextBundle::from_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        "PAUSE",
                        TextStyle {
                            font: font.0.clone_weak(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ) // Set the alignment of the Text
                    .with_text_alignment(TextAlignment::TOP_CENTER)
                    // Set the style of the TextBundle itself.
                    .with_style(Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(5.0),
                            right: Val::Px(15.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
        });
}

fn show_ui(mut query: Query<&mut Visibility, With<ItemsMenu>>) {
    query.single_mut().is_visible = true;
}

fn hide_ui(mut query: Query<&mut Visibility, With<ItemsMenu>>) {
    query.single_mut().is_visible = false;
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

fn spawn_koci4(
    mut cmd: Commands,
    tiles: Res<TileSheet>,
    anim_query: Query<Entity, With<Animator>>
) {
    let e1 = cmd.spawn_bundle(DemonBundle::new([0.0, 0.0], 0, &tiles)).insert(ActiveDemon).id();
    let e2 = cmd.spawn_bundle(DemonBundle::new([-1.0, 0.0], 1, &tiles)).id();

    cmd.entity(anim_query.single()).push_children(&[e1, e2]);

    cmd.spawn_bundle(TileBundle::new([1.0, 1.0], 1, &tiles));
    cmd.spawn_bundle(TileBundle::new([0.0, 1.0], 2, &tiles));
    cmd.spawn_bundle(TileBundle::new([1.0, 0.0], 2, &tiles));
}

fn change_active_demon(
    keys: Res<Input<KeyCode>>,
    mut cmd: Commands,
    mut query: Query<(Entity, Option<&ActiveDemon>), With<Demon>>
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    };

    let mut query_iter = (&mut query).into_iter();

    while let Some((e, demon)) = query_iter.next() {
        if demon.is_some() {
            cmd.entity(e).remove::<ActiveDemon>();

            break;
        };
    };

    if let Some((e, _)) = query_iter.next() {
        cmd.entity(e).insert(ActiveDemon);
    } else {
        cmd.entity((&mut query).into_iter().next().unwrap().0).insert(ActiveDemon);
    };
}

fn move_active_demon(
    mut query: Query<&mut Transform, With<ActiveDemon>>,
    keys: Res<Input<KeyCode>>
) {
    let mut active = query.single_mut();

    if keys.just_pressed(KeyCode::D) {
        active.translation += Vec3::new(TILE_SCALE, 0.0, 0.0);
    };

    if keys.just_pressed(KeyCode::A) {
        active.translation += Vec3::new(-TILE_SCALE, 0.0, 0.0);
    };

    if keys.just_pressed(KeyCode::W) {
        active.translation += Vec3::new(0.0, TILE_SCALE, 0.0);
    };

    if keys.just_pressed(KeyCode::S) {
        active.translation += Vec3::new(0.0, -TILE_SCALE, 0.0);
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
