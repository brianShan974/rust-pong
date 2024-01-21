use super::paddle::Paddle;
use super::scene::Edges;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;
use crate::game::paddle;
use crate::math_utils::vec2::Vec2;

pub const DEFAULT_RADIUS: u8 = 3;
pub const DEFAULT_BALL_SPEED: f32 = (SCREEN_WIDTH / 200) as f32;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: u8,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2, radius: u8) -> Self {
        Self { pos, vel, radius }
    }

    pub fn generate_with_vel(vel: Vec2) -> Self {
        Self {
            pos: Vec2::default(),
            vel,
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
        use Edges::{Bottom, Top};
        match *collision {
            WithEdge(Top) => {
                // if self.pos.y +
                todo!("Implement reset_pos");
            }
            _ => {}
        }
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
