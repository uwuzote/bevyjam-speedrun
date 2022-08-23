use bevy::prelude::*;

fn normalize_item_count(pair: (Item, u8)) -> (Item, u8) {
    match pair {
        (_, 0) => (Item::Nothing, 0),
        x => x
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum Item {
    #[default]
    Nothing = 0,
    Hellfire,
    InfernoFlames,
    Pebbles,
    Sulfur,
}

impl Item {
    pub fn texture_id(&self) -> usize {
        (*self) as u8
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Storage<const CAP: usize> ([(Item, u8); CAP]);

impl<const CAP: usize> Default for Storage<CAP> {
    fn default() -> Self {
        Storage([(Item::Nothing, 0); CAP])
    }
}

impl<const CAP: usize> Storage<CAP> {
    /// Panics if slice len > CAP
    pub fn from_slice(slice: &'_ [(Item, u8)]) -> Self {
        let mut this = Self::default();

        for (i, pair) in slice.into_iter().enumerate() {
            this.0[i] = normalize_item_count(*pair);
        }

        this
    }

    pub fn new(inner: [(Item, u8); CAP]) -> Self {
        Storage(inner)
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

impl<const CAP: usize> std::ops::Deref for Storage<CAP> {
    type Target = [(Item, u8); CAP];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const CAP: usize> std::ops::DerefMut for Storage<CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, PartialEq, Eq, Default, Component)]
pub struct UnsizedStorage {
    pub cap: usize,
    pub data: Box<Storage<60>>
};

// #[derive(Clone, PartialEq, Eq, Default, Component)]
// pub struct Container<const CAP: usize> (pub Storage<CAP>);

#[derive(Clone, PartialEq, Eq, Component)]
pub enum DemonInventory {
    One([Item; 3], Box<Storage<6>>), // 6
    Two([Item; 4], Box<Storage<12>>), // 12
    Three([Item; 5], Box<Storage<20>>), // 25
    Four([Item; 6], Box<Storage<50>>) // 50
}

impl DemonInventory {
    pub fn new(souls: usize, content: &[Item], items: Box<Storage>) -> Self {
        match souls {
            1 => {
                Self::One(
                    {
                        let mut this = [Item::Nothing; 3];

                        for (i, item) in content.into_iter().enumerate() {
                            this[i] = item;
                        }

                        this
                    },
                    {
                        let mut 
                    }
                )
            }
        }
    }
}