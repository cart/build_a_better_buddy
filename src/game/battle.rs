use std::time::Duration;

use crate::{
    game::{
        animate::{AnimateRange, Ease},
        buddy::{
            spawn_buddy, Buddy, BuddyBundle, BuddyColor, BuddyFace, Health, Offset, Side, Slot,
            Strength,
        },
        counters::{Coins, Trophies},
        pad::{pad_enter_battle, pad_exit_battle, position_pad, PAD_SPACING},
        ui::UiRoot,
        BattleMessages,
    },
    AppState,
};
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Battle>()
            .add_system_set(
                SystemSet::on_enter(AppState::Battle)
                    .with_system(pad_enter_battle)
                    .with_system(enter_battle),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Battle)
                    .with_system(battle)
                    .with_system(position_pad),
            )
            .add_system_set(SystemSet::on_exit(AppState::Battle).with_system(pad_exit_battle));
    }
}

pub enum Action {
    Begin {
        timer: Timer,
    },
    StartAttack,
    ExecuteAttack {
        left_buddy: Entity,
        right_buddy: Entity,
        left_strength: usize,
        right_strength: usize,
        left_died: bool,
        right_died: bool,
        animate_in: AnimateRange,
        animate_out: AnimateRange,
    },
    Shift {
        animate_shift: AnimateRange,
        left_buddy: Entity,
        right_buddy: Entity,
        left_died: bool,
        right_died: bool,
    },
    ShowMessage {
        entity: Entity,
        timer: Timer,
    },
    RestoreBuddies {
        animate: AnimateRange,
    },
}

impl Default for Battle {
    fn default() -> Self {
        Self {
            action: Action::Begin {
                timer: Timer::default(),
            },
        }
    }
}

pub struct Battle {
    action: Action,
}

pub fn enter_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut battle: ResMut<Battle>,
    mut trophies: ResMut<Trophies>,
    buddies: Query<(Entity, &Side), With<Buddy>>,
) {
    trophies.rounds += 1;
    // clean up old battle entities
    for (entity, side) in buddies.iter() {
        if *side == Side::Right {
            commands.entity(entity).despawn_recursive();
        }
    }

    for i in 0..Slot::MAX_PER_SIDE {
        let buddy = BuddyBundle {
            color: BuddyColor::random(),
            slot: Slot::new(i),
            face: BuddyFace::random(),
            side: Side::Right,
            ..Default::default()
        };
        spawn_buddy(&mut commands, &asset_server, buddy);
    }

    battle.action = Action::Begin {
        timer: Timer::from_seconds(2.0, false),
    };
}

