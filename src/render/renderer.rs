use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::{
    game::{ball::Ball, game::Game, paddle::Paddle},
    math_utils::vec2::Vec2,
};

pub struct Renderer<'a, 'b> {
    game: &'a Game,
    canvas: &'b mut WindowCanvas,
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

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(game: &'a Game, canvas: &'b mut WindowCanvas) -> Self {
        Self { game, canvas }
    }

    fn get_left_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_left_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    fn get_right_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_right_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    fn get_ball_rects(&self) -> Vec<Rect> {
        self.game
            .get_balls()
            .iter()
            .map(get_rect_from_ball)
            .collect()
    }

    fn get_all_rects(&self) -> Vec<Rect> {
        let mut all_rects = Vec::new();

        let mut left_paddles = self.get_left_paddle_rects();
        let mut right_paddles = self.get_right_paddle_rects();
        let mut balls = self.get_ball_rects();

        all_rects.append(&mut left_paddles);
        all_rects.append(&mut right_paddles);
        all_rects.append(&mut balls);

        all_rects
    }
}
