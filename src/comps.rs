use bevy::prelude::*;

// #[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Tile;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Demon;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActiveDemon;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Animator;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Highlighter;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiRoot;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiStorageSection;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiDemonInvNode;

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiStorageInvNode;
