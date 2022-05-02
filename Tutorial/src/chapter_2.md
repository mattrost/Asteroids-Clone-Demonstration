# Components
## What is a Component
According to [Adam Martin](http://t-machine.org/index.php/2007/11/11/entity-systems-are-the-future-of-mmog-development-part-2/), a component is the raw data for one aspect
of an object, and how it interacts with the world. An entity is made up of
this raw data and can be made of multiple components. A few examples he listed
for a Bicycle component are the following:
- Material
- Usable by humans
- Means of transportation
- Can be bought and sold
- A potential present
- Man-made

## Our First Component - Health
We are going to start a little bit simpler with our first component.
Each player and each flying asteroid will have their own health. When the
health reaches zero, that entity will die. To make a component for this we
will use an integer to represent the health value. A component is created by
first putting `#[derive(Component)]` on a line and beneath it creating a
struct that is the component.

```rust, noplayground
#[derive(Component)]
struct Health {
    hp: u64,
}
```

Each health component will have an hp attribute that stores the amount of
hit points that an entity has.

## Position
A commonly used comopnent for entities is the position. This will be represented
by the x and y axis positions. In Bevy, the origin (x=0, y=0) is located at
the center of the window.

```rust, noplayground
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

## Velocity
Entities that are moving in space have a velocity. This component is used to
store the velocity of the entity. We also are introducing an implementation here
to our Velocity struct. This implementation is used to accelerate the velocity.
This will be used when a player presses the accelerate button on their keyboard.

We also are introducing a constant here, `MAX_SPEED`. This is a global
constant that is used to set the maximum velocity for any entity.

```rust, noplayground
const MAX_SPEED: f32 = 100.;

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
```

## Direction
Similarly to the component above, Velocity, Direction is another important
parameter to the movement system we are creating for the game. The angle
is measured in radians and is used to determine the direction that a ship
is facing. This is important for accelerating and when the player shoots
their lasers. This component contains two implementations for turning.

```rust, noplayground
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
```

## Damage
The Damage component represents the amount of damage that an entity will
inflict on an entity that has Health upon collision. An example of the use for
this would be when a laser collides into an asteroid or when an asteroid collides
into a player. A laser would have a significantly smaller damage value than an
asteroid would.

```rust, noplayground
#[derive(Component)]
struct Damage {
    damage: u64,
}
```

## Projectile Timer
The Projectile Timer component is specifically used for lasers shot by the
player. This component creates a timer and determines when the laser entity
should no longer be present on the screen. This is used to prevent lasers
from spawning and existing infinitely until contact. After a timer has elapsed,
we will know when to de-spawn our laser entities.

```rust, noplayground
#[derive(Component)]
struct ProjectileTimer {
    timer: Timer,
    elapsed: bool,
}
```

## Human
The final component we are creating for this tutorial is a Human component.
This is used to show if a human player is controlling an object or not. The
use for this would be to have the future controlling systems only allow for the
player to control their ship entity.

```rust, noplayground
#[derive(Component)]
struct Human {
    is_human: bool,
}
```
