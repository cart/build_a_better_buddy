use crate::{
    game::{
        pad::{pad_enter_battle, pad_exit_battle, position_pad},
        ui::UiRoot,
    },
    menu::{HOVERED_BUTTON, NORMAL_BUTTON},
    AppState,
};
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Battle)
                .with_system(pad_enter_battle)
                .with_system(enter_battle),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Battle)
                .with_system(position_pad)
                .with_system(shop_button),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Battle)
                .with_system(exit_battle)
                .with_system(pad_exit_battle),
        );
    }
}

pub struct BattleState {
    shop_button: Entity,
}

pub fn enter_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Query<Entity, With<UiRoot>>,
) {
    let ui_root = ui_root.single();
    let shop_button = spawn_shop_button(&mut commands, &asset_server, ui_root);
    commands.insert_resource(BattleState { shop_button })
}

pub fn exit_battle(mut commands: Commands, battle_state: Res<BattleState>) {
    commands
        .entity(battle_state.shop_button)
        .despawn_recursive();
}

#[derive(Component)]
pub struct ShopButton;

fn spawn_shop_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    ui_root: Entity,
) -> Entity {
    let mut shop_button = None;
    commands.entity(ui_root).with_children(|parent| {
        shop_button = Some(
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
                .insert(ShopButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Shop",
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

    shop_button.unwrap()
}

pub fn shop_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ShopButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Shop).unwrap();
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
