use rand::rngs::ThreadRng;

use super::ball::Collision;
use super::operation::{Operation, OperationTypes};
use super::{ball::Ball, paddle::Paddle};

use super::paddle::DEFAULT_PADDLE_WIDTH;

/// The default screen width.
pub const SCREEN_WIDTH: u32 = 800;
/// The default screen height.
pub const SCREEN_HEIGHT: u32 = 600;

/// The default margin on the left and the right of the screen. This is the distance between the
/// left/right edge of the left/right paddles and the left/right edges.
pub const PADDLE_MARGIN: u32 = DEFAULT_PADDLE_WIDTH;

/// The 4 edges of the scene.
pub enum Edges {
    /// The top edge.
    Top,
    /// The bottom edge.
    Bottom,
    /// The left edge.
    Left,
    /// The right edge.
    Right,
}

/// There are 2 sides in the game, that are left and right.
#[derive(Debug, Clone, Copy)]
pub enum Sides {
    /// The left side of the scene.
    Left,
    /// The right side of the scene.
    Right,
}

/// The scene struct. It contains vectors of left and right paddles and balls. Edges are not
/// contained since there will always be 4 edges.
#[derive(Default)]
pub struct Scene {
    /// The vector containing all left paddles.
    left_paddles: Vec<Paddle>,
    /// The vector containing all right paddles.
    right_paddles: Vec<Paddle>,
    /// The vector containing all balls.
    balls: Vec<Ball>,
}

impl Scene {
    /// Add balls into the scene.
    pub fn add_balls(&mut self, balls: Vec<Ball>) {
        self.balls.extend(balls);
    }

    /// Clear all balls in the scene.
    pub fn clear_balls(&mut self) {
        self.balls.clear();
    }

    /// First clear all balls, then add the provided balls.
    pub fn reset_balls(&mut self, balls: Vec<Ball>) {
        self.clear_balls();
        self.add_balls(balls);
    }

    /// Add left paddles to the scene.
    pub fn add_left_paddles(&mut self, paddles: Vec<Paddle>) {
        self.left_paddles.extend(paddles);
    }

    /// Add right paddles to the scene.
    pub fn add_right_paddles(&mut self, paddles: Vec<Paddle>) {
        self.right_paddles.extend(paddles);
    }

    /// Clear all left paddles in the scene.
    pub fn clear_left_paddles(&mut self) {
        self.left_paddles.clear();
    }

    /// Clear all right paddles in the scene.
    pub fn clear_right_paddles(&mut self) {
        self.right_paddles.clear();
    }

    /// First clear all left paddles, then add the provided left paddles.
    pub fn reset_left_paddles(&mut self, paddles: Vec<Paddle>) {
        self.clear_left_paddles();
        self.add_left_paddles(paddles);
    }

    /// First clear all right paddles, then add the provided right paddles.
    pub fn reset_right_paddles(&mut self, paddles: Vec<Paddle>) {
        self.clear_right_paddles();
        self.add_right_paddles(paddles);
    }

    /// Clear all paddles in the scene.
    pub fn clear_paddles(&mut self) {
        self.clear_left_paddles();
        self.clear_right_paddles();
    }

    /// Whether there is no left paddle in the scene.
    pub fn has_no_left_paddles(&self) -> bool {
        self.left_paddles.is_empty()
    }

    /// Whether there is no right paddle in the scene.
    pub fn has_no_right_paddles(&self) -> bool {
        self.right_paddles.is_empty()
    }

    /// Whether there is no ball in the scene.
    pub fn has_no_balls(&self) -> bool {
        self.balls.is_empty()
    }

    /// Get all left paddles in the scene.
    pub fn get_left_paddles(&self) -> &Vec<Paddle> {
        &self.left_paddles
    }

    /// Get all right paddles in the scene.
    pub fn get_right_paddles(&self) -> &Vec<Paddle> {
        &self.right_paddles
    }

    /// Get all balls in the scene.
    pub fn get_balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    /// Construct a default scene. A default scene is defined as the scene in a default game, and
    /// a default game is defined in `main.rs`.
    pub fn construct_default_scene_with_2_balls(rng: &mut ThreadRng) -> Self {
        Self {
            left_paddles: vec![Paddle::default_left_paddle(), Paddle::default_left_paddle()],
            right_paddles: vec![
                Paddle::default_right_paddle(),
                Paddle::default_right_paddle(),
            ],
            balls: vec![
                Ball::random_centered_ball(rng),
                Ball::random_centered_ball(rng),
            ],
        }
    }

    /// Update the scene given a sequence of operations. Returns the winner of the current game if
    /// any.
    pub fn update_scene(&mut self, ops: &mut Vec<Operation>) -> Option<Sides> {
        let mut winner: Option<Sides> = None;
        use OperationTypes::*;
        use Sides::*;
        while let Some(op) = ops.pop() {
            if let Stay = op.op_type {
                continue;
            }

            let paddle: &mut Paddle = match op.side {
                Left => self.left_paddles.get_mut(op.index).unwrap(),
                Right => self.right_paddles.get_mut(op.index).unwrap(),
            };

            match op.op_type {
                Up => paddle.move_up(),
                Down => paddle.move_down(),
                _ => {}
            }
        }

        for ball in self.balls.iter_mut() {
            ball.update_pos();
            let radius_in_f32 = ball.get_radius() as f32;

            use Collision::*;
            use Edges::*;

            // detect collision with top and bottom edge
            if (ball.get_pos().y - radius_in_f32) < 0.0 {
                ball.bounce_after_collision(WithEdge(&Top));
            } else if (ball.get_pos().y + radius_in_f32) > SCREEN_HEIGHT as f32 {
                ball.bounce_after_collision(WithEdge(&Bottom));
            }

            // detect collision with left paddles
            for paddle in self.left_paddles.iter() {
                if let Some(collision) = ball.collides_with(paddle) {
                    ball.bounce_after_collision(collision);
                }
            }

            // detect collision with right paddles
            for paddle in self.right_paddles.iter() {
                if let Some(collision) = ball.collides_with(paddle) {
                    ball.bounce_after_collision(collision);
                }
            }

            // detect if the current game is over
            // if so, set the winner variable to left or right
            if (ball.get_pos().x + radius_in_f32) < 0.0 {
                winner = Some(Sides::Right);
            } else if (ball.get_pos().x - radius_in_f32) > SCREEN_WIDTH as f32 {
                winner = Some(Sides::Left);
            }
        }

        winner
    }
}
