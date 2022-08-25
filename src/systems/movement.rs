use crate::*;
use bevy::prelude::*;

fn movement_check<'a>(newpos: Vec3, all: impl Iterator<Item = &'a Transform>) -> bool {
    for this in all {
        if newpos == this.translation {
            return false;
        }
    }

    true
}

pub fn change_active_demon(
    keys: Res<Input<KeyCode>>,
    mut cmd: Commands,
    mut query: Query<(Entity, Option<&ActiveDemon>), With<Demon>>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    };

    assert!(!query.is_empty(), "NO DEMONS TO SWITCH ON");

    let mut query_iter = query.iter_mut();
    let mut first_elem = None;

    for (e, demon) in query_iter.by_ref() {
        if first_elem.is_none() {
            first_elem = Some(e);
        }

        if demon.is_some() {
            cmd.entity(e).remove::<ActiveDemon>();

            break;
        }
    }

    let target = if let Some((e, _)) = query_iter.next() {
        e
    } else {
        first_elem.unwrap()
    };

    cmd.entity(target).insert(ActiveDemon);
}

pub fn move_active_demon(
    mut active: Query<&mut Transform, With<ActiveDemon>>,
    demons: Query<&Transform, (With<Demon>, Without<ActiveDemon>)>,
    keys: Res<Input<KeyCode>>,
) {
    let mut active = active.single_mut();

    if keys.just_pressed(KeyCode::D) {
        let newpos = active.translation + Vec3::new(STEP_SIZE, 0.0, 0.0);

        if movement_check(newpos, demons.iter()) {
            active.translation = newpos;
        }
    }

    if keys.just_pressed(KeyCode::A) {
        let newpos = active.translation + Vec3::new(-STEP_SIZE, 0.0, 0.0);

        if movement_check(newpos, demons.iter()) {
            active.translation = newpos;
        }
    }

    if keys.just_pressed(KeyCode::W) {
        let newpos = active.translation + Vec3::new(0.0, STEP_SIZE, 0.0);

        if movement_check(newpos, demons.iter()) {
            active.translation = newpos;
        }
    }

    if keys.just_pressed(KeyCode::S) {
        let newpos = active.translation + Vec3::new(0.0, -STEP_SIZE, 0.0);

        if movement_check(newpos, demons.iter()) {
            active.translation = newpos;
        }
    }
}
