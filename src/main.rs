use bevy::{input::keyboard::KeyCode, prelude::*};

const X_EXTENT: f32 = 900.0;
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

    let shapes = [
        meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    ];

    let color = Color::WHITE;
    let n = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        let mut x: f32 = 0.0;
        let y: f32 = 0.0;
        let z: f32 = 0.0;
        println!("i: {i}");
        println!("n: {n}");
        println!("shape: {:?}", shape);
        if i < n - 1 {
            x = f32::powf(-1 as f32, i as f32) * X_EXTENT * 0.5;
            let res_type = Ball {
                velocity: Vec3::new(300.0, 150.0, 0.0),
            };
            commands
                .spawn((
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(x, y, z),
                ))
                .insert(res_type);
        } else {
            let res_type = Paddle {
                velocity: Vec3::ZERO,
            };
            commands
                .spawn((
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(x, y, z),
                ))
                .insert(res_type);
        }
    }

    // Spawn paddles
    // commands
    //     .spawn(Sprite {
    //         material: Color::WHITE.into(),
    //         sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
    //         transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
    //         ..Default::default()
    //     })
    //     .insert(Paddle {
    //         velocity: Vec3::ZERO,
    //     });

    // commands
    //     .spawn(Sprite {
    //         material: Color::WHITE.into(),
    //         sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
    //         transform: Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
    //         ..Default::default()
    //     })
    //     .insert(Paddle {
    //         velocity: Vec3::ZERO,
    //     });

    // // Spawn ball
    // commands
    //     .spawn(Sprite {
    //         material: Color::WHITE.into(),
    //         sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
    //         transform: Transform::from_translation(Vec3::ZERO),
    //         ..Default::default()
    //     })
    //     .insert(Ball {
    //         velocity: Vec3::new(300.0, 150.0, 0.0),
    //     });
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
