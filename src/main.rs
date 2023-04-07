//! Renders a 2D scene containing a single, moving sprite.

use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;

pub mod backdrop;
pub mod enemy_spawning;
pub mod player_control;
pub mod ui;

const EARTH_HEALTH: u32 = 5000;
const PLAYER_HEALTH: u32 = 100;

const BACKGROUND_LAYER: f32 = 0.0;
const ACTOR_LAYER: f32 = 1.0;

const ORIGINAL_TARGET_FPS: f32 = 40.0;

// TODO: Ensure resizing window doesn't break things like enemy spawning/despawning, player movement or earth health.
// Moving Player certainly needs some tweaking since he can get stuck if the window is resized into the player.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SpaceShipProject Rust Edition!".into(),
                present_mode: PresentMode::AutoVsync,
                resolution: (1120., 605.).into(),
                // Tell wasm to use a specific canvas.
                canvas: Some(String::from("#mainScreen")),
                // Tells wasm NOT to resize the window according to the available canvas.
                fit_canvas_to_parent: false,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame))
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .insert_resource(Game {
            health: PLAYER_HEALTH,
            earth_health: EARTH_HEALTH,
            score: 0,
        })
        // TODO: Find a way so that it doesn't run when unpausing the game
        .add_system(setup.in_schedule(OnExit(AppState::Loading)))
        .add_systems(
            (
                despawn_enemies,
                enemy_collision,
                animate_sprite,
                check_game_over,
                check_game_won.after(check_game_over),
                check_game_paused,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_system(check_game_unpaused.in_set(OnUpdate(AppState::Paused)))
        .add_plugin(backdrop::BackdropPlugin)
        .add_plugin(enemy_spawning::EnemySpawningPlugin)
        .add_plugin(player_control::PlayerControlPlugin)
        .add_plugin(ui::UiOverlayPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Loading,
    InGame,
    Paused,
    GameOver,
    GameWon,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "player.png")]
    player: Handle<Image>,
    #[asset(path = "ships/BigShip.png")]
    big_ship: Handle<Image>,
    #[asset(path = "ships/darkLord.png")]
    dark_lord: Handle<Image>,
    #[asset(path = "ships/spaceCrusader.png")]
    space_crusader: Handle<Image>,
    #[asset(path = "ships/trespasser.png")]
    trespasser: Handle<Image>,
    #[asset(path = "backdrop/star.png")]
    star: Handle<Image>,
    #[asset(path = "backdrop/planet01.png")]
    planet01: Handle<Image>,
    #[asset(path = "backdrop/planet02.png")]
    planet02: Handle<Image>,
    #[asset(path = "backdrop/planet03.png")]
    planet03: Handle<Image>,
    #[asset(path = "backdrop/planet04.png")]
    planet04: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 134.,
        tile_size_y = 134.,
        columns = 12,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.,
        offset_x = 0.,
        offset_y = 0.
    ))]
    #[asset(path = "explosion.png")]
    explosion: Handle<TextureAtlas>,
    #[asset(path = "projectiles/blaster.png")]
    blaster: Handle<Image>,
    #[asset(path = "projectiles/grim.png")]
    grim: Handle<Image>,
    #[asset(path = "projectiles/hammer.png")]
    hammer: Handle<Image>,
    #[asset(path = "projectiles/ratata.png")]
    ratata: Handle<Image>,
    #[asset(path = "projectiles/stomp.png")]
    stomp: Handle<Image>,
}

#[derive(Component)]
struct Player {
    pub speed: f32,
    pub current_weapon_index: usize,
    pub weapons: Vec<Weapon>,
}

