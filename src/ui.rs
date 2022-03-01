use bevy::prelude::*;

pub fn spawn_money_element(commands: &mut ChildBuilder, asset_server: &AssetServer) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(32.0)),
                ..Default::default()
            },
            color: Color::RED.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage(asset_server.load("money.png")),
                ..Default::default()
            });
        });
}

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
        .with_children(|parent| {
            spawn_money_element(parent, asset_server);
        });

    // spawn overlay
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Bevy Jam #1",
                    TextStyle {
                        font: asset_server.load("font/AmaticSC-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::hex("323232").unwrap(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Project Build-A-Better-Buddy",
                    TextStyle {
                        font: asset_server.load("font/AmaticSC-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::hex("323232").unwrap(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}
