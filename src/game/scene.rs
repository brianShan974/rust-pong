use rand::rngs::ThreadRng;

use super::ball::Collision;
use super::operation::Operations;
use super::{ball::Ball, paddle::Paddle};

use super::paddle::{self, Sides, DEFAULT_PADDLE_WIDTH};

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub const PADDLE_MARGIN: u32 = DEFAULT_PADDLE_WIDTH;

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
}

impl Default for Scene {
    fn default() -> Self {
        use Edges::*;
        Self {
            left_paddles: Vec::new(),
            right_paddles: Vec::new(),
            balls: Vec::new(),
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

    pub fn has_no_left_paddles(&self) -> bool {
        self.left_paddles.is_empty()
    }

    pub fn has_no_right_paddles(&self) -> bool {
        self.right_paddles.is_empty()
    }

    pub fn has_no_balls(&self) -> bool {
        self.balls.is_empty()
    }

    pub fn get_left_paddles(&self) -> &Vec<Paddle> {
        &self.left_paddles
    }

    pub fn get_right_paddles(&self) -> &Vec<Paddle> {
        &self.right_paddles
    }

    pub fn get_balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    pub fn construct_default_scene_with_2_balls(rng: &mut ThreadRng) -> Self {
        use Edges::*;
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

    pub fn update_scene(&mut self, op: Operations) -> Option<Sides> {
        let mut winner: Option<Sides> = None;
        match op {
            Operations::Up(paddle) => paddle.move_up(),
            Operations::Down(paddle) => paddle.move_down(),
            Operations::Stay => {}
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

            // detect collision with paddles
            for paddle in self.left_paddles.iter() {
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
