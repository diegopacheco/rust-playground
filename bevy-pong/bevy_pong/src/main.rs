use bevy::{
    app::AppExit,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowResolution},
    asset::AssetMode,
};
use rand::Rng;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SIZE: f32 = 30.0;
const BALL_SPEED: f32 = 400.0;
const WALL_THICKNESS: f32 = 10.0;

// Game components
#[derive(Component)]
struct Paddle {
    speed: f32,
    is_left: bool,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct ScoreText {
    is_left: bool,
}

#[derive(Resource)]
struct Score {
    left: u32,
    right: u32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .insert_resource(Score { left: 0, right: 0 })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".to_string(),
                    processed_file_path: "assets_processed".to_string(),
                    watch_for_changes_override: Some(true),
                    mode: AssetMode::Unprocessed,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Pong".into(),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                paddle_movement,
                ball_movement,
                check_collisions,
                update_score_text,
                exit_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddles
    // Left paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))
                .into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 50.0, 0.0, 0.0)),
            ..default()
        },
        Paddle {
            speed: PADDLE_SPEED,
            is_left: true,
        },
        Collider,
    ));

    // Right paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))
                .into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 - 50.0, 0.0, 0.0)),
            ..default()
        },
        Paddle {
            speed: PADDLE_SPEED,
            is_left: false,
        },
        Collider,
    ));

    // Ball
    spawn_ball(&mut commands, &mut materials, &mut meshes);

    // Walls
    // Top wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                WINDOW_HEIGHT / 2.0 - WALL_THICKNESS / 2.0,
                0.0,
            )),
            ..default()
        },
        Collider,
    ));

    // Bottom wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -WINDOW_HEIGHT / 2.0 + WALL_THICKNESS / 2.0,
                0.0,
            )),
            ..default()
        },
        Collider,
    ));

    // Middle line
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            custom_size: Some(Vec2::new(2.0, WINDOW_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    // Score text
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Left score
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "0",
            TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(WINDOW_WIDTH / 4.0),
            top: Val::Px(20.0),
            ..default()
        }),
        ScoreText { is_left: true },
    ));

    // Right score
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "0",
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(3.0 * WINDOW_WIDTH / 4.0),
            top: Val::Px(20.0),
            ..default()
        }),
        ScoreText { is_left: false },
    ));
}

fn paddle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Paddle)>,
) {
    for (mut transform, paddle) in query.iter_mut() {
        let mut direction = 0.0;

        if paddle.is_left {
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= 1.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                direction -= 1.0;
            }
        }

        let new_y = transform.translation.y + direction * paddle.speed * time.delta_seconds();
        let limit = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0 - WALL_THICKNESS;

        transform.translation.y = new_y.clamp(-limit, limit);
    }
}

fn ball_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Ball)>) {
    for (mut transform, ball) in query.iter_mut() {
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();
    }
}

