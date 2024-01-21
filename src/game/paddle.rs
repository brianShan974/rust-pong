use crate::math_utils::vec2::Vec2;

use super::scene::PADDLE_MARGIN;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;

use super::ball::DEFAULT_BALL_SPEED;

pub const DEFAULT_PADDLE_HEIGHT: u8 = SCREEN_HEIGHT as u8 / 10;
pub const DEFAULT_PADDLE_WIDTH: u8 = DEFAULT_PADDLE_HEIGHT / 8;
pub const DEFAULT_PADDLE_SPEED: f32 = DEFAULT_BALL_SPEED;

pub const DEFAULT_LEFT_CENTER_X: f32 = (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH / 2) as f32;

enum Sides {
    Left,
    Right,
}

pub struct Paddle {
    position: Vec2,
    width: u8,
    height: u8,
    side: Sides,
}

impl Paddle {
    pub fn new(position: Vec2, width: u8, height: u8, side: Sides) -> Self {
        Self {
            position,
            width,
            height,
            side,
        }
    }

    pub fn default_left_paddle() -> Self {
        Self {
            position: Vec2::new(DEFAULT_LEFT_CENTER_X, (SCREEN_HEIGHT / 2) as f32),
            width: PADDLE_MARGIN as u8,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Left,
        }
    }

    pub fn default_right_paddle() -> Self {
        Self {
            position: Vec2::new(
                (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH / 2) as f32,
                (SCREEN_HEIGHT / 2) as f32,
            ),
            width: PADDLE_MARGIN as u8,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Right,
        }
    }

    pub fn is_left(&self) -> bool {
        if let Sides::Left = self.side {
            true
        } else {
            false
        }
    }

    pub fn is_right(&self) -> bool {
        if let Sides::Right = self.side {
            true
        } else {
            false
        }
    }

    pub fn move_up(&mut self) {
        let final_y = self.position.y - DEFAULT_PADDLE_SPEED;
        if final_y >= 0.0 {
            self.position.y -= DEFAULT_PADDLE_SPEED;
        }
    }

    pub fn move_down(&mut self) {
        let final_y = self.position.y + DEFAULT_PADDLE_SPEED;
        if final_y <= SCREEN_HEIGHT as f32 {
            self.position.y += DEFAULT_PADDLE_SPEED;
        }
    }
}