impl Player {
    pub fn new() -> Self {
        let weapons = vec![
            Weapon {
                name: String::from("Stomp O´ Matic"),
                cooldown_timer: Timer::new(Duration::from_secs_f32(5.0/ORIGINAL_TARGET_FPS), TimerMode::Once),
                mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
                gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
                player_projectile: Projectile::stomp(true),
                enemy_projectile: Projectile::stomp(false),
            },
            Weapon {
                name: String::from("Space Blaster"),
                cooldown_timer: Timer::new(Duration::from_secs_f32(2300.0 / 40.0/ORIGINAL_TARGET_FPS), TimerMode::Once),
                mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
                gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
                player_projectile: Projectile::blaster(true),
                enemy_projectile: Projectile::blaster(false),
            },
            Weapon {
                name: String::from("Grim Reaper"),
                cooldown_timer: Timer::new(Duration::from_secs_f32(2500.0 / 40.0/ORIGINAL_TARGET_FPS), TimerMode::Once),
                mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
                gun_positions: vec![Transform::from_xyz(0.0, 0.0, 0.0)],
                player_projectile: Projectile::grim(true),
                enemy_projectile: Projectile::grim(false),
            },
            Weapon {
                name: String::from("Space Hammer"),
                cooldown_timer: Timer::new(Duration::from_secs_f32(2000.0 / 40.0/ORIGINAL_TARGET_FPS), TimerMode::Once),
                mounting_point: Transform::from_xyz(0.0, 0.0, 0.0),
                gun_positions: vec![
                    Transform::from_xyz(-24.0, 0.0, 0.0),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    Transform::from_xyz(24.0, 0.0, 0.0),
                ],
                player_projectile: Projectile::hammer(true),
                enemy_projectile: Projectile::hammer(false),
            },
            Weapon {
                name: String::from("Ratata 9000"),
                cooldown_timer: Timer::new(Duration::from_secs_f32(4.0/ORIGINAL_TARGET_FPS), TimerMode::Once),
                mounting_point: Transform::from_xyz(10.0, 6.5, 0.0),
                gun_positions: vec![
                    Transform::from_xyz(-6.0, 0.0, 0.0),
                    Transform::from_xyz(6.0, 0.0, 0.0),
                ],
                player_projectile: Projectile::ratata(true),
                enemy_projectile: Projectile::ratata(false),
            },
        ];

        Self {
            speed: 5.618,
            weapons: weapons,
            current_weapon_index: 0,
        }
    }

    pub fn current_weapon(&mut self) -> &mut Weapon {
        &mut self.weapons[self.current_weapon_index]
    }
}

#[derive(Clone, Copy)]
enum ProjectileType {
    Blaster,
    Grim,
    Hammer,
    Ratata,
    Stomp,
}

#[derive(Component, Clone, Copy)]
struct Projectile {
    pub speed: f32,
    pub damage: f32,
    pub friendly: bool,
    pub projectile_type: ProjectileType,
}

impl Projectile {
    pub fn stomp(friendly: bool) -> Self {
        if friendly {
            Self {
                friendly: true,
                speed: 13.0,
                damage: 6.0,
                projectile_type: ProjectileType::Stomp,
            }
        } else {
            Self {
                friendly: false,
                speed: 10.0,
                damage: 6.0,
                projectile_type: ProjectileType::Stomp,
            }
        }
    }

    pub fn blaster(friendly: bool) -> Self {
        if friendly {
            Self {
                friendly: true,
                speed: 15.0,
                damage: 12.0,
                projectile_type: ProjectileType::Blaster,
            }
        } else {
            Self {
                friendly: false,
                speed: 5.0,
                damage: 50.0,
                projectile_type: ProjectileType::Blaster,
            }
        }
    }

    pub fn grim(friendly: bool) -> Self {
        if friendly {
            Self {
                friendly: true,
                speed: 5.0,
                damage: 50.0,
                projectile_type: ProjectileType::Grim,
            }
        } else {
            Self {
                friendly: false,
                speed: 4.0,
                damage: 40.0,
                projectile_type: ProjectileType::Grim,
            }
        }
    }

    pub fn hammer(friendly: bool) -> Self {
        if friendly {
            Self {
                friendly: true,
                speed: 10.0,
                damage: 4.0,
                projectile_type: ProjectileType::Hammer,
            }
        } else {
            Self {
                friendly: false,
                speed: 7.0,
                damage: 4.0,
                projectile_type: ProjectileType::Hammer,
            }
        }
    }

    pub fn ratata(friendly: bool) -> Self {
        if friendly {
            Self {
                friendly: true,
                speed: 15.0,
                damage: 3.0,
                projectile_type: ProjectileType::Ratata,
            }
        } else {
            Self {
                friendly: false,
                speed: 12.0,
                damage: 3.0,
                projectile_type: ProjectileType::Ratata,
            }
        }
    }

    pub fn image(&self, assets: &Res<MyAssets>) -> Handle<Image> {
        match self.projectile_type{
            ProjectileType::Blaster => assets.blaster.clone(),
            ProjectileType::Grim => assets.grim.clone(),
            ProjectileType::Hammer => assets.hammer.clone(),
            ProjectileType::Ratata => assets.ratata.clone(),
            ProjectileType::Stomp => assets.stomp.clone(),
        }
    }
}

#[derive(Component)]
struct Weapon {
    pub name: String,
    pub cooldown_timer: Timer,
    pub gun_positions: Vec<Transform>,
    pub mounting_point: Transform,
    pub player_projectile: Projectile,
    pub enemy_projectile: Projectile,
}

