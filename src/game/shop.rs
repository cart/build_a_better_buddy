use crate::{
    game::{counters::set_coin_text, pad::position_pad, ui::UiRoot},
    menu::{HOVERED_BUTTON, NORMAL_BUTTON},
    AppState,
};
use bevy::prelude::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Shop).with_system(enter_shop))
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(set_coin_text)
                    .with_system(position_pad)
                    .with_system(battle_button),
            )
            .add_system_set(SystemSet::on_exit(AppState::Shop).with_system(exit_shop));
    }
}

pub struct ShopState {
    battle_button: Entity,
}

pub fn enter_shop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Query<Entity, With<UiRoot>>,
) {
    let ui_root = ui_root.single();
    let battle_button = spawn_battle_button(&mut commands, &asset_server, ui_root);
    commands.insert_resource(ShopState { battle_button })
}

pub fn exit_shop(mut commands: Commands, shop_state: Res<ShopState>) {
    commands
        .entity(shop_state.battle_button)
        .despawn_recursive();
}

#[derive(Component)]
pub struct BattleButton;

fn spawn_battle_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    ui_root: Entity,
) -> Entity {
    let mut battle_button = None;
    commands.entity(ui_root).with_children(|parent| {
        battle_button = Some(
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(BattleButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Battle",
                            TextStyle {
                                font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                                font_size: 60.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .id(),
        );
    });

    battle_button.unwrap()
}

pub fn battle_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<BattleButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Battle).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
