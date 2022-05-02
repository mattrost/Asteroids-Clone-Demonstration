# Runtime Systems
Part of the power of using ECS is that each component will be able to be quickly
queried. If we decided to use Object Oriented Programming (OOP), this program
would have looked very different up to and including this point and would go
against the principles of using Bevy as a data-driven game engine.

Each runtime system we create will query against certain components to
determine what entities are relevant to the system.

Our first system we create is fundamental to the player, asteroids, and laser
entities.

## Movement
We begin by determining the Query for our movement system. For movement,
we will use Transform, Velocity, and Position. Transform is Bevy's
component that is used to move entities along the screen. When the query is run,
it will iterate over each entity that has a Transform, Velocity, and a Position
and will translate the transform component to the next location and update the
position.

```rust, noplayground
const MAX_ACCEL: f32 = 0.05;

fn movement(mut query: Query<(&mut Transform, & Velocity, &mut Position)>) {
    for (mut transform, velocity, mut position) in query.iter_mut() {
        transform.translation.x += MAX_ACCEL * velocity.x_vel;
        transform.translation.y += MAX_ACCEL * velocity.y_vel;
        position.x += MAX_ACCEL * velocity.x_vel;
        position.y += MAX_ACCEL * velocity.y_vel;
    }
}
```

We then will add this to our app with `.add_system(movement)`.

## Edge Warp

The next system we will add is an edge warp. This is an important part of the
"Asteroids" style gameplay. When an entity goes off of the screen it warps to
the other side. This System adds this functionality in by moving the position
and translating the sprite if it is crossing over the edge.

```rust, noplayground
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
```

## Player Input

The next system we are creating is critical for the player to play the game!
This system takes the keyboard input and then will determine what the player's
ship will do. The arrow keys will rotate the ship and the up arrow will accelerate.
The down key will rotate the ship opposite of the direction it is travelling.

```rust, noplayground
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
```

## Spawn Laser

This system spawns a laser when the player presses the space bar. This is
a combination of what we have learned in the player input and with our earlier
startup systems. Combining those practices, we are able to spawn a new entity
when the player presses the space bar. We also spawn it past the player's ship
to avoid the new lasers from damaging the player.

```rust, noplayground
const LASER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const BULLET_SPEED: f32 = 100.;

fn spawn_laser(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Velocity, &Direction, &Position, &Human)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (velocity, direction, position, _human) in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            let laser = LaserBundle {
                position: Position {
                    x: position.x + 30. * direction.angle.cos(),
                    y: position.y + 30. * direction.angle.sin(),
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
                        translation: Vec3::new(laser.position.x, laser.position.y, 10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(laser.position)
                .insert(laser.velocity)
                .insert(laser.projectile)
                .insert(laser.damage);
        }
    }
}
```

## Despawn Laser
The despawn laser system is used to despawn a laser entity when it has reached
its time limit that was determined upon spawning.

```rust, noplayground
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
```

## Detect Laser Collision
The detect laser collision system is used to determine when a laser contacts
another object, especially asteroids. When this happens we will remove a certain
amount of hit points from the asteroid and then despawn the laser. If the entity
that we hit with the laser goes below zero hit points we will also despawn that
entity.

```rust, noplayground
fn detect_laser_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Damage, &Transform)>,
    mut query: Query<(Entity, &Transform, &mut Health)>,
) {
    for (entity, transform, mut health) in query.iter_mut() {
        for (laser_entity, damage, laser_transform) in laser_query.iter() {
            let laser_size = laser_transform.scale.truncate();

            let collision = collide(
                laser_transform.translation,
                laser_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                health.hp = health.hp - damage.damage;

                if health.hp <= 0 {
                    commands.entity(entity).despawn();
                }
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
```
