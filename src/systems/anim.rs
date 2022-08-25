use crate::*;
use bevy::prelude::*;

pub fn spawn_animator(mut cmd: Commands) {
    cmd.spawn_bundle(SpatialBundle::visible_identity())
        .insert(Animator);
}

pub fn animator_animation(mut query: Query<&mut Transform, With<Animator>>, time: Res<Time>) {
    let mut anim = query.single_mut();
    let smth = (time.seconds_since_startup() as f32).sin();

    anim.translation = Vec3::new(0.0, smth * STEP_SIZE / 6.0, 0.0);
}
