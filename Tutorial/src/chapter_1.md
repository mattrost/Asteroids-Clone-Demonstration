# Project Initialization
## Creating a Bevy App
First we must create an empty Rust program. To do this, we will start by
running `cargo new game`. Cargo will then create a new rust app with `main.rs`
and `Cargo.toml` in its directory.
## Modifying App Dependencies
We then will open up `Cargo.toml` in a text editor or IDE and add Bevy to the
dependencies section. This tutorial was created with Bevy version 0.6.1.
Note: The `Cargo.toml` file is used to contain information about the Project
including its name, version, authors, and also contains information about
crate dependencies. Crates can be found at [crates.io](https://crates.io)
and function as packages for Rust programs. The dependencies are downloaded,
if necessary, when the app is compiled and run by using `cargo run`. We also
will add the `rand` crate which will be used later into the project when we
randomize the spawning of asteroids.

```toml
[dependencies]
bevy = "0.6.1"
rand = "0.8.5"
```

## Using a Dependency in main.rs
The next step is to update our `main.rs` file to use Bevy and initialize an
app. The example we are using below is a simple example that is shown in the
[Bevy Book Example](https://bevyengine.org/learn/book/getting-started/apps/).

```rust, noplayground
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

Make sure to now run `cargo run` in your directory. Nothing should happen yet,
but we are now set up to begin building our game!

Next we will begin working with Bevy's Entity-Component-System. An entity
is a single thing with its own identity. A component is the data that makes
up entities. A system is a behavior, implemented as a function.

To build the structure for our app we will begin by focusing on the Components.
