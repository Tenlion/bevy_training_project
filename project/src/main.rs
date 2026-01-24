
use bevy::prelude::*;
fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MonsterPlugin)
        .run();
}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_monsters, print_monster_info, print_monsters_with_type).chain());
    }
}

pub fn spawn_monsters(mut commands: Commands) {

    commands.spawn((
        Monster {
            name: String::from("Xyor'dox"),
            health: 10_000
        },
        MonsterType {
            monster_type: MonsterLibrary::VoidCreature
        }
    ));

    commands.spawn((
        Monster {
            name: String::from("Ghlorm"),
            health: 5_000
        },
        MonsterType {
            monster_type: MonsterLibrary::GiantBug
        }
    ));

    commands.spawn((
        Monster {
            name: String::from("Destroth"),
            health: 4_444
        },
        MonsterType {
            monster_type: MonsterLibrary::Abomination
        }
    ));
}

pub fn print_monster_info(monster_query: Query<&Monster>, monster_type_query: Query<&MonsterType>) {

    let monsters: Vec<&Monster> = monster_query.iter().collect();
    let monster_types: Vec<&MonsterType> = monster_type_query.iter().collect();
    for monster in 0..monsters.len() {
        println!("Monster Name: {}", monsters[monster].name);
        println!("Monster Health: {}", monsters[monster].health);
        println!("{} is a {:?}.", monsters[monster].name, monster_types[monster].monster_type);
        println!();
    }
}

pub fn print_monsters_with_type(bug_query: Query<&Monster, With<MonsterType>>) {

    let mut count: u8 = 1;
    for monster in bug_query.iter() {
        println!("Monster With a Type {}: {}", count, monster.name);
        count += 1;
    }
}

#[derive(Component)]
pub struct Monster {
    pub name: String,
    pub health:  u16
}

#[derive(Component)]
pub struct MonsterType {
    pub monster_type: MonsterLibrary
}

#[derive(Debug)]
pub enum MonsterLibrary {
    VoidCreature,
    Abomination,
    GiantBug
}
