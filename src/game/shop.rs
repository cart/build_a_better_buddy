use crate::{
    game::{
        buddy::{
            spawn_buddy, Attribute, Buddy, BuddyBundle, BuddyColor, BuddyFace, Health, Side, Slot,
        },
        counters::{set_coin_text, set_trophies_text, Coins, Trophies},
        pad::{position_pad, spawn_pad},
        ui::UiRoot,
        Z_BUDDY,
    },
    AppState,
};
use bevy::{
    math::{const_vec2, Vec3Swizzles},
    prelude::*,
    render::camera::CameraPlugin,
    text::Text2dSize,
    ui::FocusPolicy,
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Coins(6))
            .insert_resource(Trophies { won: 0, rounds: 0 })
            .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(spawn_shop_base))
            .add_system_set(SystemSet::on_enter(AppState::Shop).with_system(enter_shop))
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(set_coin_text)
                    .with_system(set_trophies_text)
                    .with_system(position_pad)
                    .with_system(buy_buddy)
                    .with_system(update_price_counter)
                    .with_system(battle_button),
            )
            .add_system_set(SystemSet::on_exit(AppState::Shop).with_system(exit_shop));
    }
}

const SHOP_BUDDY_SLOTS: usize = 3;

#[derive(Component)]
pub struct ShopPad;

pub fn spawn_shop_base(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..SHOP_BUDDY_SLOTS {
        spawn_pad(&mut commands, &asset_server, Side::Shop, Slot::new(i));
    }
}

pub struct ShopState {
    battle_button: Entity,
}

pub fn enter_shop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Query<Entity, With<UiRoot>>,
    buddies: Query<(Entity, &Side), With<Buddy>>,
) {
    let ui_root = ui_root.single();
    let battle_button = spawn_battle_button(&mut commands, &asset_server, ui_root);
    commands.insert_resource(ShopState { battle_button });

    // clean up old shop entities
    for (entity, side) in buddies.iter() {
        if *side == Side::Shop {
            commands.entity(entity).despawn_recursive();
        }
    }

    for i in 0..SHOP_BUDDY_SLOTS {
        let buddy = BuddyBundle {
            color: BuddyColor::random(),
            slot: Slot::new(i),
            face: BuddyFace::random(),
            side: Side::Shop,
            health: Health(Attribute::new(2)),
            transform: Transform::from_xyz(0.0, -500.0, 0.0),
            ..Default::default()
        };
        let buddy_id = spawn_buddy(&mut commands, &asset_server, buddy);
        add_price(&mut commands, &asset_server, buddy_id, 2);
    }
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
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .insert(FocusPolicy::Pass)
            .with_children(|parent| {
                battle_button = Some(
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(236.0), Val::Px(186.0)),
                                margin: Rect {
                                    bottom: Val::Auto,
                                    top: Val::Auto,
                                    right: Val::Px(100.0),
                                    ..Default::default()
                                },
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                align_self: AlignSelf::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .insert(BattleButton)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    image: asset_server.load("battle_button.png").into(),
                                    ..Default::default()
                                })
                                .insert(FocusPolicy::Pass);
                        })
                        .id(),
                )
            });
    });

    battle_button.unwrap()
}

pub fn battle_button(
    mut state: ResMut<State<AppState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BattleButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            state.set(AppState::Battle).unwrap();
        }
    }
}

const BUDDY_EXTENTS: Vec2 = const_vec2!([65.0, 65.0]);

fn buy_buddy(
    mut commands: Commands,
    mut coins: ResMut<Coins>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut buddies: Query<(Entity, &Transform, &mut Slot, &mut Side, Option<&Price>), With<Buddy>>,
    children: Query<&Children>,
    price_counters: Query<&PriceCounter>,
    price_icons: Query<&PriceIcon>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, global_transform) = cameras
        .iter()
        .find(|(camera, _)| camera.name.as_deref() == Some(CameraPlugin::CAMERA_2D))
        .unwrap();
    let cursor_screen = if let Some(cursor) = window.cursor_position() {
        cursor
    } else {
        return;
    };

    let cursor_world = screen_to_world(
        Vec2::new(window.width(), window.height()),
        cursor_screen,
        camera,
        global_transform,
    );
    if mouse_button.just_pressed(MouseButton::Left) {
        let occupied_slots = buddies
            .iter()
            .filter_map(|(_, _, slot, side, _)| {
                if *side == Side::Left {
                    Some(slot.current)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (entity, transform, mut slot, mut side, price) in buddies.iter_mut() {
            if *side != Side::Shop {
                continue;
            }
            let pos = transform.translation;
            let min = pos.xy() - BUDDY_EXTENTS;
            let max = pos.xy() + BUDDY_EXTENTS;
            if cursor_world.x < max.x
                && cursor_world.x > min.x
                && cursor_world.y < max.y
                && cursor_world.y > min.y
            {
                let open_slot = (0..3).find(|i| !occupied_slots.contains(i));
                if let Some(open_slot) = open_slot {
                    *side = Side::Left;
                    *slot = Slot::new(open_slot);
                    coins.0 -= price.unwrap().0;
                    remove_price(
                        &mut commands,
                        entity,
                        &children,
                        &price_counters,
                        &price_icons,
                    )
                }
                break;
            }
        }
    }
}

fn screen_to_world(
    window_size: Vec2,
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

#[derive(Component)]
pub struct Price(usize);

#[derive(Component)]
pub struct PriceCounter;

#[derive(Component)]
pub struct PriceIcon;

pub fn add_price(
    commands: &mut Commands,
    asset_server: &AssetServer,
    entity: Entity,
    price: usize,
) {
    commands
        .entity(entity)
        .insert(Price(price))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(-90.0, 70.0, Z_BUDDY + 0.3)
                        .with_scale(Vec3::splat(0.5)),
                    texture: asset_server.load("price.png"),
                    ..Default::default()
                })
                .insert(PriceIcon);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "0",
                        TextStyle {
                            font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                            font_size: 70.0,
                            color: Color::hex("323232").unwrap(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Bottom,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    text_2d_size: Text2dSize {
                        size: Size::new(100., 100.),
                    },
                    transform: Transform::from_xyz(-70.0, 38.0, Z_BUDDY + 0.1),
                    ..Default::default()
                })
                .insert(PriceCounter);
        });
}

fn remove_price(
    commands: &mut Commands,
    entity: Entity,
    children: &Query<&Children>,
    price_counters: &Query<&PriceCounter>,
    price_icons: &Query<&PriceIcon>,
) {
    // commands.entity(entity).remove::<Price>();
    if let Ok(children) = children.get(entity) {
        for child in children.iter().copied() {
            if price_icons.get(child).is_ok() {
                commands.entity(child).despawn();
            }
            if price_counters.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}

fn update_price_counter(
    mut counters: Query<(&mut Text, &Parent), With<PriceCounter>>,
    prices: Query<&Price>,
) {
    for (mut text, parent) in counters.iter_mut() {
        if let Ok(price) = prices.get(parent.0) {
            text.sections[0].value = price.0.to_string();
        }
    }
}
