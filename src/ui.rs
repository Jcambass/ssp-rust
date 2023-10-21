use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    shooting::WeaponSwitchedEvent, AppState, Game, Layers, LevelUpEvent, MyAssets, Player,
    EARTH_HEALTH, PLAYER_HEALTH,
};

pub struct UiOverlayPlugin;

impl Plugin for UiOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnExit(AppState::Loading)))
            .add_system(clear_msg.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_stats.in_set(OnUpdate(AppState::InGame)))
            .add_system(level_up_msg.in_set(OnUpdate(AppState::InGame)))
            .add_system(weapon_switched_msg.in_set(OnUpdate(AppState::InGame)))
            .add_system(gameover_screen.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(gamewon_screen.in_schedule(OnEnter(AppState::GameWon)))
            .add_system(pause_screen.in_schedule(OnEnter(AppState::Paused)))
            .add_system(clear_msg_now.in_schedule(OnExit(AppState::Paused)));
    }
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct EarthHealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
pub struct MessageText;

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.single();

    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 39.0,
        color: Color::WHITE,
    };

    let box_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 25.0,
        color: Color::WHITE,
    };

    let health_img_handle = my_assets.health_box.clone();
    let health_img_size = assets.get(&health_img_handle).unwrap().size();

    let score_img_handle = my_assets.score_box.clone();
    let score_img_size = assets.get(&score_img_handle).unwrap().size();

    let scale = 1.0;
    let padding = 12.0;
    let health_box_center_x = -(window.width() / 2.0) + (health_img_size.x * scale / 2.0) + padding;
    let health_box_center_y = (window.height() / 2.0) - (health_img_size.y * scale / 2.0) - padding;

    let score_box_start_x = (window.width() / 2.0) - (score_img_size.x * scale / 2.0) - padding;
    let score_box_start_y = (window.height() / 2.0) - (score_img_size.y * scale / 2.0) - padding;

    commands.spawn(SpriteBundle {
        // TODO: Do not clone here.
        texture: health_img_handle,
        transform: Transform {
            translation: Vec3 {
                x: health_box_center_x,
                y: health_box_center_y,
                z: Layers::UI.order_nr(),
            },
            scale: Vec3::splat(scale),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        // TODO: Do not clone here.
        texture: score_img_handle,
        transform: Transform {
            translation: Vec3 {
                x: score_box_start_x,
                y: score_box_start_y,
                z: Layers::UI.order_nr(),
            },
            scale: Vec3::splat(scale),
            ..default()
        },
        ..default()
    });

    commands.spawn(
        (TextBundle {
            text: Text::from_section(PLAYER_HEALTH.to_string(), box_style.clone()),
            ..default()
        }.with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(61.0),
                left: Val::Px(100.0),
                ..default()
            },
            ..default()
        }), HealthText));


        // Health bar

        let health_bar_start_x = health_box_center_x - health_img_size.x * scale / 2.0;
        let health_bar_start_y = health_box_center_y - health_img_size.y * scale / 2.0;

        let orange_left_img_handle = my_assets.orange_left.clone();
        let orange_left_size = assets.get(&orange_left_img_handle).unwrap().size();
        commands.spawn((SpriteBundle {
            // TODO: Do not clone here.
            texture: orange_left_img_handle,
            transform: Transform {
                translation: Vec3 {
                    x: health_bar_start_x,
                    y: health_bar_start_y,
                    z: Layers::UI.order_nr(),
                },
                scale: Vec3::splat(scale),
                ..default()
            },
            ..default()
        }, HealthBar));


        for i in 1..PLAYER_HEALTH {
            let orange_middle_img_handle = my_assets.orange_middle.clone();
            let orange_middle_size = assets.get(&orange_middle_img_handle).unwrap().size();

            commands.spawn((SpriteBundle {
                // TODO: Do not clone here.
                texture: orange_middle_img_handle,
                transform: Transform {
                    translation: Vec3 {
                        x: health_bar_start_x + i as f32,
                        y: health_bar_start_y,
                        z: Layers::UI.order_nr(),
                    },
                    scale: Vec3 {
                        x: scale * 0.5,
                        y: scale,
                        z: scale,
                    },
                    ..default()
                },
                ..default()
            }, HealthBar));
        };

        let orange_right_img_handle = my_assets.orange_right.clone();
        let orange_right_size = assets.get(&orange_right_img_handle).unwrap().size();
        commands.spawn((SpriteBundle {
            // TODO: Do not clone here.
            texture: orange_right_img_handle,
            transform: Transform {
                translation: Vec3 {
                    x: health_bar_start_x + PLAYER_HEALTH as f32,
                    y: health_bar_start_y,
                    z: Layers::UI.order_nr(),
                },
                scale: Vec3::splat(scale),
                ..default()
            },
            ..default()
        }, HealthBar));

        // Rest

        commands.spawn(
            (TextBundle {
                text: Text::from_section(EARTH_HEALTH.to_string(), box_style.clone()),
                ..default()
            }.with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(102.0),
                    left: Val::Px(100.0),
                    ..default()
                },
                ..default()
            }), EarthHealthText));

            commands.spawn(
                (TextBundle {
                    text: Text::from_section(0.to_string(), box_style.clone()),
                    ..default()
                }.with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(61.0),
                        left: Val::Px(window.width() - 100.0),
                        ..default()
                    },
                    ..default()
                }), ScoreText));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                margin: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {

            // TODO: Move this text a bit higher.
            // Original has 270 as y which might be somthing like +32.5 for us.
            parent.spawn((
                TextBundle::from_sections([TextSection::new(
                    "PROTECT EARTH AS LONG AS YOU CAN!!!",
                    text_style.clone(),
                )])
                .with_text_alignment(TextAlignment::Center),
                MessageText,
            ));
        });

    commands.insert_resource(MessageConfig {
        msg_timer: Timer::new(Duration::from_secs_f32(4.25), TimerMode::Once),
    });
}

