use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::event::Events;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;
//use bevy::window::WindowResized;

/*
Future Ideas:

Have neverending level with hi score. Spawn random letters on asteroids.
Player must consecutively shoot asteroids to spell an English word. Word
is scored with difficulty of letters, scrabble style. Maybe a few random
powerups. On hit, player loses and present high score.
 */

const WINDOW_X: f32 = 1280.;
const WINDOW_Y: f32 = 720.;

const SHIP_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
const ASTEROID_COLOR: Color = Color::rgb(0.0, 1.0, 1.0);
const LASER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const MAX_SPEED: f32 = 100.;
const BULLET_SPEED: f32 = 100.;
const MAX_ACCEL: f32 = 0.05;
const TURN_FACTOR: f32 = 3.0;

#[derive(Component)]
struct Health(u64);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Damage {
    damage: u64,
}

#[derive(Component)]
struct Velocity {
    x_vel: f32,
    y_vel: f32,
}
impl Velocity {
    fn accelerate(&mut self, direction: f32) {
        self.x_vel = self.x_vel + direction.cos();
        self.y_vel = self.y_vel + direction.sin();
        let squares: f32 = self.x_vel * self.x_vel + self.y_vel * self.y_vel;

        if squares > MAX_SPEED * MAX_SPEED {
            let current_speed: f32 = squares.sqrt();
            self.x_vel *= MAX_SPEED / current_speed;
            self.y_vel *= MAX_SPEED / current_speed;
        }
    }
}

#[derive(Component)]
struct ProjectileTimer {
    timer: Timer,
    elapsed: bool,
}

#[derive(Component)]
struct Direction {
    angle: f32,
}
impl Direction {
    fn rotate_right(&mut self) {
        self.angle -= TURN_FACTOR * 0.01;
        if self.angle >= 2. * PI {
            self.angle -= 2. * PI
        }
    }
    fn rotate_left(&mut self) {
        self.angle += TURN_FACTOR * 0.01;
        if self.angle < 0. {
            self.angle += 2. * PI
        }
    }
}

#[derive(Component)]
struct Human {
    is_human: bool,
}

#[derive(Bundle)]
struct PlayerShipBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    human: Human,
}

#[derive(Bundle)]
struct AsteroidBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
}

#[derive(Bundle)]
struct LaserBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
    projectile: ProjectileTimer,
    damage: Damage,
}

fn spawn_player(mut commands: Commands) {
    let player = PlayerShipBundle {
        health: Health(10),
        position: Position { x: 0., y: 0. },
        velocity: Velocity {
            x_vel: 0.0,
            y_vel: 0.0,
        },
        direction: Direction { angle: PI / 2. },
        human: Human { is_human: true },
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
        .insert(player.direction)
        .insert(player.human);
}

fn spawn_asteroid(mut commands: Commands) {
    let rand_angle: f32 = rand::random::<f32>() * 2. * PI;
    let rand_vel: f32 = rand::random::<f32>() * MAX_SPEED;

    let mut rand_pos_x: f32 = rand::random::<f32>() * WINDOW_X;
    let mut rand_pos_y: f32 = rand::random::<f32>() * WINDOW_Y;

    if rand_pos_x > WINDOW_X / 2. {
        rand_pos_x -= WINDOW_X;
    }

    if rand_pos_y > WINDOW_Y / 2. {
        rand_pos_y -= WINDOW_Y;
    }

    let rand_vel_x: f32 = rand_vel * rand_angle.cos();
    let rand_vel_y: f32 = rand_vel * rand_angle.sin();

    let asteroid = AsteroidBundle {
        health: Health(10),
        position: Position {
            x: rand_pos_x,
            y: rand_pos_y,
        },
        velocity: Velocity {
            x_vel: rand_vel_x,
            y_vel: rand_vel_y,
        },
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ASTEROID_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 10.0),
                translation: Vec3::new(rand_pos_x, rand_pos_y, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(asteroid.health)
        .insert(asteroid.position)
        .insert(asteroid.velocity);
}

fn spawn_laser(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Velocity, &Direction, &Position, &Human)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (velocity, direction, position, _human) in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            let laser = LaserBundle {
                health: Health(1),
                position: Position {
                    x: position.x,
                    y: position.y,
                },
                velocity: Velocity {
                    x_vel: BULLET_SPEED * direction.angle.cos() + velocity.x_vel,
                    y_vel: BULLET_SPEED * direction.angle.sin() + velocity.y_vel,
                },
                projectile: ProjectileTimer {
                    timer: Timer::from_seconds(0.5, true),
                    elapsed: false,
                },
                damage: Damage { damage: 5 },
            };
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: LASER_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        scale: Vec3::new(5.0, 5.0, 10.0),
                        translation: Vec3::new(position.x, position.y, 10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(laser.health)
                .insert(laser.position)
                .insert(laser.velocity)
                .insert(laser.projectile)
                .insert(laser.damage);
        }
    }
}

fn despawn_laser(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ProjectileTimer)>,
) {
    for (entity, mut projectile) in query.iter_mut() {
        if projectile.timer.tick(time.delta()).just_finished() {
            projectile.elapsed = true;
        }
        if projectile.elapsed == true {
            commands.entity(entity).despawn();
        }
    }
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Direction,
        &Position,
        &Human,
    )>,
) {
    for (mut transform, mut velocity, mut direction, _position, _human) in query.iter_mut() {
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
            let dir_velocity: f32 = (velocity.y_vel / velocity.x_vel).atan();
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
    }
}

fn movement(mut query: Query<(&mut Transform, &mut Velocity, &mut Position)>) {
    for (mut transform, velocity, mut position) in query.iter_mut() {
        transform.translation.x += MAX_ACCEL * velocity.x_vel;
        transform.translation.y += MAX_ACCEL * velocity.y_vel;
        position.x += MAX_ACCEL * velocity.x_vel;
        position.y += MAX_ACCEL * velocity.y_vel;
    }
}

fn edge_warp(mut query: Query<(&mut Transform, &mut Position)>) {
    let edge_buffer: f32 = 15.;

    for (mut transform, mut position) in query.iter_mut() {
        if position.x > (WINDOW_X / 2. - edge_buffer) {
            transform.translation.x -= WINDOW_X;
            position.x -= WINDOW_X
        } else if position.x < (-WINDOW_X / 2. + edge_buffer) {
            transform.translation.x += WINDOW_X;
            position.x += WINDOW_X
        }

        if position.y > (WINDOW_Y / 2. - edge_buffer) {
            transform.translation.y -= WINDOW_Y;
            position.y -= WINDOW_Y;
        } else if position.y < (-WINDOW_Y / 2. + edge_buffer) {
            transform.translation.y += WINDOW_Y;
            position.y += WINDOW_Y;
        }
    }
}

/*
fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
    }
}
*/

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_asteroid)
        //.add_system(resize_notificator)
        .add_system(player_input)
        .add_system(movement)
        .add_system(edge_warp)
        .add_system(spawn_laser)
        .add_system(despawn_laser)
        .run();
}
