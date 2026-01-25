
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_SIZE: f32 = 16.0;
pub const ENEMY_COUNT: u8 = 3;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 16.0;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies).chain())
        .add_systems(Update, (player_movement, enemy_movement, confine_player_movement, update_enemy_direction).chain())
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2
}

pub fn spawn_player
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    // .single() replaced .get_single().  The purpose of .single() is the same as .iter() except
    // that it will return only one instance that the query picks up (most likely the first one?).
    // If there isn't exactly one instance of the thing you're querying, an error will be returned.
    let window: &Window = window_query.single()?;

    commands.spawn((
        Player {},
        Sprite::from_image(asset_server.load("sprites/Items_BlackPearl.png")),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Visibility::Visible
    ));

    Ok(())
}

pub fn spawn_camera
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    // Creating a camera and placing its position at the center of the window.
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
    ));

    Ok(())
}

pub fn spawn_enemies
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    for _ in 0..=ENEMY_COUNT {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
            },
            Sprite::from_image(asset_server.load("sprites/Items_Star.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Visibility::Visible
        ));
    }

    Ok(())
}

pub fn player_movement
(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
)
{
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        // Normalization prevents the player from speeding up when moving diagonally.
        // We apply this whenever an intended direction is applied (any movement).
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement
(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
)
    -> Result<()>
{
    if let Ok(mut player_transform) = player_query.single_mut() {

        let window: &Window = window_query.single()?;

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        // Binding the Player X Position
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }

        // Binding the Player Y Position
        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }

    Ok(())
}

pub fn enemy_movement
(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
)
{
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn update_enemy_direction
(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {

        let mut direction_changed: bool = false;
        let translation: Vec3 = transform.translation;

        // Flipping the X direction if a horizontal boundary was hit.
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        // Flipping the Y direction if a vertical boundary was hit.
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            emit_sound(&mut commands,&asset_server,"audio/wall_hit.wav");
        }
    }

    Ok(())
}


// Gabe's dope function for sound.
pub fn emit_sound(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    path: &'static str
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load(path)),
        PlaybackSettings::DESPAWN)
    );
}
