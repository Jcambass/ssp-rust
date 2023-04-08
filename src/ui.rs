use std::time::Duration;

use bevy::prelude::*;

use crate::{AppState, Game, LevelUpEvent, EARTH_HEALTH, PLAYER_HEALTH, shooting::WeaponSwitchedEvent, Player};

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
struct EarthHealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
pub struct MessageText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    };

    // No idea why the text of the flex box ones is not centerd until I add this one with PositionType::Absolute.
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Score: ", text_style.clone()),
            TextSection::new("0", text_style.clone()),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect::default(),
            ..default()
        }),
        ScoreText,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Earth Health: ", text_style.clone()),
                    TextSection::new(EARTH_HEALTH.to_string(), text_style.clone()),
                ])
                .with_text_alignment(TextAlignment::Center),
                EarthHealthText,
            ));

            parent.spawn((
                TextBundle::from_sections([TextSection::new(
                    "PROTECT EARTH AS LONG AS YOU CAN!!!",
                    text_style.clone(),
                )])
                .with_text_alignment(TextAlignment::Center),
                MessageText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Health: ", text_style.clone()),
                    TextSection::new(PLAYER_HEALTH.to_string(), text_style.clone()),
                ])
                .with_text_alignment(TextAlignment::Center),
                HealthText,
            ));
        });

    commands.insert_resource(MessageConfig {
        msg_timer: Timer::new(Duration::from_secs_f32(4.25), TimerMode::Once),
    })
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
    set.p0().single_mut().sections[1].value = format!("{}", game.health);
    set.p1().single_mut().sections[1].value = format!("{}", game.earth_health);
    set.p2().single_mut().sections[1].value = format!("{}", game.score);
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
        TextSection::new("VICTORY - YOU HAVE SUCCESSFULLY PROTECTED EARTH!!!\n", text_style.clone()),
        TextSection::new("YOUR SCORE: ", text_style.clone()),
        TextSection::new(game.score.to_string(), text_style.clone()),
        TextSection::new("\n", text_style.clone()),
        TextSection::new("YOUR FINAL SCORE: ", text_style.clone()),
        TextSection::new((game.score + game.earth_health + game.health).to_string(), text_style.clone()),
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
