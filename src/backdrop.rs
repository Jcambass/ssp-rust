use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{AppState, MyAssets, ORIGINAL_TARGET_FPS, Layers};

const MIN_STAR_SPAWN_SECONDS: f32 = 1.0;
const MAX_STAR_SPAWN_SECONDS: f32 = 2.4;

const MIN_PLANET_SPAWN_SECONDS: f32 = 80.0;
const MAX_PLANET_SPAWN_SECONDS: f32 = 240.0;

pub struct BackdropPlugin;

impl Plugin for BackdropPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(
                (setup_initial_backdrop, setup_backdrop_spawning)
                    .in_schedule(OnExit(AppState::Loading)),
            )
            .add_systems(
                (
                    spawn_stars,
                    spawn_planets,
                    obstacle_movement,
                    despawn_obstacle,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}

#[derive(Resource)]
struct BackdropSpawnConfig {
    star_timer: Timer,
    planet_timer: Timer,
}

#[derive(Component)]
struct Obstacle {
    pub speed: f32,
}

impl Obstacle {
    pub fn star() -> Self {
        Self { speed: 0.413 }
    }

    pub fn planet() -> Self {
        Self { speed: 0.47 }
    }
}

fn setup_initial_backdrop(
    mut commands: Commands,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let window = primary_query.single();

    let img_handle = my_assets.star.clone();
    let img_size = assets.get(&img_handle).unwrap().size();

    let min_x_offset = -(window.width() / 2.0) + (img_size.x / 2.);
    let max_x_offset = window.width() / 2.0 - (img_size.x / 2.);

    let min_y_offset = -(window.height() / 2.0) + (img_size.y / 2.);
    let max_y_offset = window.height() / 2.0 - (img_size.y / 2.);

    for _ in 0..30 {
        // TODO: Technicaly they have a slightly different speed here, but it's close enough for now.
        let star = Obstacle::star();
        commands.spawn((
            SpriteBundle {
                texture: img_handle.clone(),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    rand::thread_rng().gen_range(min_y_offset..max_y_offset),
                    Layers::Stars.order_nr(),
                ),
                ..default()
            },
            star,
        ));
    }

    let planet = Obstacle::planet();

    let next_planet_type = rand::thread_rng().gen_range(0..100);
    let img_handle = if next_planet_type < 45 {
        my_assets.planet01.clone()
    } else if next_planet_type < 70 {
        my_assets.planet02.clone()
    } else if next_planet_type < 95 {
        my_assets.planet03.clone()
    } else {
        my_assets.planet04.clone()
    };

    let img_size = assets.get(&img_handle).unwrap().size();

    let min_x_offset = -(window.width() / 2.0) + (img_size.x / 2.);
    let max_x_offset = window.width() / 2.0 - (img_size.x / 2.);

    commands.spawn((
        SpriteBundle {
            texture: img_handle,
            transform: Transform::from_xyz(
                rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                (window.height() / 2.) - (img_size.y / 2.),
                Layers::Planets.order_nr(),
            ),
            ..default()
        },
        planet,
    ));
}

fn setup_backdrop_spawning(mut commands: Commands) {
    commands.insert_resource(BackdropSpawnConfig {
        star_timer: Timer::new(
            Duration::from_secs_f32(random_star_spawn_time()),
            TimerMode::Once,
        ),
        planet_timer: Timer::new(
            Duration::from_secs_f32(random_planet_spawn_time()),
            TimerMode::Once,
        ),
    })
}

fn random_star_spawn_time() -> f32 {
    rand::thread_rng().gen_range(MIN_STAR_SPAWN_SECONDS..=MAX_STAR_SPAWN_SECONDS)
}

fn random_planet_spawn_time() -> f32 {
    rand::thread_rng().gen_range(MIN_PLANET_SPAWN_SECONDS..=MAX_PLANET_SPAWN_SECONDS)
}

fn obstacle_movement(time: Res<Time>, mut sprite_position: Query<(&mut Obstacle, &mut Transform)>) {
    for (obs, mut transform) in &mut sprite_position {
        transform.translation.y -= obs.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
    }
}

fn spawn_stars(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<BackdropSpawnConfig>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let window = primary_query.single();

    config.star_timer.tick(time.delta());

    if config.star_timer.finished() {
        config.star_timer = Timer::new(
            Duration::from_secs_f32(random_star_spawn_time()),
            TimerMode::Once,
        );

        let star = Obstacle::star();
        let img_handle = my_assets.star.clone();
        let img_size = assets.get(&img_handle).unwrap().size();

        let min_x_offset = -(window.width() / 2.0) + (img_size.x / 2.);
        let max_x_offset = window.width() / 2.0 - (img_size.x / 2.);

        commands.spawn((
            SpriteBundle {
                texture: img_handle,
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    (window.height() / 2.) + (img_size.y / 2.),
                    Layers::Stars.order_nr(),
                ),
                ..default()
            },
            star,
        ));
    }
}

fn spawn_planets(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<BackdropSpawnConfig>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let window = primary_query.single();

    config.planet_timer.tick(time.delta());

    if config.planet_timer.finished() {
        config.planet_timer = Timer::new(
            Duration::from_secs_f32(random_planet_spawn_time()),
            TimerMode::Once,
        );

        let planet = Obstacle::planet();

        let next_planet_type = rand::thread_rng().gen_range(0..100);
        let img_handle = if next_planet_type < 45 {
            my_assets.planet01.clone()
        } else if next_planet_type < 70 {
            my_assets.planet02.clone()
        } else if next_planet_type < 95 {
            my_assets.planet03.clone()
        } else {
            my_assets.planet04.clone()
        };

        let img_size = assets.get(&img_handle).unwrap().size();

        let min_x_offset = -(window.width() / 2.0) + (img_size.x / 2.);
        let max_x_offset = window.width() / 2.0 - (img_size.x / 2.);

        commands.spawn((
            SpriteBundle {
                texture: img_handle,
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    (window.height() / 2.) + (img_size.y / 2.),
                    Layers::Planets.order_nr(),
                ),
                ..default()
            },
            planet,
        ));
    }
}

fn despawn_obstacle(
    mut commands: Commands,
    mut sprite_position: Query<(Entity, &mut Obstacle, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (obs_entity, _obs, transform, img_handle) in &mut sprite_position {
        let obs_size = assets.get(img_handle).unwrap().size();

        if obstacle_past_bottom(transform.translation.y, window, obs_size) {
            commands.entity(obs_entity).despawn();
        }
    }
}

fn obstacle_past_bottom(y: f32, window: &Window, obj_size: Vec2) -> bool {
    let min_y = -(window.height() / 2.) - (obj_size.y / 2.);
    y < min_y
}
