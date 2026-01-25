
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 16.0;
pub const ENEMY_COUNT: u8 = 4;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies).chain())
        .add_systems(Update, (player_movement, confine_player_movement).chain())
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

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

    for _ in 0..ENEMY_COUNT {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Enemy {},
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
