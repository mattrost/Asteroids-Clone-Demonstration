# Bundles
In Bevy, a bundle is a trait that groups components together. The purpose of
bundling is to make the spawning of entities have a more easily visible
grouping. Lets start first with the player's ship!

## Player Ship Bundle
To create a bundle, we use `#[derive(Bundle)]` in front of our bundle, similarly
to what we did above with the components. We then create a structure that is
composed of the components that we would like to group together.

A player's ship has health, position, velocity, direction, and human components.

```rust, noplayground
#[derive(Bundle)]
struct PlayerShipBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    human: Human,
}
```

## Asteroid Bundle
An asteroid has a health, position, and a velocity. Unlike a ship, it does not
have a direction or a human component, as it is floating freely without
the ability to accelerate so it has no need for direction, and it is not
to be controlled by the player's inputs so it has no need for the human
component.

```rust, noplayground
#[derive(Bundle)]
struct AsteroidBundle {
    health: Health,
    position: Position,
    velocity: Velocity,
}
```

## Laser Bundle
The final bundle we are creating is a laser bundle. It is made up of position,
velocity, projectile timer, and damage.

```rust, noplayground
#[derive(Bundle)]
struct LaserBundle {
    position: Position,
    velocity: Velocity,
    projectile: ProjectileTimer,
    damage: Damage,
}
```

All of these bundles are used to spawn entities. As you can see, many of the
entities will have some of the same components. Each bundle is a little
different than the others and has some of its own unique components. Feel free
to read up more on how a component and entity interact together online!
