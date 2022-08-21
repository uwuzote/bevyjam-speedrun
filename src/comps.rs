use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Tile;

#[derive(Default, Component)]
pub struct Demon;

#[derive(Component)]
pub struct ActiveDemon;

#[derive(Component, Eq, PartialEq)]
pub struct ItemsMenu;
