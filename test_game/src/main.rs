
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_camera).chain())
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player
(
    // Parameters
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    // Return
    -> Result<()>
{
    // Function Code

    // .single() replaced .get_single().  The purpose of .single() is the same as .iter() except
    // that it will return only one instance that the query picks up (most likely the first one?).
    // If there isn't exactly one instance of the thing you're querying, an error will be returned.
    let window: &Window = window_query.single()?;

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/Items_BlackPearl.png")),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Visibility::Visible,
        Player {}
    ));

    Ok(())
}

pub fn spawn_camera
(
    // Parameters
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
)
    // Return
    -> Result<()>
{
    // Function Code

    let window: &Window = window_query.single()?;

    // Creating a camera and placing its position at the center of the window.
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
    ));

    Ok(())
}
