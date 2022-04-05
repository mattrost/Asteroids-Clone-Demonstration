use std::f64::consts::PI;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseScrollUnit::Pixel;
use rand::random;

const SHIP_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const MAX_SPEED: f64 = 100.;

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
impl Velocity {
    fn xy_velocity(&self) -> f64 {
        let squares: f64 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;
        let xy_vel: f64 = squares.sqrt();
        xy_vel
    }
    fn accelerate(&mut self, direction: f64) {
        self.x_vel = self.x_vel + direction.cos();
        self.y_vel = self.y_vel + direction.sin();
        let squares: f64 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;

        if squares > MAX_SPEED*MAX_SPEED {
            let current_speed: f64 = squares.sqrt();
            self.x_vel *= (MAX_SPEED / current_speed);
            self.y_vel *= (MAX_SPEED / current_speed);
        }
    }
    fn print_speed(&self) {
        let squares: f64 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;
        let max_speed: f64 = squares.sqrt();
        println!("max speed = {}", max_speed);
    }
}

#[derive(Component)]
struct Direction {
    angle: f64,
}
impl Direction {
    fn rotate_right(&mut self) {
        self.angle += 0.01;
        if self.angle >= 2.*PI {
            self.angle -= 2.*PI
        }
    }
    fn rotate_left(&mut self) {
        self.angle -= 0.01;
        if self.angle < 0. {
            self.angle += 2.*PI
        }
    }
}

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
        direction: Direction { angle: PI/2. },
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SHIP_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(20.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player.health)
        .insert(player.position)
        .insert(player.velocity)
        .insert(player.direction);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Direction)>
) {
    for (mut velocity, mut direction) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            direction.rotate_left();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.rotate_right();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.accelerate(direction.angle);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.accelerate(direction.angle);
        }
    }
    for (velocity) in query.iter() {
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
