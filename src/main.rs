use bevy::prelude::*;

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const BALL_SIZE: f32 = 10.0;

struct Paddle {
    velocity: Vec2,
}

struct Ball {
    velocity: Vec2,
}

fn setup(mut commands: Commands) {
    // Spawn paddles
    commands
        .spawn(SpriteBundle {
            material: Color::WHITE.into(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            translation: Transform2d::from_translation(Vec2::new(-400.0, 0.0)),
            ..Default::default()
        })
        .insert(Paddle {
            velocity: Vec2::ZERO,
        });

    commands
        .spawn(SpriteBundle {
            material: Color::WHITE.into(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            translation: Transform2d::from_translation(Vec2::new(400.0, 0.0)),
            ..Default::default()
        })
        .insert(Paddle {
            velocity: Vec2::ZERO,
        });

    // Spawn ball
    commands
        .spawn(SpriteBundle {
            material: Color::WHITE.into(),
            sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
            translation: Transform2d::from_translation(Vec2::new(0.0, 0.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec2::new(300.0, 150.0, 0.0),
        });
}

fn paddle_movement(keyboard_input: Res<KeyCodes>, mut query: Query<(&mut Paddle, &Transform2d)>) {
    for (mut paddle, transform) in query.iter_mut() {
        let mut movement = Vec2::ZERO;
        if keyboard_input.pressed(KeyCodes::A) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCodes::D) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCodes::W) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCodes::S) {
            movement.x += 1.0;
        }
        if !movement.is_zero() {
            paddle.velocity = movement.normalize() * PADDLE_SPEED;
        } else {
            paddle.velocity = Vec2::ZERO;
        }

        transform.translation.x = transform.translation.x.min(390.0).max(-390.0);
        transform.translation.y = transform.translation.y.min(290.0).max(-290.0);
    }
}

fn ball_movement(mut query: Query<(&mut Ball, &Transform2d)>) {
    for (mut ball, transform) in query.iter_mut() {
        transform.translation += ball.velocity * 0.02;

        // Reverse ball's direction when hitting the top or bottom of the window
        if transform.translation.y >= 290.0 || transform.translation.y <= -290.0 {
            ball.velocity.y *= -1.0;
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(paddle_movement.system_2d())
        .add_system(ball_movement.system_2d())
        .run()
}
