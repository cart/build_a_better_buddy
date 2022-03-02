pub mod animate;
pub mod battle_ground;
pub mod buddy;
pub mod counters;
pub mod ui;

use crate::{
    game::{
        animate::AnimatePlugin,
        battle_ground::{enter_battle_ground, position_pad, spawn_pad},
        buddy::BuddyPlugin,
        counters::{set_coin_text, Coins},
    },
    AppState,
};
use bevy::prelude::*;
use buddy::{spawn_buddy, BuddyBundle, BuddyColor, BuddyFace, Side, Slot};
use ui::spawn_ui;

const Z_FOREGROUND: f32 = 10.0;
const Z_PAD: f32 = 11.0;
const Z_BUDDY: f32 = 20.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BuddyPlugin)
            .add_plugin(AnimatePlugin)
            .insert_resource(Coins(20))
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_game.exclusive_system().at_start())
                    .with_system(enter_battle_ground),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(set_coin_text)
                    .with_system(position_pad),
            );
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_ui(&mut commands, &asset_server);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("foreground.png"),
        transform: Transform::from_xyz(0.0, 0.0, Z_FOREGROUND),
        ..Default::default()
    });

    spawn_pad(&mut commands, &asset_server, Side::Left, Slot::new(0));
    spawn_pad(&mut commands, &asset_server, Side::Left, Slot::new(1));
    spawn_pad(&mut commands, &asset_server, Side::Left, Slot::new(2));

    spawn_pad(&mut commands, &asset_server, Side::Right, Slot::new(0));
    spawn_pad(&mut commands, &asset_server, Side::Right, Slot::new(1));
    spawn_pad(&mut commands, &asset_server, Side::Right, Slot::new(2));

    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Happy,
            side: Side::Left,
            slot: Slot::new(0),
            color: BuddyColor::blue(),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Neutral,
            side: Side::Left,
            slot: Slot::new(1),
            color: BuddyColor::blue(),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Neutral,
            side: Side::Left,
            slot: Slot::new(2),
            color: BuddyColor::blue(),
            ..Default::default()
        },
    );

    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Neutral,
            side: Side::Right,
            slot: Slot::new(0),
            color: BuddyColor::red(),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Happy,
            side: Side::Right,
            slot: Slot::new(1),
            color: BuddyColor::red(),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Happy,
            side: Side::Right,
            slot: Slot::new(2),
            color: BuddyColor::red(),
            ..Default::default()
        },
    );
}
