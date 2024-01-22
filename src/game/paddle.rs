use crate::math_utils::vec2::Vec2;

use super::scene::PADDLE_MARGIN;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;

use super::ball::DEFAULT_BALL_SPEED;

pub const DEFAULT_PADDLE_HEIGHT: u32 = SCREEN_HEIGHT / 10;
pub const DEFAULT_PADDLE_WIDTH: u32 = DEFAULT_PADDLE_HEIGHT / 8;
pub const DEFAULT_PADDLE_SPEED: f32 = DEFAULT_BALL_SPEED;

pub const DEFAULT_LEFT_CENTER_X: f32 = (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH / 2) as f32;

#[derive(Debug, Clone, Copy)]
pub enum Sides {
    Left,
    Right,
}

pub struct Paddle {
    pos: Vec2,
    width: u32,
    height: u32,
    side: Sides,
}

impl Paddle {
    pub fn new(pos: Vec2, width: u32, height: u32, side: Sides) -> Self {
        Self {
            pos,
            width,
            height,
            side,
        }
    }

    pub fn default_left_paddle() -> Self {
        Self {
            pos: Vec2::new(DEFAULT_LEFT_CENTER_X, (SCREEN_HEIGHT / 2) as f32),
            width: PADDLE_MARGIN,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Left,
        }
    }

    pub fn default_right_paddle() -> Self {
        Self {
            pos: Vec2::new(
                (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH / 2) as f32,
                (SCREEN_HEIGHT / 2) as f32,
            ),
            width: PADDLE_MARGIN,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Right,
        }
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
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
        let final_y = self.pos.y - DEFAULT_PADDLE_SPEED;
        if final_y >= 0.0 {
            self.pos.y -= DEFAULT_PADDLE_SPEED;
        }
    }

    pub fn move_down(&mut self) {
        let final_y = self.pos.y + DEFAULT_PADDLE_SPEED;
        if final_y <= SCREEN_HEIGHT as f32 {
            self.pos.y += DEFAULT_PADDLE_SPEED;
        }
    }
}
