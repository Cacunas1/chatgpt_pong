use bevy::{input::keyboard::KeyCode, prelude::*};

enum Side {
    L,
    R,
}

#[derive(Component)]
struct Player {
    p: Side,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    last_collision: Option<Entity>, // Track last collision to prevent multiple hits
}

const SPEED: f32 = 300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, 150.0);
const BALL_SIZE: f32 = 12.5;
const MIN_BALL_SPEED: f32 = 200.0;
const MAX_BALL_SPEED: f32 = 400.0;

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
    window: Single<&Window>,
) {
    commands.spawn(Camera2d);
    let win_width = window.width();
    let xr = 0.5 * win_width - PLAYER_SIZE.x;
    let xl = -xr;

    let green = Color::srgb(0.0, 1.0, 0.0);
    let red = Color::srgb(1.0, 0.0, 0.0);

    // Left paddle
    commands.spawn((
        Sprite::from_color(green, PLAYER_SIZE),
        Transform::from_xyz(xl, 0.0, 0.0),
        Player { p: Side::L },
    ));

    // Right paddle
    commands.spawn((
        Sprite::from_color(red, PLAYER_SIZE),
        Transform::from_xyz(xr, 0.0, 0.0),
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
            last_collision: None,
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

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        let new_position =
            player_transform.translation + direction * 1.1 * SPEED * time.delta_secs();

        // Clamp position within window bounds
        let half_size = 0.5 * PLAYER_SIZE;
        player_transform.translation.x = new_position.x.clamp(
            -0.5 * win_width + half_size.x,
            0.5 * win_width - half_size.x,
        );
        player_transform.translation.y = new_position.y.clamp(
            -0.5 * win_height + half_size.y,
            0.5 * win_height - half_size.y,
        );
    }
}

fn ball_movement(
    time: Res<Time>,
    windows: Single<&Window>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Ball>)>,
) {
    let win_width = windows.width();
    let win_height = windows.height();

    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        let delta = time.delta_secs();

        // Store previous position for collision resolution
        let previous_pos = ball_transform.translation;

        // Move ball
        ball_transform.translation.x += ball.velocity.x * delta;
        ball_transform.translation.y += ball.velocity.y * delta;

        // Ball collision bounds
        let ball_bounds = Rect {
            left: ball_transform.translation.x - BALL_SIZE,
            right: ball_transform.translation.x + BALL_SIZE,
            top: ball_transform.translation.y + BALL_SIZE,
            bottom: ball_transform.translation.y - BALL_SIZE,
        };

        // Check paddle collisions
        for (player_entity, player_transform) in player_query.iter() {
            if ball.last_collision == Some(player_entity) {
                continue; // Skip if we just collided with this paddle
            }

            let player_bounds = Rect {
                left: player_transform.translation.x - PLAYER_SIZE.0 .5 * x,
                right: player_transform.translation.x + PLAYER_SIZE.0 .5 * x,
                top: player_transform.translation.y + PLAYER_SIZE.0 .5 * y,
                bottom: player_transform.translation.y - PLAYER_SIZE.0 .5 * y,
            };

            if check_collision(&ball_bounds, &player_bounds) {
                // Calculate collision response
                let hit_position = (ball_transform.translation.y - player_transform.translation.y)
                    / (PLAYER_SIZE.0 .5 * y);

                // Reverse x direction
                ball.velocity.x = -ball.velocity.x;

                // Add vertical velocity based on where the ball hits the paddle
                ball.velocity.y = hit_position * SPEED * 0.8;

                // Ensure the ball maintains a minimum speed
                let speed = ball.velocity.length();
                if speed < MIN_BALL_SPEED {
                    ball.velocity = ball.velocity.normalize() * MIN_BALL_SPEED;
                } else if speed > MAX_BALL_SPEED {
                    ball.velocity = ball.velocity.normalize() * MAX_BALL_SPEED;
                }

                // Move ball out of collision
                if ball.velocity.x > 0.0 {
                    ball_transform.translation.x = player_bounds.left - BALL_SIZE;
                } else {
                    ball_transform.translation.x = player_bounds.right + BALL_SIZE;
                }

                // Remember this collision
                ball.last_collision = Some(player_entity);
            }
        }

        // Window bounds collision
        if ball_bounds.right >= 0.5 * win_width || ball_bounds.left <= -0.5 * win_width {
            // Reset ball to center
            ball_transform.translation.x = 0.0;
            ball_transform.translation.y = 0.0;
            ball.velocity = Vec2::new(
                if ball_bounds.left <= -0.5 * win_width {
                    1.0
                } else {
                    -1.0
                },
                0.5,
            )
            .normalize()
                * MIN_BALL_SPEED;
            ball.last_collision = None;
        }

        if ball_bounds.top >= 0.5 * win_height {
            ball_transform.translation.y = 0.5 * win_height - BALL_SIZE;
            ball.velocity.y = -ball.velocity.y.abs();
            ball.last_collision = None;
        } else if ball_bounds.bottom <= -0.5 * win_height {
            ball_transform.translation.y = -0.5 * win_height + BALL_SIZE;
            ball.velocity.y = ball.velocity.y.abs();
            ball.last_collision = None;
        }
    }
}

struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

fn check_collision(a: &Rect, b: &Rect) -> bool {
    a.left < b.right && a.right > b.left && a.top > b.bottom && a.bottom < b.top
}
