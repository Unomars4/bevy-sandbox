use bevy::{math::f32, prelude::*, sprite::Anchor, text::cosmic_text::ttf_parser::gpos::Anchor};

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(f32);

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        Sprite {
            color: Color::Srgba(0.5, 1.0, 0.5),
            custom_size: Some(Vec2::new(30.0, 50.0)),
            anchor: Anchor::BottomCenter,
            ..Default::default()
        },
    ));
}

fn main() {
    println!("Commence Endgame 🚀");
    App::new().add_plugins(DefaultPlugins).run();
}
