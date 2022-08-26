use bevy::prelude::*;

// #[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Tile;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Demon;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActiveDemon;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Animator;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Highlighter;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiRoot;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiStorageSection;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiDemonInvNode;

#[derive(Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiStorageInvNode;

#[derive(Debug, Clone, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HighlighterTexture(pub Handle<Image>);
