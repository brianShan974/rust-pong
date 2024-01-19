use super::paddle::Paddle;

enum Operations {
    Up(Paddle),
    Down(Paddle),
    Stay,
}

