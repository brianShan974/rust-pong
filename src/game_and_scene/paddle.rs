use crate::math_utils::vec2::Vec2;

use super::scene::PADDLE_MARGIN;
use super::scene::SCREEN_HEIGHT;
use super::scene::SCREEN_WIDTH;

use super::ball::DEFAULT_BALL_SPEED;

/// The default paddle height.
pub const DEFAULT_PADDLE_HEIGHT: u32 = SCREEN_HEIGHT / 10;
/// The default paddle width.
pub const DEFAULT_PADDLE_WIDTH: u32 = DEFAULT_PADDLE_HEIGHT / 8;
/// The default paddle speed. The unit is pixels per frame.
pub const DEFAULT_PADDLE_SPEED: f32 = DEFAULT_BALL_SPEED * 5.0;

/// The default x position of the center of the left paddles.
pub const DEFAULT_LEFT_CENTER_X: f32 = (PADDLE_MARGIN + DEFAULT_PADDLE_WIDTH / 2) as f32;
/// The default x position of the center of the right paddles.
pub const DEFAULT_RIGHT_CENTER_X: f32 =
    (SCREEN_WIDTH - PADDLE_MARGIN - DEFAULT_PADDLE_WIDTH / 2) as f32;

/// There are 2 sides in the game, that are left and right.
#[derive(Debug, Clone, Copy)]
pub enum Sides {
    Left,
    Right,
}

/// The paddle struct.
pub struct Paddle {
    pos: Vec2,
    width: u32,
    height: u32,
    side: Sides,
}

impl Paddle {
    /// The constructor of a paddle.
    pub fn new(pos: Vec2, width: u32, height: u32, side: Sides) -> Self {
        Self {
            pos,
            width,
            height,
            side,
        }
    }

    /// Generates a default left paddle.
    pub fn default_left_paddle() -> Self {
        Self {
            pos: Vec2::new(DEFAULT_LEFT_CENTER_X, (SCREEN_HEIGHT / 2) as f32),
            width: PADDLE_MARGIN,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Left,
        }
    }

    /// Generates a default left paddle.
    pub fn default_right_paddle() -> Self {
        Self {
            pos: Vec2::new(DEFAULT_RIGHT_CENTER_X, (SCREEN_HEIGHT / 2) as f32),
            width: PADDLE_MARGIN,
            height: DEFAULT_PADDLE_HEIGHT,
            side: Sides::Right,
        }
    }

    /// Get the position of the paddle. The position of a paddle is defined as the position of the
    /// center of the paddle.
    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get the width of the paddle.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height of the paddle.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Whether the paddle is a left paddle.
    pub fn is_left(&self) -> bool {
        matches!(self.side, Sides::Left)
    }

    /// Whether the paddle is a right paddle.
    pub fn is_right(&self) -> bool {
        matches!(self.side, Sides::Right)
    }

    /// The method called when an up operation is received.
    pub fn move_up(&mut self) {
        let final_y = self.pos.y - DEFAULT_PADDLE_SPEED;
        if final_y >= 0.0 {
            self.pos.y -= DEFAULT_PADDLE_SPEED;
        }
    }

    /// The method called when a down operation is received.
    pub fn move_down(&mut self) {
        let final_y = self.pos.y + DEFAULT_PADDLE_SPEED;
        if final_y <= SCREEN_HEIGHT as f32 {
            self.pos.y += DEFAULT_PADDLE_SPEED;
        }
    }
}
