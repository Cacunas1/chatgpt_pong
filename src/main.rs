use bevy::{input::keyboard::KeyCode, prelude::*};

// Side of the player's paddle
enum Side {
    L,
    R,
}
// Component to mark our player rectangle
#[derive(Component)]
struct Player {
    p: Side,
}

// Movement speed constant
const SPEED: f32 = 300.0;
// Rectangle size
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 100.0);
// Window size
// const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // colors
    let green = Color::srgb(0.0, 1.0, 0.0);
    let red = Color::srgb(1.0, 0.0, 0.0);
    // Player rectangle
    commands.spawn((
        Sprite::from_color(green, PLAYER_SIZE),
        Transform::from_xyz(-150.0, 0.0, 0.0),
        Player { p: Side::L },
    ));
    // Player rectangle
    commands.spawn((
        Sprite::from_color(red, PLAYER_SIZE),
        Transform::from_xyz(150.0, 0.0, 0.0),
        Player { p: Side::R },
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    windows: Single<&Window>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    let win_width = windows.width();
    let win_height = windows.height();

    for (mut player_transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        match player.p {
            Side::L => {
                // Get input direction
                if keyboard.pressed(KeyCode::KeyW) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    direction.x += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyA) {
                    direction.x -= 1.0;
                }
            }
            Side::R => {
                // Get input direction
                if keyboard.pressed(KeyCode::KeyI) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyK) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(KeyCode::KeyL) {
                    direction.x += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyJ) {
                    direction.x -= 1.0;
                }
            }
        };

        // Normalize direction to prevent faster diagonal movement
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        // Calculate new position
        let new_position = player_transform.translation + direction * SPEED * time.delta_secs();

        // Clamp position within window bounds
        let half_size = PLAYER_SIZE / 2.0;
        player_transform.translation.x = new_position.x.clamp(
            -win_width / 2.0 + half_size.x,
            win_width / 2.0 - half_size.x,
        );
        player_transform.translation.y = new_position.y.clamp(
            -win_height / 2.0 + half_size.y,
            win_height / 2.0 - half_size.y,
        );
    }
}
