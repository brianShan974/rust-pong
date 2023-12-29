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

impl Scene {
    pub fn add_balls(&mut self, balls: Vec<Ball>) {
        self.balls.extend(balls);
    }

    pub fn clear_balls(&mut self) {
        self.balls.clear();
    }

    pub fn reset_balls(&mut self, balls: Vec<Ball>) {
        self.clear_balls();
        self.add_balls(balls);
    }

    pub fn add_left_paddles(&mut self, paddles: Vec<Paddle>) {
        self.left_paddles.extend(paddles);
    }

    pub fn add_right_paddles(&mut self, paddles: Vec<Paddle>) {
        self.right_paddles.extend(paddles);
    }

    pub fn clear_left_paddles(&mut self) {
        self.left_paddles.clear();
    }

    pub fn clear_right_paddles(&mut self) {
        self.right_paddles.clear();
    }

    pub fn reset_left_paddles(&mut self, paddles: Vec<Paddle>) {
        self.clear_left_paddles();
        self.add_left_paddles(paddles);
    }

    pub fn reset_right_paddles(&mut self, paddles: Vec<Paddle>) {
        self.clear_right_paddles();
        self.add_right_paddles(paddles);
    }

    pub fn clear_paddles(&mut self) {
        self.clear_left_paddles();
        self.clear_right_paddles();
    }

}
