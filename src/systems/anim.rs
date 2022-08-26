use crate::*;
use bevy::prelude::*;

// pub fn spawn_animator(mut cmd: Commands) {
//     cmd.spawn_bundle(SpatialBundle::visible_identity())
//         .insert(Animator);
// }

// pub fn spawn_highlighter(
//     mut cmd: Commands,
//     animator: Query<Entity, With<Animator>>,
//     active: Query<(&TextureAtlasSprite, &Transform), With<ActiveDemon>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let (sprite, trans) = active.single();

//     cmd.entity(animator.single()).with_children(|cmd| {
//         let mut bundle = highlighter_sprite_bundle(sprite.index + DEMON_SHADE_ADDER, asset_server.load("demons.png"));

//         bundle.transform.translation.x = trans.translation.x;
//         bundle.transform.translation.y = trans.translation.y;

//         cmd.spawn_bundle(bundle).insert(Highlighter);
//     });
// }

pub fn animator_animation(mut query: Query<&mut Transform, With<Animator>>, time: Res<Time>) {
    let mut anim = query.single_mut();
    let smth = (time.seconds_since_startup() as f32).sin();

    anim.translation = Vec3::new(0.0, smth * STEP_SIZE / 6.0, 0.0);
}

pub fn highlighter_animation(mut high: Query<&mut Transform, With<Highlighter>>, time: Res<Time>) {
    let mut high = high.single_mut();
    let sin = ((time.seconds_since_startup() as f32) / 1.4).sin();
    // let smth = 1.0 + (sin / 2.0 + 0.5) / 7.0 + 0.1;
    let smth = 1.172 + (sin / 14.0);

    high.scale = Vec3::new(DEMON_SCALE * smth, DEMON_SCALE * smth, 1.0);
}

pub fn update_highlighter(
    mut high: Query<(&mut Transform, &mut Handle<Image>), (With<Highlighter>, Without<ActiveDemon>)>,
    active: Query<(&Transform, &HighlighterTexture), (With<ActiveDemon>, Without<Highlighter>)>,
) {
    let mut high = high.single_mut();
    let (new_trans, handle) = active.single();

    high.0.translation.x = new_trans.translation.x;
    high.0.translation.y = new_trans.translation.y;

    *high.1 = handle.0.clone_weak();
}
