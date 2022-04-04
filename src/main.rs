use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use rand::random;

const SHIP_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
struct Health(u64);

#[derive(Component)]
struct ShipPosition {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct Velocity {
    x_vel: f64,
    y_vel: f64,
}

#[derive(Component)]
struct Direction(i64);

#[derive(Bundle)]
struct ShipBundle {
    health: Health,
    position: ShipPosition,
    velocity: Velocity,
    direction: Direction,
}

fn spawn_player(mut commands: Commands) {
    let player = ShipBundle {
        health: Health(10),
        position: ShipPosition { x: 10., y: 10. },
        velocity: Velocity { x_vel: 0.0, y_vel: 0.0 },
        direction: Direction(0),
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SHIP_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player.health)
        .insert(player.position)
        .insert(player.velocity)
        .insert(player.direction);
}

fn accelerate(mut query: Query<&mut Velocity>,
) {
    for mut velocity in query.iter_mut() {
        if velocity.x_vel < 500. {
            velocity.x_vel = velocity.x_vel + 1.0;
        }
        println!("velocity: {:}", velocity.x_vel);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_control: Query<&mut Transform, With<ShipPosition>>,
) {
    for mut transform in ship_control.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(accelerate)
        .add_plugins(DefaultPlugins)
        .run();
}