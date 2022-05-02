# Startup Systems
So far we have made the Components for our game. Up next is to create the
systems. A system is used for many low level functions. A few examples would
be for rendering graphics, creating new entities, or taking user input.
These systems will be added to our `fn main()` by using
`App.new().add_startup_system()` for systems that run on initialization and `App.new().add_system()` for systems that run in a loop.

## Our First System - Setup Camera
Our first system that we are going to create is a startup system. To create
a system, we create a function that takes the `Commands` which allows us to
spawn entities on the screen, or into our `World`. This system creates a camera
to display our 2D world to the player!

```rust, noplayground
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
```

We then will add this startup system to our main function by updating `fn main()`
to the following:

``` rust, noplayground
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .run();
}
```

Each new system we add will be tacked onto `App::new()`.

## Our First Complicated System - Spawn Player
Similarly to `setup_camera` above, we create a function that uses `Commands`
to spawn our ship onto the screen.

We begin by creating a player as a Player Ship Bundle. We initialize the values
for each component for this player entity. Next we use the commands to create
a sprite and put the player entity into our world.

Note: We are using another constant here

```rust, noplayground
const SHIP_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);

fn spawn_player(mut commands: Commands) {
    let player = PlayerShipBundle {
        health: Health { hp: 10 },
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
```

This system creates the player's ship's entity and spawns them at the center of
the window facing upwards.

## Spawn Asteroid
Asteroids isn't too much fun with just a player in the world. Let's create a
system for spawning Asteroids in a random position with a random velocity! We
are now going to use the `rand` crate that we added to our dependencies.
Please note that the first half of this system randomizes the position and
velocity for the asteroid.

```rust, noplayground
const WINDOW_X: f32 = 1280.;
const WINDOW_Y: f32 = 720.;

const ASTEROID_COLOR: Color = Color::rgb(0.0, 1.0, 1.0);

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
        health: Health { hp: 200 },
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
```
That is it for our startup systems! Using these we have been able to take
our components and create entities made up of them and represent them on the
screen!

Please continue to the next chapter to make some systems that continuously run!
