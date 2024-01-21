use super::paddle::Paddle;

pub enum Operations {
    Up(Paddle),
    Down(Paddle),
    Stay,
}
