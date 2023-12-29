use super::{ball::Ball, paddle::Paddle};

pub const SCREEN_WIDTH: u16 = 800;
pub const SCREEN_HEIGHT: u16 = 800;

pub enum Edges {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct Scene {
    left_paddles: Vec<Paddle>,
    right_paddles: Vec<Paddle>,
    balls: Vec<Ball>,
    edges: [Edges; 4],
}

impl Default for Scene {
    fn default() -> Self {
        use Edges::*;
        Self {
            left_paddles: Vec::new(),
            right_paddles: Vec::new(),
            balls: Vec::new(),
            edges: [Top, Bottom, Left, Right],
        }
    }
}

impl Scene {}