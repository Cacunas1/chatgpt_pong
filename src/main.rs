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

#[derive(Component, Debug)]
struct Ball {
    velocity: Vec2,
}

// Movement speed constant
const SPEED: f32 = 300.0;
// Rectangle size
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 100.0);
// Rectangle size
const BALL_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, ball_movement))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    // Ball
    let ball = Circle::new(BALL_SIZE);
    commands
        .spawn((
            Mesh2d(meshes.add(ball)),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(Ball {
            velocity: Vec2::new(300.0, 150.0),
        });
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

fn ball_movement(
    time: Res<Time>,
    windows: Single<&Window>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    player_query: Query<&Transform, (With<Player>, Without<Ball>)>,
) {
    let win_width = windows.width();
    let win_height = windows.height();

    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        // Move ball
        let translation = &mut ball_transform.translation;
        translation.x += ball.velocity.x * time.delta_secs();
        translation.y += ball.velocity.y * time.delta_secs();

        // Ball collision box
        let ball_left = translation.x - BALL_SIZE;
        let ball_right = translation.x + BALL_SIZE;
        let ball_top = translation.y + BALL_SIZE;
        let ball_bottom = translation.y - BALL_SIZE;

        // Check for collisions with players
        for player_transform in player_query.iter() {
            let player_size = PLAYER_SIZE / 2.0;
            let player_left = player_transform.translation.x - player_size.x;
            let player_right = player_transform.translation.x + player_size.x;
            let player_top = player_transform.translation.y + player_size.y;
            let player_bottom = player_transform.translation.y - player_size.y;

            // Simple AABB collision detection
            if ball_right >= player_left
                && ball_left <= player_right
                && ball_top >= player_bottom
                && ball_bottom <= player_top
            {
                // Reverse x direction and add some randomness to y velocity
                ball.velocity.x = -ball.velocity.x;
                ball.velocity.y +=
                    (translation.y - player_transform.translation.y) / player_size.y * SPEED * 0.5;

                // Normalize velocity to maintain constant speed
                ball.velocity = ball.velocity.normalize() * SPEED;
            }
        }

        // Bounce off window borders
        if ball_right >= win_width / 2.0 || ball_left <= -win_width / 2.0 {
            // Reset ball to center if it hits left or right walls
            translation.x = 0.0;
            translation.y = 0.0;
            ball.velocity = Vec2::new(1.0, 0.5).normalize() * SPEED;
        }

        if ball_top >= win_height / 2.0 || ball_bottom <= -win_height / 2.0 {
            ball.velocity.y = -ball.velocity.y;
        }
    }
}