#[derive(Resource)]
struct MessageConfig {
    msg_timer: Timer,
}

fn clear_msg(
    mut query: Query<&mut Text, With<MessageText>>,
    time: Res<Time>,
    mut config: ResMut<MessageConfig>,
) {
    config.msg_timer.tick(time.delta());
    if config.msg_timer.just_finished() {
        let mut text = query.single_mut();
        text.sections = vec![];
    }
}

fn update_stats(
    mut set: ParamSet<(
        Query<&mut Text, With<HealthText>>,
        Query<&mut Text, With<EarthHealthText>>,
        Query<&mut Text, With<ScoreText>>,
    )>,
    game: Res<Game>,
) {
    set.p0().single_mut().sections[0].value = format!("{}", game.health);
    set.p1().single_mut().sections[0].value = format!("{}", game.earth_health);
    set.p2().single_mut().sections[0].value = format!("{}", game.score);
}

fn level_up_msg(
    mut query: Query<&mut Text, With<MessageText>>,
    mut config: ResMut<MessageConfig>,
    mut ev_levelup: EventReader<LevelUpEvent>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
) {
    for _ev in ev_levelup.iter() {
        // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
        let text_style = TextStyle {
            font: asset_server.load("fonts/impact.ttf"),
            font_size: 42.0,
            color: Color::WHITE,
        };

        let mut text = query.single_mut();
        text.sections = vec![
            TextSection::new("LEVEL UP! New weapon in slot ", text_style.clone()),
            TextSection::new(game.level.to_string(), text_style.clone()),
            TextSection::new(" unlocked.", text_style.clone()),
        ];

        config.msg_timer = Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once);
    }
}

fn weapon_switched_msg(
    mut query: Query<&mut Text, With<MessageText>>,
    mut config: ResMut<MessageConfig>,
    mut ev_weaponswitched: EventReader<WeaponSwitchedEvent>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Handle<Image>)>,
) {
    let (mut player, _pos, _img) = player_query.single_mut();

    for _ev in ev_weaponswitched.iter() {
        // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
        let text_style = TextStyle {
            font: asset_server.load("fonts/impact.ttf"),
            font_size: 42.0,
            color: Color::WHITE,
        };

        let mut text = query.single_mut();
        text.sections = vec![
            TextSection::new(player.current_weapon().name.to_string(), text_style.clone()),
            TextSection::new(" equipped.", text_style.clone()),
        ];

        config.msg_timer = Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once);
    }
}

fn gameover_screen(
    mut query: Query<&mut Text, With<MessageText>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
) {
    // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 42.0,
        color: Color::WHITE,
    };

    let mut text = query.single_mut();
    text.sections = vec![
        TextSection::new("GAME OVER!\n", text_style.clone()),
        TextSection::new("YOUR SCORE: ", text_style.clone()),
        TextSection::new(game.score.to_string(), text_style.clone()),
    ];
}

fn gamewon_screen(
    mut query: Query<&mut Text, With<MessageText>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
) {
    // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 42.0,
        color: Color::WHITE,
    };

    let mut text = query.single_mut();
    text.sections = vec![
        TextSection::new(
            "VICTORY - YOU HAVE SUCCESSFULLY PROTECTED EARTH!!!\n",
            text_style.clone(),
        ),
        TextSection::new("YOUR SCORE: ", text_style.clone()),
        TextSection::new(game.score.to_string(), text_style.clone()),
        TextSection::new("\n", text_style.clone()),
        TextSection::new("YOUR FINAL SCORE: ", text_style.clone()),
        TextSection::new(
            (game.score + game.earth_health + game.health).to_string(),
            text_style.clone(),
        ),
    ];
}

fn pause_screen(mut query: Query<&mut Text, With<MessageText>>, asset_server: Res<AssetServer>) {
    // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 42.0,
        color: Color::WHITE,
    };

    let mut text = query.single_mut();
    text.sections = vec![TextSection::new("GAME PAUSED\n", text_style.clone())];
}

fn clear_msg_now(mut query: Query<&mut Text, With<MessageText>>) {
    let mut text = query.single_mut();
    text.sections = vec![];
}
