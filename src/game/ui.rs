use crate::game::counters::{spawn_coins_element, spawn_trophies_element};
use bevy::prelude::*;

#[derive(Component)]
pub struct UiRoot;

pub fn spawn_ui(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(UiRoot)
        .with_children(|parent| {
            spawn_coins_element(parent, asset_server);
            spawn_trophies_element(parent, asset_server);
        });
}
