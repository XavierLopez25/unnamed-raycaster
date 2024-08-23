# Unnamed Raycaster

Unnamed Raycaster is a first-person maze navigation game built in Rust. The game features a simple raycasting engine that simulates 3D perspective in 2D mazes. Players navigate the maze, avoiding obstacles and reaching a goal.


## Demonstrations

#### Controller Usage Demo

See how the controller can be used to navigate and interact within the game:

https://github.com/user-attachments/assets/43de7cd7-d3e6-4d9b-9d5f-cc87e1dd603c




#### Game Footage

Check out this footage to see the game in action:

https://github.com/user-attachments/assets/41e2e8fe-5338-4405-9c4e-fbc42388d799




## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Prerequisites

You need Rust installed on your machine. The easiest way to install Rust and `cargo` (the Rust package manager) is through `rustup`. Install `rustup` by following the instructions here:

[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Installing

To get a development environment running, clone the repository and build the project:

```bash
git clone https://yourrepositorylink.com/unnamed-raycaster.git
cd unnamed-raycaster
cargo build
```
To run the game, use:

```bash
cargo run
```
## Game Controls

#### Start the Game: 
- Press 'S' at the title screen to start the game.
#### Movement:
- Keyboard:
    - Use WASD or arrow keys to move forward, backward, and turn.
- Mouse:
    - Horizontal movement controls player turning.
- Gamepad: 
    - Use the Left Stick for moving forward/backward and turning. D-pad can also be used for discrete movements.

### Built With
- cpal - Low-level library for audio playback
- gilrs - Game Input Library for Rust
- image - Imaging processing operations library
- lazy_static - A macro for declaring lazily evaluated statics in Rust
- minifb - A minimal cross-platform window library
- nalgebra-glm - A high-level linear algebra and geometry library
- once_cell - Single initialization and lazy evaluation of statics
- rodio - A Rust audio playback library

### Versioning
We use SemVer for versioning. For the versions available, see the tags on this repository.

### License
This project is licensed under the MIT License - see the LICENSE.md file for details
