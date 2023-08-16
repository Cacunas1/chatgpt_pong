use bevy::window::PrimaryWindow;
use bevy::{input::keyboard::KeyCode, prelude::*};
use cond_utils::Between;

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
const PADDLE_HALF_HEIGHT: f32 = 0.5 * PADDLE_HEIGHT;
const BALL_SPEED: f32 = 250.0;
const BALL_SIZE: f32 = PADDLE_WIDTH;
const BALL_HALF_SIZE: f32 = 0.5 * BALL_SIZE;

#[derive(Component)]
pub struct Paddle {
    id: u8,
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec3,
    bounce: Vec<bool>,
    score: Vec<u8>,
}

#[derive(Component)]
pub struct MyMusic {
    name: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create camera
    commands.spawn(Camera2dBundle::default());
    // Create paddle sprite
    let paddle_sprite = Sprite {
        color: Color::rgb(1.0, 1.0, 1.0),
        custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        ..default()
    };
    // Spawn paddles
    commands.spawn((
        SpriteBundle {
            sprite: paddle_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
            ..default()
        },
        Paddle { id: 0 },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: paddle_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
            ..default()
        },
        Paddle { id: 1 },
    ));

    // Create ball sprite
    let ball_sprite = Sprite {
        color: Color::rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
        ..default()
    };
    // Spawn ball
    commands.spawn((
        SpriteBundle {
            sprite: ball_sprite,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Ball {
            velocity: Vec3::new(300.0, 150.0, 0.0),
            bounce: vec![true, true],
            score: vec![0, 0],
        },
    ));
    // Spawn sound
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/explosionCrunch_000.ogg"),
            ..default()
        },
        MyMusic {
            name: String::from("point"),
        },
    ));
}

fn paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>,
    time: Res<Time>,
) {
    for (mut transform, paddle) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if paddle.id == 0 {
            if keyboard_input.pressed(KeyCode::A) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::W) {
                direction.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction.y -= 1.0;
            }
        }
        if paddle.id == 1 {
            if keyboard_input.pressed(KeyCode::Left) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                direction.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                direction.y -= 1.0;
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PADDLE_SPEED * time.delta_seconds();
    }
}

fn ball_movement(
    mut query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query_music: Query<(&AudioSink, &MyMusic)>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = -0.5 * window.width() + BALL_HALF_SIZE;
    let x_max = 0.5 * window.width() - BALL_HALF_SIZE;
    let y_min = -0.5 * window.height() + BALL_HALF_SIZE;
    let y_max = 0.5 * window.height() - BALL_HALF_SIZE;

    for (mut transform, mut ball) in query.iter_mut() {
        let mut v = ball.velocity;
        if !v.is_normalized() {
            v = v.normalize();
        }
        transform.translation += v * BALL_SPEED * time.delta_seconds();
        ball.velocity = v;

        // Reverse ball's direction when hitting the top or bottom of the window
        if transform.translation.y >= y_max || transform.translation.y <= y_min {
            ball.velocity.y *= -1.0;
        }
        if transform.translation.x > x_max || transform.translation.x < x_min {
            transform.translation = Vec3::ZERO;
            ball.velocity *= -1.0;
            ball.bounce = vec![true, true];

            if transform.translation.x >= x_max {
                ball.score[1] += 1;
            } else if transform.translation.x <= x_min {
                ball.score[0] += 1;
            }

            if let Ok(sink) = query_music.get_single() {
                if sink.1.name == "point" {
                    sink.0.play();
                }
            }
        }
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&Transform, &mut Ball), With<Ball>>,
    paddle_query: Query<(&Transform, &Paddle), With<Paddle>>,
) {
    if let Ok((b_transform, mut ball)) = ball_query.get_single_mut() {
        for (p_transform, paddle) in paddle_query.iter() {
            let i = usize::from(paddle.id);
            let dist = b_transform.translation.distance(p_transform.translation);

            if dist <= 350.0 {
                let b_x = b_transform.translation.x;
                let b_y = b_transform.translation.y;
                let p_x = p_transform.translation.x;
                let p_y = p_transform.translation.y;

                let ball_in_range_x = b_x.within(
                    p_x - (PADDLE_HALF_WIDTH + BALL_HALF_SIZE),
                    p_x + (PADDLE_HALF_WIDTH + BALL_HALF_SIZE),
                );
                let ball_in_range_y = b_y.within(
                    p_y - (PADDLE_HALF_HEIGHT + BALL_HALF_SIZE),
                    p_y + (PADDLE_HALF_HEIGHT + BALL_HALF_SIZE),
                );

                if ball_in_range_x && ball_in_range_y && ball.bounce[i] {
                    ball.bounce[i] = false;
                    ball.bounce[(i + 1) % 2] = true;
                    ball.velocity.x *= -1.0;
                }
            }
        }
    }
}

pub fn confine_paddle_movement(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = -0.5 * window.width() + PADDLE_HALF_WIDTH;
    let x_max = 0.5 * window.width() - PADDLE_HALF_WIDTH;
    let y_min = -0.5 * window.height() + PADDLE_HALF_HEIGHT;
    let y_max = 0.5 * window.height() - PADDLE_HALF_HEIGHT;

    for mut paddle_transform in paddle_query.iter_mut() {
        let mut translation = paddle_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        paddle_transform.translation = translation;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, paddle_movement)
        .add_systems(Update, ball_movement)
        .add_systems(Update, ball_paddle_collision)
        .add_systems(Update, confine_paddle_movement)
        .run();
}
