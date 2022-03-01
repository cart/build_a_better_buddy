use bevy::prelude::*;

use crate::{
    buddy::{spawn_buddy, BuddyBundle, BuddyColor, BuddyFace, BuddySlot, Side},
    ui::spawn_ui,
    AppState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_game));
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_ui(&mut commands, &asset_server);
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Happy,
            side: Side::Left,
            slot: BuddySlot::new(0),
            color: BuddyColor::blue(),
            transform: Transform::from_xyz(-250.0, 0.0, 0.0),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Neutral,
            side: Side::Left,
            slot: BuddySlot::new(0),
            color: BuddyColor::blue(),
            transform: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..Default::default()
        },
    );

    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Happy,
            side: Side::Right,
            slot: BuddySlot::new(0),
            color: BuddyColor::red(),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        },
    );
    spawn_buddy(
        &mut commands,
        &asset_server,
        BuddyBundle {
            face: BuddyFace::Neutral,
            side: Side::Right,
            slot: BuddySlot::new(0),
            color: BuddyColor::red(),
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            ..Default::default()
        },
    );
}
