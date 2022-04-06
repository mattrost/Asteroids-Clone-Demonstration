use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::event::Events;
use bevy::window::WindowResized;

const WINDOW_X: f32 = 1280.;
const WINDOW_Y: f32 = 720.;

const SHIP_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
const MAX_SPEED: f32 = 100.;
const SPEED_FACTOR: f32 = 0.05;
const TURN_FACTOR: f32 = 3.5;


#[derive(Component)]
struct Health(u64);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x_vel: f32,
    y_vel: f32,
}
impl Velocity {
    fn xy_velocity(&self) -> f32 {
        let squares: f32 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;
        let xy_vel: f32 = squares.sqrt();
        xy_vel
    }
    fn accelerate(&mut self, direction: f32) {
        self.x_vel = self.x_vel + direction.cos();
        self.y_vel = self.y_vel + direction.sin();
        let squares: f32 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;

        if squares > MAX_SPEED*MAX_SPEED {
            let current_speed: f32 = squares.sqrt();
            self.x_vel *= MAX_SPEED / current_speed;
            self.y_vel *= MAX_SPEED / current_speed;
        }
    }
    fn print_speed(&self) {
        let squares: f32 = self.x_vel*self.x_vel + self.y_vel*self.y_vel;
        let max_speed: f32 = squares.sqrt();
        println!("max speed = {}", max_speed);
    }
}

#[derive(Component)]
struct Direction {
    angle: f32,
}
impl Direction {
    fn rotate_right(&mut self) {
        self.angle -= TURN_FACTOR*0.01;
        if self.angle >= 2.*PI {
            self.angle -= 2.*PI
        }
    }
    fn rotate_left(&mut self) {
        self.angle += TURN_FACTOR*0.01;
        if self.angle < 0. {
            self.angle += 2.*PI
        }
    }
}

#[derive(Bundle)]
struct PlayerShipBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
    direction: Direction,
}

fn spawn_player(mut commands: Commands) {
    let player = PlayerShipBundle {
        health: Health(10),
        position: Position { x: 0., y: 0. },
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
                scale: Vec3::new(30.0, 30.0, 10.0),
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
    mut query: Query<(&mut Transform, &mut Velocity, &mut Direction, &mut Position)>
) {
    for (mut transform, mut velocity, mut direction, mut position) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            let prev_dir = direction.angle;
            direction.rotate_left();
            let dir_change = direction.angle - prev_dir;
            transform.rotate(Quat::from_rotation_z(dir_change));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            let prev_dir = direction.angle;
            direction.rotate_right();
            let dir_change = direction.angle - prev_dir;
            transform.rotate(Quat::from_rotation_z(dir_change));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            let dir_velocity: f32 = (velocity.y_vel/velocity.x_vel).atan();
            let difference: f32 = direction.angle - dir_velocity;
            println!("Difference {}", difference);
            if difference < PI {
                let prev_dir = direction.angle;
                direction.rotate_right();
                let dir_change = direction.angle - prev_dir;
                transform.rotate(Quat::from_rotation_z(dir_change));
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.accelerate(direction.angle);
        }

        transform.translation.x += SPEED_FACTOR*velocity.x_vel;
        transform.translation.y += SPEED_FACTOR*velocity.y_vel;
        position.x += SPEED_FACTOR*velocity.x_vel;
        position.y += SPEED_FACTOR*velocity.y_vel;
    }
}

fn edge_warp(mut query: Query<(&mut Transform, &mut Position)>) {
    let edge_buffer: f32 = 15.;
    for (mut transform, mut position) in query.iter_mut() {
        println!("Position: x = {}  y = {}", position.x, position.y);
        if position.x > (WINDOW_X/2. - edge_buffer) {
            transform.translation.x -= WINDOW_X;
            position.x -= WINDOW_X
        } else if position.x < -(WINDOW_X/2. + edge_buffer) {
            transform.translation.x += WINDOW_X;
            position.x += WINDOW_X
        }

        if position.y > (WINDOW_Y/2. - edge_buffer) {
            transform.translation.y -= WINDOW_Y;
            position.y -= WINDOW_Y;
        } else if position.y < (-WINDOW_Y/2. + edge_buffer) {
            transform.translation.y += WINDOW_Y;
            position.y += WINDOW_Y;
        }
    }
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
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
        .add_system(resize_notificator)
        .add_system(player_movement)
        .add_system(edge_warp)
        .add_plugins(DefaultPlugins)
        .run();
}
