# Introduction

Hello! This is a tutorial on how to make a simple 2D space combat game with
the Bevy game engine. This is the first program I have made with Bevy, so if
you have any questions, concerns, or suggestions please submit an issue on
[GitHub](https://github.com/mattrost/meteors)
 or feel free to send me an email at
[rostma@oregonstate.edu](mailto:rostma@oregonstate.edu).

This tutorial assumes that the user has already installed rust on their
computer. If that is not the case, please refer to the
["Getting Started"](https://www.rust-lang.org/tools/install)
section of rust-lang.org. This tutorial also assumes that users have some
programming fundamental skills and some understanding of how Rust and
Cargo work. Users do not need to feel proficient with Rust, especially since
I am also currently learning Rust! This tutorial was created after following
the
[Bevy Book Example](https://bevyengine.org/learn/book/introduction/)
and
[M Buffett's Bevy Snake Clone](https://mbuffett.com/posts/bevy-snake-tutorial/).

This tutorial uses player created physics. If you are interested in pre-existing
physics for Bevy, feel free to take a look at [Rapier](https://crates.io/crates/rapier2d)
