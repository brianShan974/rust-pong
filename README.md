# rust-pong
My implementation of pong, one of the first video games in history, in rust.
This is part of a reinforcement learning project, which is why it is a template repository.

This is a simple template repository. It has the game logic and very basic rendering. Keyboard input is now handled.

Make sure to have SDL2 installed on your machine before trying to build this project.

## Building and Running the Project

In order to build this project, simply use
```cargo build```
and in order to run this game, simply use
```cargo run```

In a default game, there are 2 paddles on each side of the scene.
In order to control the paddles on the left side, press `W` and `S` to move one of the paddles up and down, and press `A` and `D` to control the other.
In order to control the paddles on the left side, press `Up` arrow key and `Down` arrow key to move one of the paddles up and down, and press `Left` arrow key and `Right` arrow key to control the other.

In order to exit the game, either use the close button, or press the `Esc` key.
The game will only exit after the next time one of the balls flies out of the screen.

## Documents

In order to read the documents of this project, simply use
```cargo doc```
or
```cargo doc --open```
