use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppState, Player, ORIGINAL_TARGET_FPS, ACTOR_LAYER, MyAssets, Projectile};

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            player_movement,
            player_shoot,
            projectile_move
        ).in_set(OnUpdate(AppState::InGame)));
    }
}

fn player_shoot(
    time: Res<Time>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_position: Query<(&mut Player, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let (mut player, transform, player_img_handle) = player_position.single_mut();
    let player_size = assets.get(player_img_handle).unwrap().size();

    let weapon = player.current_weapon();
    weapon.cooldown_timer.tick(time.delta());

    if keyboard_input.just_pressed(KeyCode::J) && weapon.cooldown_timer.finished() {
        for pos in &weapon.gun_positions {
            let texture = weapon.player_projectile.image(&my_assets);
            let projectile_size = assets.get(&texture).unwrap().size();

            commands.spawn((
                SpriteBundle {
                    texture: texture,
                    transform: Transform::from_xyz(
                        transform.translation.x + weapon.mounting_point.translation.x + pos.translation.x,
                        transform.translation.y + weapon.mounting_point.translation.y + pos.translation.y + player_size.y/2.0 + projectile_size.y/2.0,
                        ACTOR_LAYER,
                    ),
                    ..default()
                },
                weapon.player_projectile,
            ));
        }
        weapon.cooldown_timer.reset()
    }
}

fn projectile_move(
    time: Res<Time>,
    mut projectiles: Query<(&mut Projectile, &mut Transform, &Handle<Image>)>,
) {
    for (projectile, mut transform, _img_handle) in &mut projectiles {
        transform.translation.y = transform.translation.y + projectile.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Player, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (player, mut transform, img_handle) in &mut sprite_position {
        let player_size = assets.get(img_handle).unwrap().size();

        if keyboard_input.pressed(KeyCode::W) {
            let new_y =
                transform.translation.y + player.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
            if valid_move(transform.translation.x, new_y, window, player_size) {
                transform.translation.y = new_y;
            }
        }

        if keyboard_input.pressed(KeyCode::S) {
            let new_y =
                transform.translation.y - player.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
            if valid_move(transform.translation.x, new_y, window, player_size) {
                transform.translation.y = new_y;
            }
        }

        if keyboard_input.pressed(KeyCode::A) {
            let new_x =
                transform.translation.x - player.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
            if valid_move(new_x, transform.translation.y, window, player_size) {
                transform.translation.x = new_x;
            }
        }

        if keyboard_input.pressed(KeyCode::D) {
            let new_x =
                transform.translation.x + player.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
            if valid_move(new_x, transform.translation.y, window, player_size) {
                transform.translation.x = new_x;
            }
        }
    }
}

fn valid_move(x: f32, y: f32, window: &Window, player_size: Vec2) -> bool {
    let min_x = -(window.width() / 2.) + (player_size.x / 2.);
    let max_x = (window.width() / 2.) - (player_size.x / 2.);

    let min_y = -(window.height() / 2.) + (player_size.y / 2.);
    let max_y = (window.height() / 2.) - (player_size.y / 2.);
    x >= min_x && x <= max_x && y >= min_y && y <= max_y
}
