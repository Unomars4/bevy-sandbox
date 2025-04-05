use bevy::{input::keyboard::KeyboardInput, prelude::*, sprite::Anchor};

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;
const JUMP_FORCE: f32 = 600.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 1.0, 0.5),
            custom_size: Some(Vec2::new(30.0, 50.0)),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(800.0, 10.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-400.0, GROUND_LEVEL, 0.0),
    ));
}

fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    for e in events.read() {
        if let Ok((mut velocity, transfrom)) = query.get_single_mut() {
            if e.state.is_pressed()
                && e.key_code == KeyCode::Space
                && transfrom.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

fn main() {
    println!("Commence Endgame 🚀");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (jump, player_movement))
        .run();
}
