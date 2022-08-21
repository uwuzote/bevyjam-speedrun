use bevy::prelude::*;

#[derive(Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tile;

#[derive(Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Demon;

#[derive(Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActiveDemon;

#[derive(Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemsMenu;

#[derive(Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Animator;