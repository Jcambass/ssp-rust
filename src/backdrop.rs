use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow, reflect::erased_serde::__private::serde::__private::de};
use rand::Rng;

use crate::{AppState, ORIGINAL_TARGET_FPS, MyAssets, BACKGROUND_LAYER};

const MIN_STAR_SPAWN_SECONDS: f32 = 1.0;
const MAX_STAR_SPAWN_SECONDS: f32 = 2.4;

pub struct BackdropPlugin;

impl Plugin for BackdropPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems((
            setup_initial_backdrop,
            setup_backdrop_spawning
        ).in_schedule(OnEnter(AppState::InGame)))
        .add_systems((
            spawn_stars,
            star_movement,
            despawn_star,
        ).in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Resource)]
struct BackdropSpawnConfig {
    start_timer: Timer,
}

#[derive(Component)]
struct Star {
    pub speed: f32,
}

impl Star {
    pub fn new() -> Self {
        Self { speed: 0.413 }
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
        let star = Star::new();
        commands.spawn((
            SpriteBundle {
                texture: img_handle.clone(),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    rand::thread_rng().gen_range(min_y_offset..max_y_offset),
                    BACKGROUND_LAYER,
                ),
                ..default()
            },
            star,
        ));
    }
}

fn setup_backdrop_spawning(mut commands: Commands) {
    commands.insert_resource(BackdropSpawnConfig {
        start_timer: Timer::new(Duration::from_secs_f32(random_star_spawn_time()), TimerMode::Once),
    })
}

fn random_star_spawn_time() -> f32 {
    rand::thread_rng().gen_range(MIN_STAR_SPAWN_SECONDS..=MAX_STAR_SPAWN_SECONDS)
}

fn star_movement(time: Res<Time>, mut sprite_position: Query<(&mut Star, &mut Transform)>) {
    for (star, mut transform) in &mut sprite_position {
        transform.translation.y -= star.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
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

    config.start_timer.tick(time.delta());

    if config.start_timer.finished() {
        config.start_timer =  Timer::new(Duration::from_secs_f32(random_star_spawn_time()), TimerMode::Once);

        let star = Star::new();
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
                    BACKGROUND_LAYER,
                ),
                ..default()
            },
            star,
        ));
    }
}


fn despawn_star(
    mut commands: Commands,
    mut sprite_position: Query<(Entity, &mut Star, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (star_entity, _star, transform, img_handle) in &mut sprite_position {
        let star_size = assets.get(img_handle).unwrap().size();

        if star_past_bottom(transform.translation.y, window, star_size) {
            commands.entity(star_entity).despawn();
            println!("Despawned Star!")
        }
    }
}

fn star_past_bottom(y: f32, window: &Window, star_size: Vec2) -> bool {
    let min_y = -(window.height() / 2.) - (star_size.y / 2.);
    y < min_y
}