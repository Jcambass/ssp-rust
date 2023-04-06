use bevy::prelude::*;

use crate::{AppState, Game, EARTH_HEALTH, PLAYER_HEALTH};

pub struct UiOverlayPlugin;

impl Plugin for UiOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnExit(AppState::Loading)))
            .add_system(update_stats.in_set(OnUpdate(AppState::InGame)))
            .add_system(gameover_screen.in_schedule(OnEnter(AppState::GameOver)));
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
                TextBundle::from_sections([]).with_text_alignment(TextAlignment::Center),
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
