use super::paddle::Paddle;

pub enum Operations<'a> {
    Up(&'a mut Paddle),
    Down(&'a mut Paddle),
    Stay,
}
