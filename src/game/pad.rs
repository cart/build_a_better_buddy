use crate::{
    game::{
        animate::{AnimateRange, Ease},
        buddy::{Side, Slot},
        Z_PAD,
    },
    AppState,
};
use bevy::prelude::*;
use std::time::Duration;

const PAD_SPACING: f32 = 180.0;
const SIDE_SPACING: f32 = 120.0;
const RIGHT_PAD_OUT: f32 = 1500.0;
const PAD_CENTER_OFFSET: f32 = ((Slot::MAX_PER_SIDE - 1) as f32 * PAD_SPACING) / 2.0;

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
    right_animate_in: AnimateRange,
    right_animate_out: AnimateRange,
    left_animate_center: AnimateRange,
    left_animate_side: AnimateRange,
}

impl Default for Pad {
    fn default() -> Self {
        let mut value = Self {
            right_animate_out: AnimateRange::new(
                Duration::from_secs_f32(1.5),
                Ease::InOutCirc,
                SIDE_SPACING..RIGHT_PAD_OUT,
                false,
            ),
            right_animate_in: AnimateRange::new(
                Duration::from_secs_f32(2.0),
                Ease::InOutCirc,
                RIGHT_PAD_OUT..SIDE_SPACING,
                false,
            ),
            left_animate_center: AnimateRange::new(
                Duration::from_secs_f32(1.5),
                Ease::InOutCirc,
                -SIDE_SPACING..PAD_CENTER_OFFSET,
                false,
            ),
            left_animate_side: AnimateRange::new(
                Duration::from_secs_f32(2.0),
                Ease::InOutCirc,
                PAD_CENTER_OFFSET..-SIDE_SPACING,
                false,
            ),
        };
        value.right_animate_out.set_percent(1.0);
        value.left_animate_center.set_percent(1.0);
        value
    }
}

pub fn spawn_pads(commands: &mut Commands, asset_server: &AssetServer) {
    for i in 0..Slot::MAX_PER_SIDE {
        spawn_pad(commands, asset_server, Side::Left, Slot::new(i));
        spawn_pad(commands, asset_server, Side::Right, Slot::new(i));
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

pub fn position_pad(
    time: Res<Time>,
    state: Res<State<AppState>>,
    mut pads: Query<(&mut Pad, &mut Transform, &Side, &Slot)>,
) {
    for (mut pad, mut transform, side, slot) in pads.iter_mut() {
        let side_sign;
        let offset = match side {
            Side::Left => {
                side_sign = -1.0;
                if *state.current() == AppState::Battle {
                    pad.left_animate_side.tick(time.delta())
                } else {
                    pad.left_animate_center.tick(time.delta())
                }
            }
            Side::Right => {
                side_sign = 1.0;
                if *state.current() == AppState::Battle {
                    pad.right_animate_in.tick(time.delta())
                } else {
                    pad.right_animate_out.tick(time.delta())
                }
            }
        };

        *transform =
            Transform::from_xyz(slot.0 as f32 * PAD_SPACING * side_sign + offset, 0.0, 0.0);
    }
}

pub fn pad_exit_battle(mut pads: Query<(&mut Pad, &Side)>) {
    for (mut pad, side) in pads.iter_mut() {
        if *side == Side::Right {
            pad.right_animate_out.reset();
        }

        if *side == Side::Left {
            pad.left_animate_center.reset();
        }
    }
}

pub fn pad_enter_battle(mut pads: Query<(&mut Pad, &Side)>) {
    for (mut pad, side) in pads.iter_mut() {
        if *side == Side::Right {
            pad.right_animate_in.reset();
        }

        if *side == Side::Left {
            pad.left_animate_side.reset();
        }
    }
}
