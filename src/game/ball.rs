use rand::rngs::ThreadRng;

use super::paddle::Paddle;
use super::scene::Edges;
use crate::math_utils::vec2::Vec2;

use super::paddle::DEFAULT_PADDLE_WIDTH;
use super::scene::PADDLE_MARGIN;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;

pub const DEFAULT_RADIUS: u16 = 3;
pub const DEFAULT_BALL_SPEED: f32 = (SCREEN_WIDTH / 200) as f32;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: u16,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2, radius: u16) -> Self {
        Self { pos, vel, radius }
    }

    pub fn generate_with_vel(vel: Vec2) -> Self {
        Self {
            pos: Vec2::default(),
            vel,
            radius: DEFAULT_RADIUS,
        }
    }

    pub fn random_centered_ball(randomizer: Option<&mut ThreadRng>) -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::random_with_magnitude(DEFAULT_BALL_SPEED, randomizer),
            radius: DEFAULT_RADIUS,
        }
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    pub fn get_vel(&self) -> &Vec2 {
        &self.vel
    }

    pub fn bounce_after_collision(&mut self, collision: Collision) {
        use Collision::*;
        use Edges::{Bottom, Top};
        self.reset_pos(&collision);
        match collision {
            WithEdge(Top) | WithEdge(Bottom) => self.vel.y *= -1.0,
            WithPaddle(paddle) => self.vel.x *= -1.0,
            _ => {}
        }
    }

    fn reset_pos(&mut self, collision: &Collision) {
        use Collision::*;
        match collision {
            WithPaddle(paddle) => {
                if paddle.is_left() {
                    self.pos.x = (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH + self.radius) as f32;
                } else {
                    self.pos.x = SCREEN_WIDTH as f32
                        - (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH + self.radius) as f32
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

    pub fn update_pos(&mut self) {
        self.pos += &self.vel;
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::default(),
            radius: DEFAULT_RADIUS,
        }
    }
}

pub enum Collision {
    WithEdge(Edges),
    WithPaddle(Paddle),
}
