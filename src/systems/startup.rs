use crate::*;
use bevy::prelude::*;

pub fn startup_spawns(world: &mut World) {
    let demons = world.get_resource::<DemonSheet>().unwrap();

    let demon = demons[0].clone();
    let demon2 = demons[2].clone();
    let high = demons[1].clone();
    let high2 = demons[3].clone();

    let _animator = world
        .spawn()
        .insert_bundle(SpatialBundle::visible_identity())
        .insert(Animator)
        .with_children(|parent| {
            let _active = parent
                .spawn_bundle(DemonBundle::new(
                    [0.0, 0.0],
                    demon,
                    high.clone_weak(),
                    DemonInventory::new(1, &[], &[(Item::Sulfur, 255)]).unwrap(),
                ))
                .insert(ActiveDemon)
                .id();

            let _high = parent
                .spawn_bundle(HighlighterBundle::new(([0.0, 0.0], high)))
                .id();

            let _second = parent
                .spawn_bundle(DemonBundle::new(
                    [1.0, 1.0],
                    demon2,
                    high2.clone_weak(),
                    DemonInventory::new(2, &[], &[(Item::Pebbles, 25)]).unwrap(),
                ))
                .id();
        })
        .id();
}
