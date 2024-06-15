use rand::rngs::ThreadRng;

use super::paddle::Paddle;
use super::scene::Edges;
use crate::math_utils::vec2::Vec2;

use super::paddle::DEFAULT_PADDLE_WIDTH;
use super::scene::PADDLE_MARGIN;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;

/// The default radius of the ball. The unit is in pixels.
pub const DEFAULT_RADIUS: u32 = 3;
/// The default speed of the ball. The unit is in pixels per frame.
pub const DEFAULT_BALL_SPEED: f32 = (SCREEN_WIDTH / 200) as f32;

/// The ball struct.
pub struct Ball {
    /// The position of the center of the ball.
    pos: Vec2,
    /// The velocity of the ball. The unit is pixels per frame.
    vel: Vec2,
    /// The radius of the ball. The unit is pixels.
    radius: u32,
}

impl Ball {
    /// The constructor.
    pub fn new(pos: Vec2, vel: Vec2, radius: u32) -> Self {
        Self { pos, vel, radius }
    }

    /// Generate a new ball given its velocity. The generated ball will be located at the center of
    /// the screen and has the default radius.
    pub fn generate_with_vel(vel: Vec2) -> Self {
        Self {
            pos: Vec2::default(),
            vel,
            radius: DEFAULT_RADIUS,
        }
    }

    /// Generate a new ball as generate_with_vel, but the velocity is random instead.
    pub fn random_centered_ball(rng: &mut ThreadRng) -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::random_with_magnitude(DEFAULT_BALL_SPEED, None, rng),
            radius: DEFAULT_RADIUS,
        }
    }

    /// Get the position of the ball.
    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get the velocity of the ball.
    pub fn get_vel(&self) -> &Vec2 {
        &self.vel
    }

    /// Get the radius of the ball.
    pub fn get_radius(&self) -> u32 {
        self.radius
    }

    /// Update the ball after it bounces with a paddle or an edge.
    pub fn bounce_after_collision(&mut self, collision: Collision) {
        use Collision::*;
        use Edges::{Bottom, Top};
        match collision {
            WithEdge(Top) | WithEdge(Bottom) => self.vel.y *= -1.0,
            WithPaddle(_) => self.vel.x *= -1.0,
            _ => {}
        }
        self.reset_pos(&collision);
    }

    /// Resets the position of the ball after bouncing to prevent bugs. By bugs, I mean that since
    /// a collision is detected when the ball and the paddle (or edge) overlap, part of the ball
    /// may still be inside the paddle (or out of the edge) after changing its direction, in which
    /// case the ball will keep changing its direction. Therefore, we need to move it outside the
    /// paddle (or inside the edge), so that it will only change its direction once.
    fn reset_pos(&mut self, collision: &Collision) {
        use Collision::*;
        match collision {
            WithPaddle(paddle) => {
                if paddle.is_left() {
                    self.pos.x = (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH + self.radius) as f32;
                } else {
                    self.pos.x =
                        (SCREEN_WIDTH - PADDLE_MARGIN - DEFAULT_PADDLE_WIDTH - self.radius) as f32
                }
            }
            _ => {
                if self.pos.y + self.radius as f32 > SCREEN_HEIGHT as f32 {
                    self.pos.y = (SCREEN_HEIGHT - self.radius) as f32;
                } else if (self.pos.y - self.radius as f32) < 0.0 {
                    self.pos.y = self.radius as f32;
                }
            }
        };
    }

    /// Updates its position according to its velocity.
    pub fn update_pos(&mut self) {
        self.pos += &self.vel;
    }

    /// Detects collision with a paddle.
    pub fn collides_with<'a>(&self, paddle: &'a Paddle) -> Option<Collision<'a>> {
        use Collision::WithPaddle;

        let &Vec2 {
            x: paddle_pos_x,
            y: paddle_pos_y,
        } = paddle.get_pos();
        let half_height: f32 = (paddle.get_height() / 2) as f32;
        let half_width: f32 = (paddle.get_width() / 2) as f32;
        let radius_as_f32 = self.radius as f32;

        // here, bounds mean the bounds of the y-values, not what we see on the screen
        // upper bound means that y-value shouldn't be greater than this value
        // lower bound means that y-value shouldn't be less than this value
        // on screen, upper bound is actually beneath lower bound
        let y_lower_bound: f32 = paddle_pos_y - half_height - radius_as_f32;
        let y_upper_bound: f32 = paddle_pos_y + half_height + radius_as_f32;

        let x_lower_bound: f32 = paddle_pos_x - half_width;
        let x_upper_bound: f32 = paddle_pos_x + half_width;

        if self.pos.y > y_lower_bound
            && self.pos.y < y_upper_bound
            && self.pos.x > x_lower_bound
            && self.pos.x < x_upper_bound
        {
            Some(WithPaddle(paddle))
        } else {
            None
        }
    }
}

impl Default for Ball {
    /// Generate a default ball.
    fn default() -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::default(),
            radius: DEFAULT_RADIUS,
        }
    }
}

/// A collision can either take place between a ball and a paddle, or between a ball and an edge.
pub enum Collision<'a> {
    WithEdge(&'a Edges),
    WithPaddle(&'a Paddle),
}
