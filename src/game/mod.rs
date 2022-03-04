pub mod animate;
pub mod battle;
pub mod buddy;
pub mod counters;
pub mod pad;
pub mod shop;
pub mod ui;

use crate::{
    game::{
        animate::AnimatePlugin, battle::BattlePlugin, buddy::BuddyPlugin, counters::Coins,
        pad::spawn_pads, shop::ShopPlugin,
    },
    AppState,
};
use bevy::prelude::*;
use ui::spawn_ui;

const Z_FOREGROUND: f32 = 10.0;
const Z_PAD: f32 = 11.0;
const Z_BUDDY: f32 = 20.0;
const Z_MESSAGE: f32 = 300.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Coins(20))
            .add_plugin(BuddyPlugin)
            .add_plugin(AnimatePlugin)
            .add_plugin(ShopPlugin)
            .add_plugin(BattlePlugin)
            .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(setup_game));
    }
}

pub struct BattleMessages {
    you_win: Entity,
    you_lose: Entity,
    you_tie: Entity,
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    spawn_ui(&mut commands, &asset_server);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("foreground.png"),
        transform: Transform::from_xyz(0.0, 100.0, Z_FOREGROUND),
        ..Default::default()
    });

    spawn_pads(&mut commands, &asset_server);

    let you_win = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("you_win.png"),
            visibility: Visibility { is_visible: false },
            transform: Transform::from_xyz(0.0, 100.0, Z_MESSAGE),
            ..Default::default()
        })
        .id();

    let you_lose = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("you_lose.png"),
            visibility: Visibility { is_visible: false },
            transform: Transform::from_xyz(0.0, 100.0, Z_MESSAGE),
            ..Default::default()
        })
        .id();

    let you_tie = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("you_tie.png"),
            visibility: Visibility { is_visible: false },
            transform: Transform::from_xyz(0.0, 200.0, Z_MESSAGE),
            ..Default::default()
        })
        .id();
    commands.insert_resource(BattleMessages {
        you_win,
        you_lose,
        you_tie,
    });
    state.set(AppState::Shop).unwrap();
}
