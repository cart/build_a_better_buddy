use bevy::prelude::*;

use rand::Rng;
use std::{f32::consts::PI, time::Duration};

use crate::game::{
    animate::{AnimateRange, AnimateScale, Ease},
    battle_ground::Pad,
    Z_BUDDY,
};

pub struct BuddyPlugin;

impl Plugin for BuddyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutlineTimer>()
            .add_system(update_outlines)
            .add_system(set_buddy_face)
            .add_system(move_buddy);
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct Slot(pub usize);

const MAX_BUDDIES_PER_SIDE: usize = 3;

impl Slot {
    pub fn new(slot: usize) -> Self {
        if slot >= MAX_BUDDIES_PER_SIDE {
            panic!("invalid buddy slot {slot}");
        }

        Self(slot)
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Component, Default)]
pub struct Buddy;

#[derive(Component, Default)]
pub struct BuddyOutline;

#[derive(Component)]
pub enum BuddyFace {
    Happy,
    Neutral,
}

impl BuddyFace {
    pub fn get_path(&self) -> &'static str {
        match self {
            BuddyFace::Happy => "buddy/face/happy.png",
            BuddyFace::Neutral => "buddy/face/neutral.png",
        }
    }
}

#[derive(Component)]
pub struct BuddyBlink {
    timer: Timer,
    is_blinking: bool,
}

impl Default for BuddyBlink {
    fn default() -> Self {
        Self::new(false)
    }
}

impl BuddyBlink {
    pub fn new(is_blinking: bool) -> Self {
        let mut rng = rand::thread_rng();
        let seconds = if is_blinking {
            rng.gen_range(0.05..0.2)
        } else {
            rng.gen_range(10.0..20.0)
        };
        Self {
            timer: Timer::new(Duration::from_secs_f32(seconds), false),
            is_blinking,
        }
    }

    pub fn blink(&mut self, delta: Duration) -> bool {
        if self.timer.tick(delta).just_finished() {
            *self = BuddyBlink::new(!self.is_blinking);
        }

        self.is_blinking
    }
}

#[derive(Component, Default)]
pub struct BuddyColor(Color);

impl BuddyColor {
    pub fn red() -> Self {
        Self(Color::hex("ad8988").unwrap())
    }

    pub fn blue() -> Self {
        Self(Color::hex("8a89ae").unwrap())
    }
}

impl Default for BuddyFace {
    fn default() -> Self {
        BuddyFace::Happy
    }
}

#[derive(Component)]
pub struct BuddyFaceSprite;

#[derive(Component)]
pub struct BuddyBodySprite;

pub struct OutlineTimer(Timer);

impl Default for OutlineTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, true))
    }
}

#[derive(Bundle, Default)]
pub struct BuddyBundle {
    pub buddy: Buddy,
    pub face: BuddyFace,
    pub blink: BuddyBlink,
    pub slot: Slot,
    pub color: BuddyColor,
    pub wobble: BuddyWobble,
    pub side: Side,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn spawn_buddy(commands: &mut Commands, asset_server: &AssetServer, buddy: BuddyBundle) {
    commands
        .spawn_bundle(buddy)
        .insert(AnimateScale::new(
            Duration::from_secs_f32(0.6),
            Ease::OutBack,
            0.0..1.0,
            false,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("buddy/base.png"),
                    transform: Transform::from_xyz(0.0, 0.0, Z_BUDDY).with_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .insert(BuddyBodySprite);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("buddy/outline.png"),
                    transform: Transform::from_xyz(0.0, 0.0, Z_BUDDY + 0.1)
                        .with_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .insert(BuddyOutline);
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, Z_BUDDY + 0.2)
                        .with_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .insert(BuddyFaceSprite);
        });
}

