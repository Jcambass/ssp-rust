//! Renders a 2D scene containing a single, moving sprite.

use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(Game { health: 100 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(limit_fps)
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
struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            speed: 5.618,
        }
    }
}

#[derive(Component, Clone)]
struct Enemy {
    pub image: String,
    pub health: u32,
    pub collision_damage: u32,
    pub bounty: u32,
    pub speed: f32,
}

impl Enemy {
    pub fn random() -> Self {
        let next_enemy_type =  rand::thread_rng().gen_range(0..100);

        if next_enemy_type < 45 {
            Self::trespasser()
        } else if next_enemy_type < 70 {
            Self::space_crusader()
        } else if next_enemy_type < 95 {
            Self::big_ship()
        } else {
            Self::dark_lord()
        }
    }

    pub fn big_ship() -> Self {
        let speed =  rand::thread_rng().gen_range(0.971 - 0.03..0.971 + 0.034);

        Self {
            image: String::from("ships/BigShip.png"),
            health: 100,
            speed: speed,
            collision_damage: 35,
            bounty: 120
        }
    }

    pub fn dark_lord() -> Self {
        let speed =  rand::thread_rng().gen_range(0.63 - 0.03..0.63 + 0.06);

        Self {
            image: String::from("ships/darkLord.png"),
            health: 250,
            speed: speed,
            collision_damage: 75,
            bounty: 250,
        }
    }

    pub fn space_crusader() -> Self {
        let speed =  rand::thread_rng().gen_range(1.001 - 0.04..1.001 + 0.04);

        Self {
            image: String::from("ships/spaceCrusader.png"),
            health: 80,
            speed: speed,
            collision_damage: 55,
            bounty: 180
        }
    }

    pub fn trespasser() -> Self {
        let speed =  rand::thread_rng().gen_range(1.53 - 0.09..1.53 + 0.08);

        Self {
            image: String::from("ships/trespasser.png"),
            health: 30,
            speed: speed,
            collision_damage: 13,
            bounty: 35
        }
    }
}

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

// TODO: Remove FPS Limit and use correct speed settings
fn limit_fps(mut settings: ResMut<bevy_framepace::FramepaceSettings>,) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(40.0);
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
        Player::new(),
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
        let enemy = Enemy::random();

        commands.spawn((
            SpriteBundle {
                // TODO: Do not clone here.
                texture: asset_server.load(enemy.clone().image),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    window.height() / 2.,
                    0.,
                ),
                ..default()
            },
            enemy,
        ));
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += player.speed; //* time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= player.speed; // * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed; // * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += player.speed; // * time.delta_seconds();
        }
    }
}

fn enemy_movement(time: Res<Time>, mut sprite_position: Query<(&mut Enemy, &mut Transform)>) {
    for (enemy, mut transform) in &mut sprite_position {
        transform.translation.y -= enemy.speed; // * time.delta_seconds();
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

    for (enemy_entity, enemy, pos, img) in &enemies_query {
        let enemy_size = assets.get(img).unwrap().size();

        if collide(
            pos.translation,
            enemy_size,
            player.1.translation,
            player_size,
        )
        .is_some()
        {
            game.health -= enemy.collision_damage;
            commands.entity(enemy_entity).despawn();
        }
    }
}
