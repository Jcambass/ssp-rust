use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

use crate::{
    AnimationIndices, AnimationTimer, AppState, Enemy, Game, Layers, MyAssets, Player,
    ORIGINAL_TARGET_FPS,
};

const PLAYER_WIDTH: f32 = 49.5;
const PLAYER_WING_TIPS: f32 = 25.0;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponSwitchedEvent>().add_systems(
            (
                player_shoot,
                enemy_shoot,
                projectile_move,
                projectile_collision,
                weapon_switching,
            )
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}

#[derive(Clone, Copy)]
pub enum ProjectileType {
    Blaster,
    Grim,
    Hammer,
    Ratata,
    Stomp,
}

#[derive(Component, Clone, Copy)]
pub struct Projectile {
    pub speed: f32,
    pub damage: u32,
    pub friendly: bool,
    pub projectile_type: ProjectileType,
    pub pushback: f32,
}

impl Projectile {
    pub fn image(&self, assets: &Res<MyAssets>) -> Handle<Image> {
        match self.projectile_type {
            ProjectileType::Blaster => assets.blaster.clone(),
            ProjectileType::Grim => assets.grim.clone(),
            ProjectileType::Hammer => assets.hammer.clone(),
            ProjectileType::Ratata => assets.ratata.clone(),
            ProjectileType::Stomp => assets.stomp.clone(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: String,
    pub cooldown_timer: Timer,
    pub gun_positions: Vec<Transform>,
    pub mounting_point: Transform,
    pub projectile: Projectile,
}

impl Weapon {
    pub fn stomp(friendly: bool) -> Self {
        let mut cooldown = 5.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }
        let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
        timer.pause();

        Weapon {
            name: String::from("Stomp O´ Matic"),
            cooldown_timer: timer,
            mounting_point: if friendly {
                Transform::from_xyz(-PLAYER_WIDTH, -PLAYER_WING_TIPS, 0.0)
            } else {
                Transform::from_xyz(0.0, 0.0, 0.0)
            },
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 13.0,
                    damage: 6,
                    projectile_type: ProjectileType::Stomp,
                    pushback: 0.0,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 10.0,
                    damage: 6,
                    projectile_type: ProjectileType::Stomp,
                    pushback: 0.0,
                }
            },
        }
    }

    pub fn blaster(friendly: bool) -> Self {
        let mut cooldown = 2300.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
        timer.pause();

        Weapon {
            name: String::from("Space Blaster"),
            cooldown_timer: timer,
            mounting_point: if friendly {
                Transform::from_xyz(PLAYER_WIDTH, -PLAYER_WING_TIPS, 0.0)
            } else {
                Transform::from_xyz(0.0, 0.0, 0.0)
            },
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 15.0,
                    damage: 12,
                    projectile_type: ProjectileType::Blaster,
                    pushback: 0.0,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 5.0,
                    damage: 50,
                    projectile_type: ProjectileType::Blaster,
                    pushback: 0.0,
                }
            },
        }
    }

    pub fn grim(friendly: bool) -> Self {
        let mut cooldown = 2500.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
        timer.pause();

        Weapon {
            name: String::from("Grim Reaper"),
            cooldown_timer: timer,
            mounting_point: if friendly {
                Transform::from_xyz(0.0, 0.0, 0.0)
            } else {
                Transform::from_xyz(0.0, 0.0, 0.0)
            },
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 5.0,
                    damage: 50,
                    projectile_type: ProjectileType::Grim,
                    pushback: 0.0,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 4.0,
                    damage: 40,
                    projectile_type: ProjectileType::Grim,
                    pushback: 0.0,
                }
            },
        }
    }

    pub fn hammer(friendly: bool) -> Self {
        let mut cooldown = 2000.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
        timer.pause();

        Weapon {
            name: String::from("Space Hammer"),
            cooldown_timer: timer,
            mounting_point: if friendly {
                Transform::from_xyz(0.0, 6.5, 0.0)
            } else {
                Transform::from_xyz(0.0, 0.0, 0.0)
            },
            gun_positions: vec![
                Transform::from_xyz(-24.0, 0.0, 0.0),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Transform::from_xyz(24.0, 0.0, 0.0),
            ],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 10.0,
                    damage: 4,
                    projectile_type: ProjectileType::Hammer,
                    pushback: 36.0,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 7.0,
                    damage: 4,
                    projectile_type: ProjectileType::Hammer,
                    pushback: 36.0,
                }
            },
        }
    }

    pub fn ratata(friendly: bool) -> Self {
        let mut cooldown = 4.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
        timer.pause();

        Weapon {
            name: String::from("Ratata 9000"),
            cooldown_timer: timer,
            mounting_point: if friendly {
                Transform::from_xyz(0.0, -PLAYER_WING_TIPS, 0.0)
            } else {
                Transform::from_xyz(0.0, 0.0, 0.0)
            },
            gun_positions: if friendly {
                vec![
                    Transform::from_xyz(-PLAYER_WIDTH, 0.0, 0.0),
                    Transform::from_xyz(-PLAYER_WIDTH+10.0, 0.0, 0.0),
                    Transform::from_xyz(PLAYER_WIDTH-10.0, 0.0, 0.0),
                    Transform::from_xyz(PLAYER_WIDTH, 0.0, 0.0),
                ]
            } else {
                vec![
                    Transform::from_xyz(-6.0, 0.0, 0.0),
                    Transform::from_xyz(6.0, 0.0, 0.0),
                ]
            },
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 15.0,
                    damage: 3,
                    projectile_type: ProjectileType::Ratata,
                    pushback: 0.0,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 12.0,
                    damage: 3,
                    projectile_type: ProjectileType::Ratata,
                    pushback: 0.0,
                }
            },
        }
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

    if keyboard_input.just_pressed(KeyCode::J)
        && (weapon.cooldown_timer.finished() || weapon.cooldown_timer.paused())
    {
        if weapon.cooldown_timer.paused() {
            weapon.cooldown_timer.unpause();
        }

        for pos in &weapon.gun_positions {
            let texture = weapon.projectile.image(&my_assets);
            let projectile_size = assets.get(&texture).unwrap().size();

            commands.spawn((
                SpriteBundle {
                    texture: texture,
                    transform: Transform::from_xyz(
                        transform.translation.x
                            + weapon.mounting_point.translation.x
                            + pos.translation.x,
                        transform.translation.y
                            + weapon.mounting_point.translation.y
                            + pos.translation.y
                            + player_size.y / 2.0
                            + projectile_size.y / 2.0,
                        Layers::Projectiles.order_nr(),
                    ),
                    ..default()
                },
                weapon.projectile,
            ));
        }
        weapon.cooldown_timer.reset()
    }
}

