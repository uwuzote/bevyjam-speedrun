use crate::*;
use bevy::prelude::*;

pub fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

pub fn spawn_koci4(
    mut cmd: Commands,
    tiles: Res<TileSheet>,
    demons: Res<DemonSheet>,
    anim_query: Query<Entity, With<Animator>>,
) {
    let e1 = cmd
        .spawn_bundle(DemonBundle::new(
            ([0.0, 0.0], 0, &demons),
            DemonInventory::new(1, &[], &[(Item::Sulfur, 255)]).unwrap(),
        ))
        .insert(ActiveDemon)
        .id();
    let e2 = cmd
        .spawn_bundle(DemonBundle::new(
            ([-1.0, 0.0], 0, &demons),
            DemonInventory::new(1, &[], &[]).unwrap(),
        ))
        .id();

    cmd.entity(anim_query.single()).push_children(&[e1, e2]);

    cmd.spawn_bundle(tile_sprite_bundle([1.0, 1.0], 1, &tiles))
        .insert(Container(Storage::new(
            60,
            &[(Item::Sulfur, 3), (Item::Pebbles, 100), (Item::Hellfire, 4)],
        )));
}