fn update_outlines(
    time: Res<Time>,
    mut outline_clock: ResMut<OutlineTimer>,
    mut buddy_transforms: Query<&mut Transform, With<BuddyOutline>>,
) {
    if !outline_clock.0.tick(time.delta()).just_finished() {
        return;
    }

    for mut transform in buddy_transforms.iter_mut() {
        loop {
            let old_rotation = transform.rotation;
            let i = rand::thread_rng().gen_range(0u32..5) as f32;
            transform.rotation = Quat::from_rotation_z(i * PI / 2.0);
            if old_rotation != transform.rotation {
                break;
            }
        }
    }
}

fn set_buddy_face(
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut buddies: Query<(&Side, &BuddyFace, &BuddyColor, &mut BuddyBlink), With<Buddy>>,
    mut faces: Query<
        (&mut Handle<Image>, &mut Sprite, &Parent),
        (With<BuddyFaceSprite>, Without<BuddyBodySprite>),
    >,
    mut bodies: Query<(&mut Sprite, &Parent), With<BuddyBodySprite>>,
) {
    for (mut image, mut sprite, parent) in faces.iter_mut() {
        if let Ok((side, face, _, mut blink)) = buddies.get_mut(parent.0) {
            match side {
                Side::Left => {
                    sprite.flip_x = false;
                }
                Side::Right => {
                    sprite.flip_x = true;
                }
            }
            if blink.blink(time.delta()) {
                *image = asset_server.load("buddy/face/blink.png");
            } else {
                *image = asset_server.load(face.get_path());
            }
        }
    }

    for (mut sprite, parent) in bodies.iter_mut() {
        if let Ok((_, _, color, _)) = buddies.get(parent.0) {
            sprite.color = color.0;
        }
    }
}

#[derive(Component)]
pub struct BuddyWobble {
    animate_rotation: AnimateRange,
    animate_translation: AnimateRange,
    flipped: bool,
}

impl Default for BuddyWobble {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen_range(0.0..1.0))
    }
}

impl BuddyWobble {
    pub fn new(flipped: bool, percent: f32) -> Self {
        let rot = PI * 0.05;
        let trans = 10.0;
        let rot_range;
        let trans_range;
        if flipped {
            rot_range = rot..-rot;
            trans_range = -trans..trans;
        } else {
            rot_range = -rot..rot;
            trans_range = trans..-trans;
        }

        let mut rng = rand::thread_rng();
        let duration = Duration::from_secs_f32(rng.gen_range(2.0..5.0));
        let ease = Ease::InOutCirc;
        let mut animate_rotation = AnimateRange::new(duration, ease, rot_range, false);
        let mut animate_translation = AnimateRange::new(duration, ease, trans_range, false);

        animate_rotation.set_percent(percent);
        animate_translation.set_percent(percent);

        Self {
            animate_rotation,
            animate_translation,
            flipped,
        }
    }
    pub fn wobble(&mut self, delta: Duration) -> Transform {
        let z_rot = self.animate_rotation.tick(delta);
        let x = self.animate_translation.tick(delta);
        if self.animate_rotation.just_finished() {
            *self = BuddyWobble::new(!self.flipped, 0.0);
        }

        Transform {
            translation: Vec3::new(x, 0.0, 0.0),
            rotation: Quat::from_rotation_z(z_rot),
            ..Default::default()
        }
    }
}

fn move_buddy(
    time: Res<Time>,
    mut buddies: Query<
        (&mut Transform, &Side, &Slot, &mut BuddyWobble),
        (With<Buddy>, Without<Pad>),
    >,
    pads: Query<(&Transform, &Side, &Slot), (With<Pad>, Without<Buddy>)>,
) {
    for (mut buddy_transform, buddy_side, buddy_slot, mut wobble) in buddies.iter_mut() {
        for (pad_transform, pad_side, pad_slot) in pads.iter() {
            if buddy_side == pad_side && buddy_slot == pad_slot {
                *buddy_transform = *pad_transform;
            }
        }

        *buddy_transform = *buddy_transform * wobble.wobble(time.delta());
    }
}
