use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::touch::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::Stopwatch;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const SPEED: f32 = 400.0;
pub const GRAVITY: Vec3 = Vec3::new(0.0, -800.0, 0.0);
pub const DAMPENING: f32 = 0.998;
pub const FIXED_TIME: f32 = 1.0 / 600.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                prevent_default_event_handling: false,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_rate_limiter)
        .add_systems(
            (ball_movement, ball_check_border, check_ball_collision)
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(FIXED_TIME))
        .add_system(spawn_ball)
        .add_system(print_fps)
        .add_system(change_ball_color)
        .add_system(move_camera)
        .run();
}

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct SpawnRateLimiter {
    pub time: Stopwatch,
}

pub fn spawn_rate_limiter(mut commands: Commands) {
    commands.spawn(SpawnRateLimiter {
        time: Stopwatch::new(),
    });
}

pub fn spawn_ball(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    touch_input: Res<Touches>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut spawn_rate_q: Query<&mut SpawnRateLimiter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let mut spawn_rate_limiter = spawn_rate_q.get_single_mut().unwrap();
    if spawn_rate_limiter.time.tick(time.delta()).elapsed_secs() < 0.2 {
        return;
    }

    // mouse input logic
    if let Some(mouse_position) = window.cursor_position() {
        if mouse_input.just_released(MouseButton::Left) || mouse_input.pressed(MouseButton::Right) {
            println!(
                "Cursor clicked inside the primary window, at {:?}",
                mouse_position
            );
            spawn_ball_function(
                &mouse_position,
                &mut commands,
                &mut spawn_rate_limiter,
                &mut meshes,
                &mut materials,
            )
        }
    }
    // touch input logic

    for touch in touch_input.iter() {
        info!("Touch {:?}", touch);
        spawn_ball_function(
            &touch.position(),
            &mut commands,
            &mut spawn_rate_limiter,
            &mut meshes,
            &mut materials,
        )
    }
}

fn spawn_ball_function(
    // This function is to seperate the actually ball spawning from the system that actually decided where/when to spawn a ball
    position: &Vec2,
    commands: &mut Commands,
    spawn_rate_limiter: &mut Mut<'_, SpawnRateLimiter>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    spawn_rate_limiter.time.reset();

    let rand_radius: f32 = rand::thread_rng().gen_range(5.0..=20.0);
    let rand_velcoity: Vec3 = Vec3::new(
        rand::thread_rng().gen_range(-40.0..=40.0),
        rand::thread_rng().gen_range(-40.0..=40.0),
        0.0,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(rand_radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
        Ball {
            radius: rand_radius,
            velocity: rand_velcoity,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });
    info!("Window info {:?}", window)
}

pub fn move_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>,window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    camera_transform.translation = Vec3::new((window.width() as f32)/2.0, (window.height() as f32)/2.0, 0.0)
}

pub fn ball_movement(mut ball_query: Query<(&mut Transform, &mut Ball)>) {
    for (mut transform, mut ball) in ball_query.iter_mut() {
        let current_velocity = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0);
        transform.translation +=
            current_velocity * FIXED_TIME + (0.5 * GRAVITY * FIXED_TIME * FIXED_TIME);
        ball.velocity += GRAVITY * FIXED_TIME;
    }
}

pub fn ball_check_border(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut ball) in ball_query.iter_mut() {
        let radius = ball.radius;
        let x_min = 0.0 + radius;
        let x_max = window.width() - radius;
        let y_min = 0.0 + radius;
        let y_max = window.height() - radius;

        let position = transform.translation;

        if position.x < x_min {
            ball.velocity.x *= -1.0;
            transform.translation.x = x_min;
        } else if position.x > x_max {
            ball.velocity.x *= -1.0;
            transform.translation.x = x_max;
        }
        if position.y < y_min {
            ball.velocity.y *= -1.0;
            transform.translation.y = y_min;
        } else if position.y > y_max {
            ball.velocity.y *= -1.0;
            transform.translation.y = y_max;
        }
    }
}

pub fn print_fps(time: Res<Time>, key_input: Res<Input<KeyCode>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if key_input.pressed(KeyCode::Backslash) {
        info!("Approx fps = {}", 1.0 / (time.delta_seconds()));
    }

    if key_input.pressed(KeyCode::Grave) {
        let window = window_query.get_single().unwrap();
        info!("Window info : {:?}", window)
    }
}

pub fn check_ball_collision(mut ball_query: Query<(Entity, &mut Transform, &mut Ball)>) {
    let mut translation_map: HashMap<Entity, Vec3> = HashMap::new();
    let mut velocity_map: HashMap<Entity, Vec3> = HashMap::new();

    for (entity_1, transform_1, ball_1) in ball_query.iter() {
        let mut cummulative_translation = Vec3::new(0.0, 0.0, 0.0);
        let mut cummulative_velocity = Vec3::new(0.0, 0.0, 0.0);

        for (entity_2, transform_2, ball_2) in ball_query.iter() {
            if entity_1 == entity_2 {
                continue;
            };

            let distance = transform_1.translation.distance(transform_2.translation);

            if distance > (ball_1.radius + ball_2.radius) {
                continue;
            }

            let velocity_diff = ball_1.velocity - ball_2.velocity;
            let position_diff = transform_1.translation - transform_2.translation;

            if position_diff.length() == 0.0 {
                continue;
            }

            // we sum up the total translation and velocity change from all collisions
            cummulative_translation -= 0.4
                * (position_diff.length() - (ball_1.radius + ball_2.radius))
                * position_diff.normalize();
            // transform_2.translation += 0.5*(position_diff.length() - (ball_1.radius + ball_2.radius))*position_diff.normalize();
            cummulative_velocity -= ((velocity_diff).dot(position_diff)
                / (position_diff.length_squared()))
                * position_diff;
            // ball_2.velocity -= ((-velocity_diff).dot(-position_diff) / (-position_diff.length_squared())) * -position_diff;
        }
        // insert cumulated collisions into hashmap to apply mutably later
        translation_map.insert(entity_1, cummulative_translation);
        velocity_map.insert(entity_1, cummulative_velocity);
    }

    for (entity, mut transform, mut ball) in ball_query.iter_mut() {
        ball.velocity += DAMPENING * (*velocity_map.get(&entity).unwrap());
        transform.translation += *translation_map.get(&entity).unwrap();
    }
}

pub fn change_ball_color(
    ball_material_query: Query<(&Ball, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ball, color_handle) in ball_material_query.iter() {
        let color = &mut materials.get_mut(color_handle).unwrap().color;
        let new_color = Color::hsla((ball.velocity.length() / 9.0) % 360.0, 0.9, 0.6, 1.0);
        color.set_r(new_color.r());
        color.set_g(new_color.g());
        color.set_b(new_color.b());
        color.set_a(new_color.a());
    }
}
