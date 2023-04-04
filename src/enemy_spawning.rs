use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{MyAssets, AppState, Enemy};

pub struct EnemySpawningPlugin;

impl Plugin for EnemySpawningPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(setup_enemy_spawning.in_schedule(OnEnter(AppState::InGame)))
        .add_system(spawn_enemy.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Resource)]
struct EnemySpawnConfig {
    timer: Timer,
}

fn setup_enemy_spawning(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    })
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EnemySpawnConfig>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let window = primary_query.single();

    config.timer.tick(time.delta());

    if config.timer.finished() {
        let enemy = Enemy::random();
        let img_handle = enemy.clone().image(my_assets);
        let img_size = assets.get(&img_handle).unwrap().size();

        let min_x_offset = -(window.width() / 2.0) + (img_size.x / 2.);
        let max_x_offset = window.width() / 2.0 - (img_size.x / 2.);

        commands.spawn((
            SpriteBundle {
                // TODO: Do not clone here.
                texture: img_handle,
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(min_x_offset..max_x_offset),
                    (window.height() / 2.) + (img_size.y / 2.),
                    0.,
                ),
                ..default()
            },
            enemy,
        ));
    }
}