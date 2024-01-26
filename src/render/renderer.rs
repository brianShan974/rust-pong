use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::{
    game::{
        ball::Ball,
        game::Game,
        paddle::{Paddle, Sides},
    },
    math_utils::vec2::Vec2,
    DEFAULT_BACKGROUND_COLOR,
};

pub const DEFAULT_RECT_COLOR: Color = Color::WHITE;

pub struct Renderer<'a> {
    game: &'a mut Game,
    canvas: WindowCanvas,
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
    pub fn new(game: &'a mut Game, canvas: WindowCanvas) -> Self {
        Self { game, canvas }
    }

    pub fn get_left_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_left_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    pub fn get_right_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_right_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    pub fn get_ball_rects(&self) -> Vec<Rect> {
        self.game
            .get_balls()
            .iter()
            .map(get_rect_from_ball)
            .collect()
    }

    pub fn get_all_rects(&self) -> Vec<Rect> {
        let mut all_rects = Vec::new();

        let mut left_paddles = self.get_left_paddle_rects();
        let mut right_paddles = self.get_right_paddle_rects();
        let mut balls = self.get_ball_rects();

        all_rects.append(&mut left_paddles);
        all_rects.append(&mut right_paddles);
        all_rects.append(&mut balls);

        all_rects
    }

    pub fn update_game(&mut self) -> Option<Sides> {
        unimplemented!("update_game not implemented.")
    }

    pub fn render_to_canvas(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(DEFAULT_BACKGROUND_COLOR);
        self.canvas.clear();
        self.canvas.set_draw_color(DEFAULT_RECT_COLOR);
        self.canvas.fill_rects(&self.get_all_rects())?;
        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