#[derive(Clone)]
enum ShipType {
    BigShip,
    DarkLord,
    SpeedCrusader,
    Trespasser,
}

#[derive(Component, Clone)]
struct Enemy {
    pub ship_type: ShipType,
    pub health: u32,
    pub collision_damage: u32,
    pub bounty: u32,
    pub speed: f32,
}

impl Enemy {
    pub fn random() -> Self {
        let next_enemy_type = rand::thread_rng().gen_range(0..100);

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
        let speed = rand::thread_rng().gen_range(0.971 - 0.03..0.971 + 0.034);

        Self {
            ship_type: ShipType::BigShip,
            health: 100,
            speed: speed,
            collision_damage: 35,
            bounty: 120,
        }
    }

    pub fn dark_lord() -> Self {
        let speed = rand::thread_rng().gen_range(0.63 - 0.03..0.63 + 0.06);

        Self {
            ship_type: ShipType::DarkLord,
            health: 250,
            speed: speed,
            collision_damage: 75,
            bounty: 250,
        }
    }

    pub fn space_crusader() -> Self {
        let speed = rand::thread_rng().gen_range(1.001 - 0.04..1.001 + 0.04);

        Self {
            ship_type: ShipType::SpeedCrusader,
            health: 80,
            speed: speed,
            collision_damage: 55,
            bounty: 180,
        }
    }

    pub fn trespasser() -> Self {
        let speed = rand::thread_rng().gen_range(1.53 - 0.09..1.53 + 0.08);

        Self {
            ship_type: ShipType::Trespasser,
            health: 30,
            speed: speed,
            collision_damage: 13,
            bounty: 35,
        }
    }

    pub fn image(&self, assets: Res<MyAssets>) -> Handle<Image> {
        match self.ship_type {
            ShipType::BigShip => assets.big_ship.clone(),
            ShipType::DarkLord => assets.dark_lord.clone(),
            ShipType::SpeedCrusader => assets.space_crusader.clone(),
            ShipType::Trespasser => assets.trespasser.clone(),
        }
    }
}

#[derive(Resource)]
struct Game {
    pub health: u32,
    pub earth_health: u32,
    pub score: u32,
}

fn setup(mut commands: Commands, my_assets: Res<MyAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: my_assets.player.clone(),
            transform: Transform::from_xyz(0., 0., ACTOR_LAYER),
            ..default()
        },
        Player::new(),
    ));
}

fn despawn_enemies(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut sprite_position: Query<(Entity, &mut Enemy, &mut Transform, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (enemy_entity, enemy, transform, img_handle) in &mut sprite_position {
        let enemy_size = assets.get(img_handle).unwrap().size();

        if enemy_past_bottom(transform.translation.y, window, enemy_size) {
            game.earth_health = if let Some(i) = game.earth_health.checked_sub(enemy.bounty) {
                i
            } else {
                0
            };

            commands.entity(enemy_entity).despawn();
        }
    }
}

fn enemy_past_bottom(y: f32, window: &Window, enemy_size: Vec2) -> bool {
    let min_y = -(window.height() / 2.) - (enemy_size.y / 2.);
    y < min_y
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index == indices.last {
                commands.entity(entity).despawn();
            } else {
                sprite.index = sprite.index + 1;
            };
        }
    }
}

use bevy::sprite::collide_aabb::collide;

fn enemy_collision(
    mut commands: Commands,
    mut game: ResMut<Game>,
    assets: Res<Assets<Image>>,
    player_query: Query<(&mut Player, &mut Transform, &mut Handle<Image>), Without<Enemy>>,
    enemies_query: Query<(Entity, &mut Enemy, &mut Transform, &mut Handle<Image>), Without<Player>>,
    my_assets: Res<MyAssets>,
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
            game.health = if let Some(i) = game.health.checked_sub(enemy.collision_damage) {
                i
            } else {
                0
            };

            game.score += enemy.bounty;
            commands.entity(enemy_entity).despawn();

            let animation_indices = AnimationIndices { first: 0, last: 11 };
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: my_assets.explosion.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform::from_xyz(pos.translation.x, pos.translation.y, 0.),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.019, TimerMode::Repeating)),
            ));
        }
    }
}

fn check_game_paused(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

fn check_game_unpaused(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::InGame);
    }
}

fn check_game_over(game: Res<Game>, mut next_state: ResMut<NextState<AppState>>) {
    if game.health <= 0 || game.earth_health <= 0 {
        next_state.set(AppState::GameOver);
    }
}

fn check_game_won(game: Res<Game>, mut next_state: ResMut<NextState<AppState>>) {
    if game.score >= 8000 {
        next_state.set(AppState::GameWon);
    }
}
