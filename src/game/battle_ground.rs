use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    animate::{AnimateRange, Ease},
    buddy::{Side, Slot},
    Z_PAD,
};

const PAD_SPACING: f32 = 180.0;
const SIDE_SPACING: f32 = 120.0;
const PAD_OUT: f32 = 1500.0;

#[derive(Bundle, Default)]
pub struct PadBundle {
    pub pad: Pad,
    pub slot: Slot,
    pub side: Side,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct Pad {
    animate_in: AnimateRange,
    animate_out: AnimateRange,
    active: bool,
}

impl Default for Pad {
    fn default() -> Self {
        let mut value = Self {
            animate_out: AnimateRange::new(
                Duration::from_secs_f32(1.5),
                Ease::InOutCirc,
                0.0..PAD_OUT,
                false,
            ),
            animate_in: AnimateRange::new(
                Duration::from_secs_f32(2.0),
                Ease::InOutCirc,
                PAD_OUT..0.0,
                false,
            ),
            active: true,
        };
        value.animate_in.set_percent(100.0);
        value.animate_out.set_percent(100.0);
        value
    }
}

pub fn spawn_pad(commands: &mut Commands, asset_server: &AssetServer, side: Side, slot: Slot) {
    commands
        .spawn_bundle(PadBundle {
            side,
            slot,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("pad.png"),
                transform: Transform::from_xyz(0., -60., Z_PAD),
                ..Default::default()
            });
        });
}

pub fn position_pad(time: Res<Time>, mut pads: Query<(&mut Pad, &mut Transform, &Side, &Slot)>) {
    for (mut pad, mut transform, side, slot) in pads.iter_mut() {
        let side_sign = match side {
            Side::Left => -1.,
            Side::Right => 1.,
        };
        let offset = if pad.active {
            pad.animate_in.tick(time.delta())
        } else {
            pad.animate_out.tick(time.delta())
        };
        *transform = Transform::from_xyz(
            slot.0 as f32 * PAD_SPACING * side_sign + SIDE_SPACING * side_sign + offset,
            0.0,
            0.0,
        );
    }
}

pub fn exit_battle_ground(mut pads: Query<(&mut Pad, &Side)>) {
    for (mut pad, side) in pads.iter_mut() {
        if *side == Side::Right {
            pad.active = false;
            pad.animate_out.set_percent(0.0);
        }
    }
}

pub fn enter_battle_ground(mut pads: Query<(&mut Pad, &Side)>) {
    for (mut pad, side) in pads.iter_mut() {
        if *side == Side::Right {
            pad.active = true;
            pad.animate_in.set_percent(0.0);
        }
    }
}
