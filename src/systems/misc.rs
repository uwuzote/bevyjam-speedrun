use crate::*;
use bevy::prelude::*;

pub fn quit_game(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
