use bevy::prelude::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub const MAX_STORAGE: usize = 64;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default, Debug)]
#[repr(u8)]
pub enum Item {
    #[default]
    Nothing = 31,

    Hellfire = 0,
    InfernoFlames,
    Pebbles,
    Sulfur,
}

impl Item {
    pub fn texture_id(&self) -> usize {
        (*self as u8).into()
    }
}

pub type Cell = (Item, u8);

#[derive(Clone, Eq, PartialEq)]
pub struct Storage {
    cap: usize,
    storage: Box<[Cell; MAX_STORAGE]>,
}

impl Storage {
    pub fn new(cap: usize, arr: &[Cell]) -> Self {
        debug_assert!(cap >= arr.len());

        let mut this = Self::with_cap(cap);

        for (i, item) in arr.iter().enumerate() {
            this.storage[i] = *item;
        }

        this
    }

    pub fn with_cap(cap: usize) -> Self {
        debug_assert!(cap <= MAX_STORAGE);

        Storage {
            cap,
            storage: Box::new([(Item::Nothing, 0); MAX_STORAGE]),
        }
    }

    pub fn from<const SIZE: usize>(arr: &[Cell; SIZE]) -> Self {
        Self::new(SIZE, arr)
    }

    pub fn max() -> Self {
        Self::with_cap(MAX_STORAGE)
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn destruct(self) -> (usize, Box<[Cell; MAX_STORAGE]>) {
        (self.cap, self.storage)
    }

    pub fn clone_storage(&self) -> Box<[Cell; MAX_STORAGE]> {
        self.storage.clone()
    }

    pub fn view(&self) -> &[Cell] {
        &self.storage[..self.cap]
    }
}

impl Index<usize> for Storage {
    type Output = Cell;

    fn index(&self, idx: usize) -> &Cell {
        debug_assert!(idx <= self.cap);

        self.storage.index(idx)
    }
}

impl IndexMut<usize> for Storage {
    fn index_mut(&mut self, idx: usize) -> &mut Cell {
        debug_assert!(idx <= self.cap);

        self.storage.index_mut(idx)
    }
}

#[derive(Clone, Eq, PartialEq, Component)]
pub struct Container(pub Storage);

impl Deref for Container {
    type Target = Storage;

    fn deref(&self) -> &Storage {
        &self.0
    }
}

impl DerefMut for Container {
    fn deref_mut(&mut self) -> &mut Storage {
        &mut self.0
    }
}

#[derive(Clone, Eq, PartialEq, Component)]
pub struct DemonInventory {
    souls: usize,
    pub parts: Storage,
    pub inventory: Storage,
}

impl DemonInventory {
    fn souls_to_caps(souls: usize) -> Option<(usize, usize)> {
        match souls {
            1 => Some((3, 6)),
            2 => Some((5, 12)),
            3 => Some((7, 25)),
            4 => Some((20, 64)),
            _ => None,
        }
    }

    pub fn new(souls: usize, parts: &[Cell], inventory: &[Cell]) -> Option<Self> {
        let (prt, inv) = Self::souls_to_caps(souls)?;

        Some(DemonInventory {
            souls,
            parts: Storage::new(prt, parts),
            inventory: Storage::new(inv, inventory),
        })
    }

    pub fn souls(&self) -> usize {
        self.souls
    }
}
