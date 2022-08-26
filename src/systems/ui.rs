use crate::*;
use bevy::prelude::*;

fn get_current_chest<'c, 'any>(
    active: &Vec3,
    query: impl Iterator<Item = (&'c Container, &'any Transform)>,
) -> Option<&'c Container> {
    for (con, trans) in query {
        if trans.translation == *active {
            return Some(con);
        }
    }

    None
}

pub fn spawn_inventory(
    mut cmd: Commands,
    active: Query<(&DemonInventory, &Transform), With<ActiveDemon>>,
    chests: Query<(&Container, &Transform)>,
    demon_inv_node: Query<Entity, With<UiDemonInvNode>>,
    chest_inv_node: Query<Entity, With<UiStorageInvNode>>,
) {
    let (demon_inv, demon_trans) = active.single();
    let (demon_inv_node, chest_inv_node) = (demon_inv_node.single(), chest_inv_node.single());
    let opt_chest = get_current_chest(&demon_trans.translation, chests.iter());

    cmd.entity(demon_inv_node).with_children(|cmd| {
        for (item, count) in demon_inv.inventory.view() {
            // eprintln!("{:?}", (item, count));
            // cmd.spawn_bundle()
        }
    });
}
pub fn despawn_inventory() {}

pub fn toggle_menu(keys: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::E) {
        let newstate = match state.current() {
            GameState::Game => GameState::Items,
            GameState::Items => GameState::Game,
        };

        state.set(newstate).unwrap();
    };
}

pub fn show_ui(mut query: Query<&mut Visibility, With<UiRoot>>) {
    query.single_mut().is_visible = true;
}

pub fn hide_ui(mut query: Query<&mut Visibility, With<UiRoot>>) {
    query.single_mut().is_visible = false;
}

pub fn toggle_storage_menu(
    mut query: Query<&mut Style, With<UiStorageSection>>,
    keys: Res<Input<KeyCode>>,
) {
    use Display::*;

    if keys.just_pressed(KeyCode::Tab) {
        let mut node = query.single_mut();

        node.display = match node.display {
            Flex => None,
            None => Flex,
        };
    }
}

pub fn draw_ui(mut cmd: Commands, font: Res<FontGameCommands>) {
    cmd.spawn_bundle(NodeBundle {
        color: Color::rgba(0.15, 0.4, 0.3, 0.25).into(),
        style: Style {
            size: FULL_SIZE,
            // justify_content: JustifyContent::SpaceBetween,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    })
    .insert(UiRoot)
    .with_children(|cmd| {
        cmd.spawn_bundle(
            TextBundle::from_section(
                "INVENTORY",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        );

        cmd.spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: FULL_SIZE,
                flex_direction: FlexDirection::Row,
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                style: Style {
                    size: FULL_SIZE,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            })
            .with_children(|cmd| {
                cmd.spawn_bundle(
                    TextBundle::from_section(
                        "DEMON'S",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::BLUE,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(UiDemonInvNode);

                cmd.spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        size: FULL_SIZE,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                });
            });

            cmd.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                style: Style {
                    size: FULL_SIZE,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            })
            .insert(UiStorageSection)
            .with_children(|cmd| {
                cmd.spawn_bundle(
                    TextBundle::from_section(
                        "STORAGE",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::BLUE,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(UiStorageInvNode);

                cmd.spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        size: FULL_SIZE,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });
}
