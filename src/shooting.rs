use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

use crate::{
    AnimationIndices, AnimationTimer, AppState, Enemy, Game, MyAssets, Player, ACTOR_LAYER,
    ORIGINAL_TARGET_FPS,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                player_shoot,
                enemy_shoot,
                projectile_move,
                projectile_collision,
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

        Weapon {
            name: String::from("Stomp O´ Matic"),
            cooldown_timer: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 13.0,
                    damage: 6,
                    projectile_type: ProjectileType::Stomp,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 10.0,
                    damage: 6,
                    projectile_type: ProjectileType::Stomp,
                }
            },
        }
    }

    pub fn blaster(friendly: bool) -> Self {
        let mut cooldown = 2300.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        Weapon {
            name: String::from("Space Blaster"),
            cooldown_timer: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 15.0,
                    damage: 12,
                    projectile_type: ProjectileType::Blaster,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 5.0,
                    damage: 50,
                    projectile_type: ProjectileType::Blaster,
                }
            },
        }
    }

    pub fn grim(friendly: bool) -> Self {
        let mut cooldown = 2500.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        Weapon {
            name: String::from("Grim Reaper"),
            cooldown_timer: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
            gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 5.0,
                    damage: 50,
                    projectile_type: ProjectileType::Grim,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 4.0,
                    damage: 40,
                    projectile_type: ProjectileType::Grim,
                }
            },
        }
    }

    pub fn hammer(friendly: bool) -> Self {
        let mut cooldown = 2000.0 / 40.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        Weapon {
            name: String::from("Space Hammer"),
            cooldown_timer: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
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
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 7.0,
                    damage: 4,
                    projectile_type: ProjectileType::Hammer,
                }
            },
        }
    }

    pub fn ratata(friendly: bool) -> Self {
        let mut cooldown = 4.0 / ORIGINAL_TARGET_FPS;
        if !friendly {
            cooldown *= 2.0
        }

        Weapon {
            name: String::from("Ratata 9000"),
            cooldown_timer: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            mounting_point: Transform::from_xyz(10.0, 6.5, 0.0),
            gun_positions: vec![
                Transform::from_xyz(-6.0, 0.0, 0.0),
                Transform::from_xyz(6.0, 0.0, 0.0),
            ],
            projectile: if friendly {
                Projectile {
                    friendly: true,
                    speed: 15.0,
                    damage: 3,
                    projectile_type: ProjectileType::Ratata,
                }
            } else {
                Projectile {
                    friendly: false,
                    speed: 12.0,
                    damage: 3,
                    projectile_type: ProjectileType::Ratata,
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

    if keyboard_input.just_pressed(KeyCode::J) && weapon.cooldown_timer.finished() {
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
                        ACTOR_LAYER,
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

        let weapon = &mut enemy.weapon;

        weapon.cooldown_timer.tick(time.delta());

        let (_player, player_pos, player_img) = player_query.single();
        let player_size = assets.get(&player_img).unwrap().size();

        let aim_pos = transform.translation;
        let aim_size = Vec2 { x: player_size.x, y: window.height() };

        // Includes some buffer to give the player a slight advantage.
        let fully_visible = transform.translation.y + enemy_size.y/2.0 <= window.height()/2.0 - 12.0;

        if weapon.cooldown_timer.finished() && fully_visible && collide(
            player_pos.translation,
            player_size,
            aim_pos,
            aim_size,
        )
        .is_some() {
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
                            ACTOR_LAYER,
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
            transform.translation.y =
                transform.translation.y + projectile.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
        } else {
            transform.translation.y =
                transform.translation.y - projectile.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
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
    player_query: Query<
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
            for (enemy_entity, mut enemy, pos, img) in &mut enemies_query {
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

                    enemy.health = if let Some(i) = enemy.health.checked_sub(projectile.damage) {
                        i
                    } else {
                        0
                    };

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
                                    0.,
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
            let (_player_entity, _player, pos, img) = player_query.single();
            let player_size = assets.get(img).unwrap().size();

            if collide(
                pos.translation,
                player_size,
                transform.translation,
                projectile_size,
            )
            .is_some()
            {
                commands.entity(proj_entity).despawn();
                game.health = if let Some(i) = game.health.checked_sub(projectile.damage) {
                    i
                } else {
                    0
                };
            }
        }
    }
}
