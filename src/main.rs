mod buddy;
mod game;
mod menu;
mod ui;

use crate::{buddy::BuddyPlugin, game::GamePlugin, menu::MenuPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Build A Better Buddy".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .add_state(AppState::Game)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(BuddyPlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Game,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
