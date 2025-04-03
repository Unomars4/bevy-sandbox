use bevy::{math::f32, prelude::*};

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(f32);

fn main() {
    println!("Commence Endgame 🚀");
    App::new().add_plugins(DefaultPlugins).run();
}
