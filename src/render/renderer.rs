use sdl2::rect::{Point, Rect};

use crate::{
    game::{ball::Ball, game::Game, paddle::Paddle},
    math_utils::vec2::Vec2,
};

pub struct Renderer<'a> {
    game: &'a Game,
}

fn get_rect_from_paddle(paddle: &Paddle) -> Rect {
    let pos: &Vec2 = paddle.get_pos();
    Rect::from_center::<Point>(pos.into(), paddle.get_width(), paddle.get_height())
}

fn get_rect_from_ball(ball: &Ball) -> Rect {
    let pos: &Vec2 = ball.get_pos();
    let width = ball.get_radius() * 2;
    Rect::from_center::<Point>(pos.into(), width, width)
}

impl<'a> Renderer<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }

    pub fn get_left_paddles(&self) -> Vec<Rect> {
        self.game
            .get_left_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    pub fn get_right_paddles(&self) -> Vec<Rect> {
        self.game
            .get_right_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    pub fn get_balls(&self) -> Vec<Rect> {
        self.game
            .get_balls()
            .iter()
            .map(get_rect_from_ball)
            .collect()
    }
}
