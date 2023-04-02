//! Renders a 2D scene containing a single, moving sprite.

use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Game { health: 100 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_startup_system(setup_enemy_spawning)
        .add_system(spawn_enemy)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(enemy_collision)
        .add_system(update_health)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct HealthText;

#[derive(Resource)]
struct EnemySpawnConfig {
    timer: Timer,
}

#[derive(Resource)]
struct Game {
    pub health: u32,
}

fn setup_enemy_spawning(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    })
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = primary_query.single();

    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Health: ", text_style.clone()),
            TextSection::new("100", text_style.clone()),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(0.0),
                left: Val::Px(window.width() / 2.),
                ..default()
            },
            ..default()
        }),
        HealthText,
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
    ));
}

fn update_health(mut query: Query<&mut Text, With<HealthText>>, game: Res<Game>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", game.health);
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EnemySpawnConfig>,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = primary_query.single();
    let min_x_offset = -(window.width() / 2.0);
    let max_x_offset = window.width() / 2.0;

    config.timer.tick(time.delta());

    if config.timer.finished() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("ships/spaceCrusader.png"),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    window.height() / 2.,
                    0.,
                ),
                ..default()
            },
            Enemy,
        ));
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
) {
    for (mut _player, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
}

fn enemy_movement(time: Res<Time>, mut sprite_position: Query<(&mut Enemy, &mut Transform)>) {
    for (mut _enemy, mut transform) in &mut sprite_position {
        transform.translation.y -= 50. * time.delta_seconds();
    }
}

use bevy::sprite::collide_aabb::collide;
fn enemy_collision(
    mut commands: Commands,
    mut game: ResMut<Game>,
    assets: Res<Assets<Image>>,
    player_query: Query<(&mut Player, &mut Transform, &mut Handle<Image>), Without<Enemy>>,
    enemies_query: Query<(Entity, &mut Enemy, &mut Transform, &mut Handle<Image>), Without<Player>>,
) {
    let player = player_query.single();
    let player_size = assets.get(player.2).unwrap().size();

    for (enemy_entity, _enemy, pos, img) in &enemies_query {
        let enemy_size = assets.get(img).unwrap().size();

        if collide(
            pos.translation,
            enemy_size,
            player.1.translation,
            player_size,
        )
        .is_some()
        {
            game.health -= 10;
            commands.entity(enemy_entity).despawn();
        }
    }
}