fn exit_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn check_collisions(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Ball, &Transform)>,
    collider_query: Query<(&Transform, Option<&Paddle>), With<Collider>>,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if ball_query.is_empty() {
        info!("No ball found in the world, spawning a new one");
        spawn_ball(&mut commands, &mut materials, &mut meshes);
        return;
    }

    for (ball_entity, mut ball, ball_transform) in ball_query.iter_mut() {
        let ball_pos = ball_transform.translation;

        // Check if ball is outside the screen (someone scored)
        if ball_pos.x < -WINDOW_WIDTH / 2.0 {
            // Right player scored
            score.right += 1;
            // Reset ball
            commands.entity(ball_entity).despawn();
            spawn_ball(&mut commands, &mut materials, &mut meshes);
            return;
        } else if ball_pos.x > WINDOW_WIDTH / 2.0 {
            // Left player scored
            score.left += 1;
            // Reset ball
            commands.entity(ball_entity).despawn();
            spawn_ball(&mut commands, &mut materials, &mut meshes);
            return;
        }

        // Check collision with paddles and walls
        for (collider_transform, maybe_paddle) in collider_query.iter() {
            let collider_pos = collider_transform.translation;

            // Wall collision (top/bottom)
            if maybe_paddle.is_none() {
                let collision = collides_with_wall(
                    ball_pos.x,
                    ball_pos.y,
                    BALL_SIZE / 2.0,
                    collider_pos.x,
                    collider_pos.y,
                    WINDOW_WIDTH / 2.0,
                    WALL_THICKNESS / 2.0,
                );

                if collision {
                    ball.velocity.y = -ball.velocity.y;
                }
            }
            // Paddle collision
            else {
                let paddle = maybe_paddle.unwrap();

                let collision = collides_with_paddle(
                    ball_pos.x,
                    ball_pos.y,
                    BALL_SIZE / 2.0,
                    collider_pos.x,
                    collider_pos.y,
                    PADDLE_WIDTH / 2.0,
                    PADDLE_HEIGHT / 2.0,
                );

                if collision {
                    // Bounce based on where the ball hit the paddle
                    let hit_pos = (ball_pos.y - collider_pos.y) / (PADDLE_HEIGHT / 2.0);
                    let bounce_angle = hit_pos * std::f32::consts::PI / 4.0; // Max 45 degree angle

                    let speed = ball.velocity.length();
                    let direction = if paddle.is_left { 1.0 } else { -1.0 };

                    ball.velocity.x = direction * bounce_angle.cos() * speed;
                    ball.velocity.y = bounce_angle.sin() * speed;

                    // Slightly increase speed after paddle hit
                    ball.velocity *= 1.05;
                }
            }
        }
    }
}

fn update_score_text(score: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    if score.is_changed() {
        for (mut text, score_text) in query.iter_mut() {
            text.sections[0].value = if score_text.is_left {
                score.left.to_string()
            } else {
                score.right.to_string()
            };
        }
    }
}

// Utility function to check if the ball collides with a wall
fn collides_with_wall(
    ball_x: f32,
    ball_y: f32,
    ball_radius: f32,
    wall_x: f32,
    wall_y: f32,
    wall_width: f32,
    wall_height: f32,
) -> bool {
    let wall_left = wall_x - wall_width;
    let wall_right = wall_x + wall_width;
    let wall_top = wall_y + wall_height;
    let wall_bottom = wall_y - wall_height;

    let closest_x = ball_x.max(wall_left).min(wall_right);
    let closest_y = ball_y.max(wall_bottom).min(wall_top);

    let distance_x = ball_x - closest_x;
    let distance_y = ball_y - closest_y;

    distance_x * distance_x + distance_y * distance_y < ball_radius * ball_radius
}

// Utility function to check if the ball collides with a paddle
fn collides_with_paddle(
    ball_x: f32,
    ball_y: f32,
    ball_radius: f32,
    paddle_x: f32,
    paddle_y: f32,
    paddle_width: f32,
    paddle_height: f32,
) -> bool {
    let paddle_left = paddle_x - paddle_width;
    let paddle_right = paddle_x + paddle_width;
    let paddle_top = paddle_y + paddle_height;
    let paddle_bottom = paddle_y - paddle_height;

    let closest_x = ball_x.max(paddle_left).min(paddle_right);
    let closest_y = ball_y.max(paddle_bottom).min(paddle_top);

    let distance_x = ball_x - closest_x;
    let distance_y = ball_y - closest_y;

    distance_x * distance_x + distance_y * distance_y < ball_radius * ball_radius
}

// Function to spawn a new ball
fn spawn_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let mut rng = rand::thread_rng();

    // Use a bright yellow color for better visibility
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE)).into(),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0)), // Bright yellow
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Ball {
            velocity: Vec2::new(
                if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                },
                if rng.gen_bool(0.5) {
                    BALL_SPEED * 0.5
                } else {
                    -BALL_SPEED * 0.5
                },
            ),
        },
    ));
}
