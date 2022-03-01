use bevy::prelude::*;

#[derive(Default)]
pub struct Coins(pub usize);

pub fn set_coin_text(coins: Res<Coins>, mut coin_texts: Query<&mut Text, With<CoinText>>) {
    for mut text in coin_texts.iter_mut() {
        text.sections[0].value = format!("{}", coins.0);
    }
}

#[derive(Component)]
pub struct CoinText;

pub fn spawn_coins_element(commands: &mut ChildBuilder, asset_server: &AssetServer) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(74.0)),
                margin: Rect {
                    top: Val::Px(4.0),
                    left: Val::Px(4.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage(asset_server.load("money.png")),
                ..Default::default()
            });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(32.0)),
                        margin: Rect {
                            bottom: Val::Px(-13.0),
                            ..Default::default()
                        },

                        ..Default::default()
                    },
                    text: Text::with_section(
                        "0",
                        TextStyle {
                            font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                            font_size: 100.0,
                            color: Color::hex("323232").unwrap(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Bottom,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    ..Default::default()
                })
                .insert(CoinText);
        });
}

#[derive(Component)]
pub struct TrophyText;

pub fn spawn_trophies_element(commands: &mut ChildBuilder, asset_server: &AssetServer) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(74.0)),
                margin: Rect {
                    top: Val::Px(4.0),
                    left: Val::Px(4.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage(asset_server.load("trophy.png")),
                ..Default::default()
            });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(32.0)),
                        margin: Rect {
                            bottom: Val::Px(-13.0),
                            ..Default::default()
                        },

                        ..Default::default()
                    },
                    text: Text::with_section(
                        "0",
                        TextStyle {
                            font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                            font_size: 100.0,
                            color: Color::hex("323232").unwrap(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Bottom,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    ..Default::default()
                })
                .insert(TrophyText);
        });
}
