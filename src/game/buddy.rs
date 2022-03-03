use crate::game::{
    animate::{AnimateRange, AnimateScale, Ease},
    Z_BUDDY,
};
use bevy::{prelude::*, text::Text2dSize};
use rand::Rng;
use std::{f32::consts::PI, time::Duration};

pub struct BuddyPlugin;

impl Plugin for BuddyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OutlineTimer>()
            .add_system(update_outlines)
            .add_system(set_buddy_face)
            .add_system(set_health_counter)
            .add_system(set_strength_counter)
            .add_system(move_buddy)
            .add_system(wobble_buddy);
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Shop,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct Slot(pub usize);

impl Slot {
    pub const MAX_PER_SIDE: usize = 3;
    pub fn new(slot: usize) -> Self {
        if slot >= Self::MAX_PER_SIDE {
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

    pub fn random() -> BuddyFace {
        let index = rand::thread_rng().gen_range(0..2);
        match index {
            0 => BuddyFace::Happy,
            1 => BuddyFace::Neutral,
            _ => panic!("this shouldn't happen"),
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

#[derive(Component, Default, Copy, Clone)]
pub struct BuddyColor(Color);

impl BuddyColor {
    const RED: BuddyColor = Self(Color::rgb(0.67, 0.53, 0.53));
    const GREEN: BuddyColor = Self(Color::rgb(0.53, 0.67, 0.53));
    const BLUE: BuddyColor = Self(Color::rgb(0.53, 0.53, 0.67));
    const COLORS: &'static [BuddyColor] = &[Self::RED, Self::GREEN, Self::BLUE];
    pub fn random() -> BuddyColor {
        Self::COLORS[rand::thread_rng().gen_range(0..Self::COLORS.len())]
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
    pub health: Health,
    pub strength: Strength,
    pub face: BuddyFace,
    pub blink: BuddyBlink,
    pub slot: Slot,
    pub color: BuddyColor,
    pub side: Side,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn spawn_buddy(commands: &mut Commands, asset_server: &AssetServer, buddy: BuddyBundle) {
    commands.spawn_bundle(buddy).with_children(|parent| {
        parent
            .spawn_bundle(SpriteBundle::default())
            .insert(BuddyWobble::default())
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
                        transform: Transform::from_xyz(0.0, 0.0, Z_BUDDY)
                            .with_scale(Vec3::splat(0.5)),
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
        parent
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(-40.0, -70.0, Z_BUDDY + 0.3)
                    .with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("buddy/health.png"),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            "0",
                            TextStyle {
                                font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                                font_size: 110.0,
                                color: Color::hex("ececec").unwrap(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Bottom,
                                horizontal: HorizontalAlign::Left,
                            },
                        ),
                        text_2d_size: Text2dSize {
                            size: Size::new(100., 100.),
                        },
                        transform: Transform::from_xyz(-20.0, -55.0, 0.1),
                        ..Default::default()
                    })
                    .insert(HealthCounter);
            });
        parent
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(40.0, -70.0, Z_BUDDY + 0.3)
                    .with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("buddy/strength.png"),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            "0",
                            TextStyle {
                                font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                                font_size: 110.0,
                                color: Color::hex("ececec").unwrap(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Bottom,
                                horizontal: HorizontalAlign::Left,
                            },
                        ),
                        text_2d_size: Text2dSize {
                            size: Size::new(100., 100.),
                        },
                        transform: Transform::from_xyz(-10.0, -55.0, 0.1),
                        ..Default::default()
                    })
                    .insert(StrengthCounter);
            });
    });
}

#[derive(Component)]
pub struct HealthCounter;

#[derive(Component)]
pub struct Health(Attribute);

impl Default for Health {
    fn default() -> Self {
        Self(Attribute::new(1))
    }
}

#[derive(Component)]
pub struct Strength(Attribute);

#[derive(Component)]
pub struct StrengthCounter;

impl Default for Strength {
    fn default() -> Self {
        Self(Attribute::new(1))
    }
}

pub struct Attribute {
    base: usize,
    value: isize,
}

impl Attribute {
    pub fn new(base: usize) -> Self {
        Self {
            base,
            value: base as isize,
        }
    }

    pub fn set_base(&mut self, base: usize) {
        self.base = base;
    }

    pub fn reset(&mut self) {
        self.value = self.base as isize;
    }

    pub fn value(&self) -> usize {
        self.value.max(0) as usize
    }

    pub fn remove(&mut self, amount: usize) {
        self.value -= amount as isize;
    }

    pub fn add(&mut self, amount: usize) {
        self.value += amount as isize;
    }
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
    parents: Query<&Parent>,
    mut buddies: Query<(&Side, &BuddyFace, &BuddyColor, &mut BuddyBlink), With<Buddy>>,
    mut faces: Query<
        (&mut Handle<Image>, &mut Sprite, &Parent),
        (With<BuddyFaceSprite>, Without<BuddyBodySprite>),
    >,
    mut bodies: Query<(&mut Sprite, &Parent), With<BuddyBodySprite>>,
) {
    for (mut image, mut sprite, parent) in faces.iter_mut() {
        let buddy_entity = parents.get(parent.0).unwrap().0;
        if let Ok((side, face, _, mut blink)) = buddies.get_mut(buddy_entity) {
            match side {
                Side::Left => {
                    sprite.flip_x = false;
                }
                Side::Right => {
                    sprite.flip_x = true;
                }
                Side::Shop => {
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
        let buddy_entity = parents.get(parent.0).unwrap().0;
        if let Ok((_, _, color, _)) = buddies.get(buddy_entity) {
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
    mut buddies: Query<(&mut Transform, &Side, &Slot), With<Buddy>>,
    pads: Query<(&Transform, &Side, &Slot), Without<Buddy>>,
) {
    for (mut buddy_transform, buddy_side, buddy_slot) in buddies.iter_mut() {
        for (pad_transform, pad_side, pad_slot) in pads.iter() {
            if buddy_side == pad_side && buddy_slot == pad_slot {
                *buddy_transform = *pad_transform;
            }
        }
    }
}

fn wobble_buddy(time: Res<Time>, mut buddies: Query<(&mut Transform, &mut BuddyWobble)>) {
    for (mut transform, mut wobble) in buddies.iter_mut() {
        *transform = wobble.wobble(time.delta());
    }
}

fn set_health_counter(
    parents: Query<&Parent>,
    mut counters: Query<(&mut Text, &Parent), With<HealthCounter>>,
    buddies: Query<&Health>,
) {
    for (mut text, parent) in counters.iter_mut() {
        let buddy_entity = parents.get(parent.0).unwrap().0;
        if let Ok(health) = buddies.get(buddy_entity) {
            text.sections[0].value = health.0.value().to_string();
        }
    }
}

fn set_strength_counter(
    parents: Query<&Parent>,
    mut counters: Query<(&mut Text, &Parent), With<StrengthCounter>>,
    buddies: Query<&Strength>,
) {
    for (mut text, parent) in counters.iter_mut() {
        let buddy_entity = parents.get(parent.0).unwrap().0;
        if let Ok(strength) = buddies.get(buddy_entity) {
            text.sections[0].value = strength.0.value().to_string();
        }
    }
}