fn enemy_shoot(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies_query: Query<
        (Entity, &mut Enemy, &mut Transform, &mut Handle<Image>),
        Without<Player>,
    >,
    player_query: Query<(&mut Player, &mut Transform, &Handle<Image>), Without<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    my_assets: Res<MyAssets>,
) {
    let window = window_query.single();

    for (_enemy_entity, mut enemy, transform, img) in &mut enemies_query {
        let enemy_size = assets.get(&img).unwrap().size();

        if let Some(weapon) = &mut enemy.weapon {
            weapon.cooldown_timer.tick(time.delta());

            let (_player, player_pos, player_img) = player_query.single();
            let player_size = assets.get(&player_img).unwrap().size();

            let aim_height = window.height() / 2.0 + transform.translation.y;
            let aim_pos = Vec3 {
                x: transform.translation.x,
                y: transform.translation.y - aim_height / 2.0,
                z: Layers::Actors.order_nr(),
            };

            let aim_size = Vec2 {
                x: player_size.x,
                y: aim_height,
            };

            // Includes some buffer to give the player a slight advantage.
            let fully_visible =
                transform.translation.y + enemy_size.y / 2.0 <= window.height() / 2.0 - 12.0;

            if (weapon.cooldown_timer.finished() || weapon.cooldown_timer.paused())
                && fully_visible
                && collide(player_pos.translation, player_size, aim_pos, aim_size).is_some()
            {
                if weapon.cooldown_timer.paused() {
                    weapon.cooldown_timer.unpause();
                }

                for pos in &weapon.gun_positions {
                    let texture = weapon.projectile.image(&my_assets);
                    let projectile_size = assets.get(&texture).unwrap().size();

                    commands.spawn((
                        SpriteBundle {
                            texture: texture,
                            transform: Transform::from_xyz(
                                transform.translation.x
                                    + weapon.mounting_point.translation.x
                                    + pos.translation.x,
                                transform.translation.y
                                    + weapon.mounting_point.translation.y
                                    + pos.translation.y
                                    - enemy_size.y / 2.0
                                    - projectile_size.y / 2.0,
                                Layers::Projectiles.order_nr(),
                            ),
                            sprite: Sprite {
                                flip_y: true,
                                ..default()
                            },
                            ..default()
                        },
                        weapon.projectile,
                    ));
                }
                weapon.cooldown_timer.reset()
            }
        }
    }
}

fn projectile_move(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform, &Handle<Image>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
) {
    let window = window_query.single();

    for (proj_entity, projectile, mut transform, img_handle) in &mut projectiles {
        if projectile.friendly {
            transform.translation.y = transform.translation.y
                + projectile.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
        } else {
            transform.translation.y = transform.translation.y
                - projectile.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
        }

        let proj_size = assets.get(img_handle).unwrap().size();

        if projectile_past_top(transform.translation.y, window, proj_size)
            || projectile_past_bottom(transform.translation.y, window, proj_size)
        {
            commands.entity(proj_entity).despawn();
        }
    }
}

// TODO: DRYup helpers like that.
fn projectile_past_top(y: f32, window: &Window, proj_size: Vec2) -> bool {
    let max_y = (window.height() / 2.) + (proj_size.y / 2.);
    y > max_y
}

fn projectile_past_bottom(y: f32, window: &Window, proj_size: Vec2) -> bool {
    let min_y = -(window.height() / 2.) - (proj_size.y / 2.);
    y < min_y
}