pub fn battle(
    mut battle: ResMut<Battle>,
    battle_messages: Res<BattleMessages>,
    mut state: ResMut<State<AppState>>,
    mut trophies: ResMut<Trophies>,
    mut coins: ResMut<Coins>,
    time: Res<Time>,
    mut buddies: Query<(
        Entity,
        &mut Buddy,
        &mut Health,
        &mut Strength,
        &mut Transform,
        &mut Offset,
        &Side,
        &mut Slot,
    )>,
    mut visibilities: Query<&mut Visibility>,
) {
    let mut next_action = None;
    match &mut battle.action {
        Action::Begin { timer } => {
            if timer.tick(time.delta()).just_finished() {
                next_action = Some(Action::StartAttack)
            }
        }
        Action::StartAttack => {
            let mut left_buddy = None;
            let mut left_strength = 0;
            let mut right_buddy = None;
            let mut right_strength = 0;
            // NOTE : get_multiple() would be really nice here
            for (entity, _, _, strength, _, _, side, slot) in buddies.iter() {
                if *side == Side::Left && slot.current == 0 {
                    left_buddy = Some(entity);
                    left_strength = strength.0.value();
                } else if *side == Side::Right && slot.current == 0 {
                    right_buddy = Some(entity);
                    right_strength = strength.0.value();
                }
            }

            if let (Some(left_buddy), Some(right_buddy)) = (left_buddy, right_buddy) {
                next_action = Some(Action::ExecuteAttack {
                    left_buddy,
                    right_buddy,
                    left_strength,
                    right_strength,
                    left_died: false,
                    right_died: false,
                    animate_in: AnimateRange::new(
                        Duration::from_secs_f32(0.3),
                        Ease::InOutCirc,
                        0.0..40.0,
                        false,
                    ),
                    animate_out: AnimateRange::new(
                        Duration::from_secs_f32(0.3),
                        Ease::InOutCirc,
                        40.0..0.0,
                        false,
                    ),
                });
            }
        }
        Action::ExecuteAttack {
            left_buddy,
            right_buddy,
            left_strength,
            right_strength,
            left_died,
            right_died,
            animate_in,
            animate_out,
        } => {
            if !animate_in.finished() {
                let x = animate_in.tick(time.delta());
                if let Ok(mut offset) = buddies.get_component_mut::<Offset>(*left_buddy) {
                    offset.0.translation = Vec3::new(x, 0.0, 0.0);
                }
                if let Ok(mut offset) = buddies.get_component_mut::<Offset>(*right_buddy) {
                    offset.0.translation = Vec3::new(-x, 0.0, 0.0);
                }
                if animate_in.just_finished() {
                    if let Ok(mut health) = buddies.get_component_mut::<Health>(*left_buddy) {
                        health.0.remove(*right_strength);
                        *left_died = health.0.value() == 0;
                    }
                    if let Ok(mut health) = buddies.get_component_mut::<Health>(*right_buddy) {
                        health.0.remove(*left_strength);
                        *right_died = health.0.value() == 0;
                    }
                }
            } else {
                let x = animate_out.tick(time.delta());
                if let Ok(mut offset) = buddies.get_component_mut::<Offset>(*left_buddy) {
                    offset.0.translation = Vec3::new(x, 0.0, 0.0);
                }
                if let Ok(mut offset) = buddies.get_component_mut::<Offset>(*right_buddy) {
                    offset.0.translation = Vec3::new(-x, 0.0, 0.0);
                }
                if animate_out.finished() {
                    next_action = Some(Action::Shift {
                        left_buddy: *left_buddy,
                        right_buddy: *right_buddy,
                        left_died: *left_died,
                        right_died: *right_died,
                        animate_shift: AnimateRange::new(
                            Duration::from_secs_f32(1.0),
                            Ease::InOutCirc,
                            0.0..(PAD_SPACING),
                            false,
                        ),
                    })
                }
            }
        }
        Action::Shift {
            left_buddy,
            right_buddy,
            left_died,
            right_died,
            animate_shift,
        } => {
            let x = animate_shift.tick(time.delta());
            let percent = animate_shift.percent();
            for (entity, _, _, _, _, mut offset, side, _) in buddies.iter_mut() {
                if *left_died && *side == Side::Left {
                    if entity == *left_buddy {
                        offset.0.scale = Vec3::new(1.0 - percent, 1.0 - percent, 0.9);
                    } else {
                        offset.0.translation = Vec3::new(x, 0.0, 0.0);
                    }
                }

                if *right_died && *side == Side::Right {
                    if entity == *right_buddy {
                        offset.0.scale = Vec3::new(1.0 - percent, 1.0 - percent, 0.9);
                    } else {
                        offset.0.translation = Vec3::new(-x, 0.0, 0.0);
                    }
                }
            }
            if animate_shift.just_finished() {
                for (entity, mut buddy, _, _, _, mut offset, side, mut slot) in buddies.iter_mut() {
                    if *left_died && *side == Side::Left {
                        if entity == *left_buddy {
                            buddy.alive = false;
                            slot.current = 10;
                        } else if buddy.alive {
                            offset.0.translation = Vec3::new(0.0, 0.0, 0.0);
                            slot.current -= 1;
                        }
                    }

                    if *right_died && *side == Side::Right {
                        if entity == *right_buddy {
                            buddy.alive = false;
                            slot.current = 10;
                        } else if buddy.alive {
                            offset.0.translation = Vec3::new(0.0, 0.0, 0.0);
                            slot.current -= 1;
                        }
                    }
                }

                let mut left_alive = false;
                let mut right_alive = false;
                for (_, buddy, _, _, _, _, side, _) in buddies.iter() {
                    if *side == Side::Left && buddy.alive {
                        left_alive = true;
                    }
                    if *side == Side::Right && buddy.alive {
                        right_alive = true;
                    }
                }

                let action = match (left_alive, right_alive) {
                    (true, true) => Action::StartAttack,
                    (true, false) => {
                        trophies.won += 1;
                        coins.0 += 5;
                        Action::ShowMessage {
                            entity: battle_messages.you_win,
                            timer: Timer::from_seconds(2.0, false),
                        }
                    }
                    (false, true) => Action::ShowMessage {
                        entity: battle_messages.you_lose,
                        timer: Timer::from_seconds(2.0, false),
                    },
                    (false, false) => Action::ShowMessage {
                        entity: battle_messages.you_tie,
                        timer: Timer::from_seconds(2.0, false),
                    },
                };
                next_action = Some(action);
            }
        }
        Action::ShowMessage { entity, timer } => {
            let mut visible = true;
            if timer.tick(time.delta()).just_finished() {
                next_action = Some(Action::RestoreBuddies {
                    animate: AnimateRange::new(
                        Duration::from_secs_f32(1.0),
                        Ease::InOutCirc,
                        0.0..1.0,
                        false,
                    ),
                });
                visible = false;
            }
            if let Ok(mut visibility) = visibilities.get_mut(*entity) {
                visibility.is_visible = visible;
            }
        }
        Action::RestoreBuddies { animate } => {
            let x = animate.tick(time.delta());
            // let percent = animate_shift.percent();

            if animate.just_finished() {
                for (
                    entity,
                    mut buddy,
                    mut health,
                    mut strength,
                    mut transform,
                    mut offset,
                    side,
                    mut slot,
                ) in buddies.iter_mut()
                {
                    if *side == Side::Left {
                        buddy.alive = true;
                        slot.reset();
                        health.0.reset();
                        strength.0.reset();
                        *offset = Offset::default();
                    }
                }
                state.set(AppState::Shop).unwrap();
            }
        }
    }

    if let Some(next_action) = next_action {
        battle.action = next_action;
    }
}
