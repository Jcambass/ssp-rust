use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppState, Player, ORIGINAL_TARGET_FPS};

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((player_movement, player_border).in_set(OnUpdate(AppState::InGame)));
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

fn player_border(
    mut player_query: Query<(&mut Player, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let (_player, mut pos, img) = player_query.single_mut();
    let player_size = assets.get(img).unwrap().size();

    let max_y = window.height()/2.0 - player_size.y/2.0;
    let min_y = -window.height()/2.0 + player_size.y/2.0;

    if pos.translation.y >  max_y {
        pos.translation.y = max_y;
    }

    if pos.translation.y < min_y{
        pos.translation.y = min_y;
    }

    let max_x = window.width()/2.0 - player_size.x/2.0;
    let min_x = -window.width()/2.0 + player_size.x/2.0;

    if pos.translation.x >  max_x {
        pos.translation.x = max_x;
    }

    if pos.translation.x < min_x{
        pos.translation.x = min_x;
    }
}