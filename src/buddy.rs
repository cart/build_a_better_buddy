use bevy::prelude::*;

use rand::Rng;
use std::f32::consts::PI;

pub struct BuddyPlugin;

impl Plugin for BuddyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutlineTimer>()
            .add_system(update_outlines)
            .add_system(set_buddy_face);
    }
}

#[derive(Component)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component)]
pub struct BuddySlot(usize);

const MAX_BUDDIES_PER_SIDE: usize = 3;

impl BuddySlot {
    pub fn new(slot: usize) -> Self {
        if slot >= MAX_BUDDIES_PER_SIDE {
            panic!("invalid buddy slot {slot}");
        }

        Self(slot)
    }
}

impl Default for BuddySlot {
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

#[derive(Component, Default)]
pub struct BuddyColor(Color);

impl BuddyColor {
    pub fn red() -> Self {
        Self(Color::rgb(1.0, 0.8, 0.8))
    }

    pub fn blue() -> Self {
        Self(Color::rgb(0.8, 0.8, 1.0))
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
    pub slot: BuddySlot,
    pub color: BuddyColor,
    pub side: Side,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn spawn_buddy(commands: &mut Commands, asset_server: &AssetServer, mut buddy: BuddyBundle) {
    buddy.transform.scale = Vec3::splat(2.0);
    commands.spawn_bundle(buddy).with_children(|parent| {
        parent
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("buddy/base.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(BuddyBodySprite);
        parent
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("buddy/outline.png"),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(BuddyOutline);
        parent
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
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
    buddies: Query<(&Side, &BuddyFace, &BuddyColor), With<Buddy>>,
    mut faces: Query<
        (&mut Handle<Image>, &mut Sprite, &Parent),
        (With<BuddyFaceSprite>, Without<BuddyBodySprite>),
    >,
    mut bodies: Query<(&mut Sprite, &Parent), With<BuddyBodySprite>>,
) {
    for (mut image, mut sprite, parent) in faces.iter_mut() {
        if let Ok((side, face, _)) = buddies.get(parent.0) {
            match side {
                Side::Left => {
                    sprite.flip_x = false;
                }
                Side::Right => {
                    sprite.flip_x = true;
                }
            }

            *image = asset_server.load(face.get_path());
        }
    }

    for (mut sprite, parent) in bodies.iter_mut() {
        if let Ok((_, _, color)) = buddies.get(parent.0) {
            sprite.color = color.0;
        }
    }
}
