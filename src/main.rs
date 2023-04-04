//! Renders a 2D scene containing a single, moving sprite.

use std::time::Duration;

use bevy::{
    asset,
    prelude::*,
    text,
    window::{PresentMode, PrimaryWindow},
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use rand::Rng;

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
            health: 100,
            earth_health: 5000,
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems((setup, setup_ui, setup_enemy_spawning).in_schedule(OnEnter(AppState::InGame)))
        .add_systems(
            (
                player_movement,
                spawn_enemy,
                enemy_movement,
                despawn_enemies,
                enemy_collision,
                update_health,
                update_earth_health,
                check_game_over,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_system(gameover_screen.in_schedule(OnEnter(AppState::GameOver)))
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Loading,
    InGame,
    Paused,
    GameOver,
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
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { speed: 5.618 }
    }
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

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct EarthHealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct EnemySpawnConfig {
    timer: Timer,
}

#[derive(Resource)]
struct Game {
    pub health: u32,
    pub earth_health: u32,
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

    // No idea why the text of the flex box ones is not centerd until I add this one with PositionType::Absolute.
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Score: ", text_style.clone()),
            TextSection::new("0", text_style.clone()),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect::default(),
            ..default()
        }),
        ScoreText,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Earth Health: ", text_style.clone()),
                    TextSection::new("5000", text_style.clone()),
                ])
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    ..default()
                }),
                EarthHealthText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Health: ", text_style.clone()),
                    TextSection::new("100", text_style.clone()),
                ])
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    ..default()
                }),
                HealthText,
            ));
        });
}

fn setup(mut commands: Commands, my_assets: Res<MyAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: my_assets.player.clone(),
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

fn update_earth_health(mut query: Query<&mut Text, With<EarthHealthText>>, game: Res<Game>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", game.earth_health);
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

fn enemy_movement(time: Res<Time>, mut sprite_position: Query<(&mut Enemy, &mut Transform)>) {
    for (enemy, mut transform) in &mut sprite_position {
        transform.translation.y -= enemy.speed * time.delta_seconds() * ORIGINAL_TARGET_FPS;
    }
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
            println!("Despawned Enemy!")
        }
    }
}

fn enemy_past_bottom(y: f32, window: &Window, enemy_size: Vec2) -> bool {
    let min_y = -(window.height() / 2.) - (enemy_size.y / 2.);
    y < min_y
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
            game.health = if let Some(i) = game.health.checked_sub(enemy.collision_damage) {
                i
            } else {
                0
            };
            commands.entity(enemy_entity).despawn();
        }
    }
}

fn check_game_over(game: Res<Game>, mut next_state: ResMut<NextState<AppState>>) {
    if game.health <= 0 || game.earth_health <= 0 {
        next_state.set(AppState::GameOver);
    }
}

fn gameover_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = primary_query.single();

    // TODO: Text seems to render differently than original despite using the same font (double check) and same font size.
    let text_style = TextStyle {
        font: asset_server.load("fonts/impact.ttf"),
        font_size: 42.0,
        color: Color::WHITE,
    };

    commands.spawn((TextBundle::from_sections([
        TextSection::new("GAME OVER! ", text_style.clone()),
        TextSection::new("YOUR SCORE: 0", text_style.clone()),
    ])
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            bottom: Val::Px(window.height() / 2.),
            left: Val::Px(window.width() / 2.),
            ..default()
        },
        ..default()
    }),));
}