fn projectile_collision(
    mut commands: Commands,
    mut game: ResMut<Game>,
    assets: Res<Assets<Image>>,
    mut player_query: Query<
        (Entity, &mut Player, &mut Transform, &mut Handle<Image>),
        (Without<Enemy>, Without<Projectile>),
    >,
    mut enemies_query: Query<
        (Entity, &mut Enemy, &mut Transform, &mut Handle<Image>),
        (Without<Player>, Without<Projectile>),
    >,
    mut projectiles: Query<
        (Entity, &mut Projectile, &mut Transform, &Handle<Image>),
        (Without<Enemy>, Without<Player>),
    >,
    my_assets: Res<MyAssets>,
) {
    for (proj_entity, projectile, transform, img_handle) in &mut projectiles {
        let projectile_size = assets.get(img_handle).unwrap().size();

        if projectile.friendly {
            for (enemy_entity, mut enemy, mut pos, img) in &mut enemies_query {
                let enemy_size = assets.get(&img).unwrap().size();

                if collide(
                    pos.translation,
                    enemy_size,
                    transform.translation,
                    projectile_size,
                )
                .is_some()
                {
                    commands.entity(proj_entity).despawn();

                    pos.translation.y += projectile.pushback;

                    enemy.health = if let Some(i) = enemy.health.checked_sub(projectile.damage) {
                        i
                    } else {
                        0
                    };

                    let hit_texture = match projectile.projectile_type {
                        ProjectileType::Blaster => my_assets.hit_red.clone(),
                        ProjectileType::Grim => my_assets.hit_blue.clone(),
                        ProjectileType::Hammer => my_assets.hit_red.clone(),
                        ProjectileType::Ratata => my_assets.hit_blue.clone(),
                        ProjectileType::Stomp => my_assets.hit_green.clone(),
                    };

                    let animation_indices = AnimationIndices { first: 0, last: 1 };
                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: hit_texture,
                            sprite: TextureAtlasSprite::new(animation_indices.first),
                            transform: Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y,
                                Layers::Actors.order_nr(),
                            ),
                            ..default()
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
                    ));

                    if enemy.health == 0 {
                        game.score += enemy.bounty;
                        commands.entity(enemy_entity).despawn();

                        let animation_indices = AnimationIndices { first: 0, last: 11 };
                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: my_assets.explosion.clone(),
                                sprite: TextureAtlasSprite::new(animation_indices.first),
                                transform: Transform::from_xyz(
                                    pos.translation.x,
                                    pos.translation.y,
                                    Layers::Actors.order_nr(),
                                ),
                                ..default()
                            },
                            animation_indices,
                            AnimationTimer(Timer::from_seconds(0.019, TimerMode::Repeating)),
                        ));
                    }
                }
            }
        } else {
            let (_player_entity, _player, mut pos, img) = player_query.single_mut();
            let player_size = assets.get(&img).unwrap().size();

            if collide(
                pos.translation,
                player_size,
                transform.translation,
                projectile_size,
            )
            .is_some()
            {
                commands.entity(proj_entity).despawn();

                pos.translation.y -= projectile.pushback;

                game.health = if let Some(i) = game.health.checked_sub(projectile.damage) {
                    i
                } else {
                    0
                };

                let hit_texture = match projectile.projectile_type {
                    ProjectileType::Blaster => my_assets.hit_red.clone(),
                    ProjectileType::Grim => my_assets.hit_blue.clone(),
                    ProjectileType::Hammer => my_assets.hit_red.clone(),
                    ProjectileType::Ratata => my_assets.hit_blue.clone(),
                    ProjectileType::Stomp => my_assets.hit_green.clone(),
                };

                let animation_indices = AnimationIndices { first: 0, last: 1 };
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: hit_texture,
                        sprite: TextureAtlasSprite::new(animation_indices.first),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            Layers::Actors.order_nr(),
                        ),
                        ..default()
                    },
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
                ));
            }
        }
    }
}

pub struct WeaponSwitchedEvent;

fn weapon_switching(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform, &Handle<Image>)>,
    mut ev_weaponswitched: EventWriter<WeaponSwitchedEvent>,
    game: Res<Game>,
) {
    let (mut player, _transform, _player_img_handle) = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Key1) {
        if player.current_weapon_index != 0 {
            player.current_weapon_index = 0;
            ev_weaponswitched.send(WeaponSwitchedEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key2) && game.level >= 2 {
        if player.current_weapon_index != 1 {
            player.current_weapon_index = 1;
            ev_weaponswitched.send(WeaponSwitchedEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key3) && game.level >= 3 {
        if player.current_weapon_index != 2 {
            player.current_weapon_index = 2;
            ev_weaponswitched.send(WeaponSwitchedEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key4) && game.level >= 4 {
        if player.current_weapon_index != 3 {
            player.current_weapon_index = 3;
            ev_weaponswitched.send(WeaponSwitchedEvent);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key5) && game.level >= 5 {
        if player.current_weapon_index != 4 {
            player.current_weapon_index = 4;
            ev_weaponswitched.send(WeaponSwitchedEvent);
        }
    }
}
