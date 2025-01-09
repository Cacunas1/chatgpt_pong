use bevy::{input::keyboard::KeyCode, prelude::*};

const X_EXTENT: f32 = 900.0;
const Y_EXTENT: f32 = 900.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const BALL_SIZE: f32 = 10.0;

#[derive(Component, Clone, Copy)]
struct Paddle {
    velocity: Vec3,
}

#[derive(Component, Clone, Copy)]
struct Ball {
    velocity: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let l_padd = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let r_padd = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let ball = Rectangle::new(BALL_SIZE, BALL_SIZE);

    // let mut shapes = [meshes.add(l_padd), meshes.add(r_padd), meshes.add(ball)];
    let mut x: f32 = -0.5 * X_EXTENT;
    let y: f32 = 0.0;
    let z: f32 = 0.0;

    commands
        .spawn((
            Mesh2d(meshes.add(l_padd)),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::from_xyz(x, y, z),
        ))
        .insert(Paddle {
            velocity: Vec3::ZERO,
        });

    x = 0.5 * X_EXTENT;

    commands
        .spawn((
            Mesh2d(meshes.add(r_padd)),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
            Transform::from_xyz(x, y, z),
        ))
        .insert(Paddle {
            velocity: Vec3::ZERO,
        });

    x = 0.0;

    commands
        .spawn((
            Mesh2d(meshes.add(ball)),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(x, y, z),
        ))
        .insert(Ball {
            velocity: Vec3::new(300.0, 150.0, 0.0),
        });
}

fn paddle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Paddle, &mut Transform)>,
) {
    for (mut paddle, mut transform) in query.iter_mut() {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.x += 1.0;
        }
        if movement != Vec3::ZERO {
            paddle.velocity = movement.normalize() * PADDLE_SPEED;
        } else {
            paddle.velocity = Vec3::ZERO;
        }

        transform.translation.x = transform.translation.x.min(390.0).max(-390.0);
        transform.translation.y = transform.translation.y.min(290.0).max(-290.0);
    }
}

fn ball_movement(mut query: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * 0.02;

        // Reverse ball's direction when hitting the top or bottom of the window
        if transform.translation.y >= 290.0 || transform.translation.y <= -290.0 {
            ball.velocity.y *= -1.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, paddle_movement)
        .add_systems(Update, ball_movement)
        .run();
}
