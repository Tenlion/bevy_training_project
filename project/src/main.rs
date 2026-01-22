
use bevy::prelude::*;
fn main() {

    App::new()
        .add_systems(Startup, (spawn_monsters, print_monster_info).chain())
        .run();
}

pub fn spawn_monsters(mut commands: Commands) {

    commands.spawn(Monster {
        name: String::from("Xyor'dox"),
        health: 10_000
    });

    commands.spawn(Monster {
        name: String::from("Ghlorm"),
        health: 5_000
    });

    commands.spawn(Monster {
        name: String::from("Destroth"),
        health: 4_444
    });
}

pub fn print_monster_info(monster_query: Query<&Monster>) {

    for monster in monster_query.iter() {
        println!("Monster Name: {}", monster.name);
        println!("Monster Health: {}", monster.health);
        println!();
    }
}

#[derive(Component)]
pub struct Monster {
    pub name: String,
    pub health:  u16
}
