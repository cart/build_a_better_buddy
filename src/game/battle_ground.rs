use bevy::prelude::*;

use crate::game::{
    buddy::{Side, Slot},
    Z_PAD,
};

const PAD_SPACING: f32 = 180.0;
const SIDE_SPACING: f32 = 120.0;

#[derive(Bundle, Default)]
pub struct PadBundle {
    pub pad: Pad,
    pub slot: Slot,
    pub side: Side,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component, Default)]
pub struct Pad;

pub fn spawn_pad(commands: &mut Commands, asset_server: &AssetServer, side: Side, slot: Slot) {
    let side_sign = match side {
        Side::Left => -1.,
        Side::Right => 1.,
    };
    let slot_index = slot.0;
    commands
        .spawn_bundle(PadBundle {
            side,
            slot,
            transform: Transform::from_xyz(
                slot_index as f32 * PAD_SPACING * side_sign + SIDE_SPACING * side_sign,
                0.0,
                0.0,
            ),
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
