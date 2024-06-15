use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::{
    game_and_scene::{ball::Ball, game::Game, operation::Operation, paddle::Paddle, scene::Sides},
    math_utils::vec2::Vec2,
    DEFAULT_BACKGROUND_COLOR,
};

/// The default color of rectangles in the game. In this game, all paddles and balls are
/// rectangles so this constant applies on all of them.
pub const DEFAULT_RECT_COLOR: Color = Color::WHITE;

/// The game renderer. It takes a game and a canvas to render the game onto the canvas. Currently
/// it takes the ownership of both of them, but ideally it should take mutable references.
pub struct GameRenderer {
    /// The game struct that needs to be rendered.
    game: Game,
    /// The canvas that the game is rendered onto.
    canvas: WindowCanvas,
}

/// Get the rectangle used to render on the canvas. Takes a reference to the paddle and returns an
/// sdl2 rectangle.
fn get_rect_from_paddle(paddle: &Paddle) -> Rect {
    let pos: &Vec2 = paddle.get_pos();
    Rect::from_center::<Point>(pos.into(), paddle.get_width(), paddle.get_height())
}

/// Basically the same as get_rect_from_paddle, but for balls instead.
fn get_rect_from_ball(ball: &Ball) -> Rect {
    let pos: &Vec2 = ball.get_pos();
    let width = ball.get_radius() * 2;
    Rect::from_center::<Point>(pos.into(), width, width)
}

impl GameRenderer {
    /// The constructor.
    pub fn new(game: Game, canvas: WindowCanvas) -> Self {
        Self { game, canvas }
    }

    /// Collect all left paddles in the game into a vector of rectangles for further rendering.
    pub fn get_left_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_left_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    /// Collect all right paddles in the game into a vector of rectangles for further rendering.
    pub fn get_right_paddle_rects(&self) -> Vec<Rect> {
        self.game
            .get_right_paddles()
            .iter()
            .map(get_rect_from_paddle)
            .collect()
    }

    /// Collect all balls in the game into a vector of rectangles for further rendering.
    pub fn get_ball_rects(&self) -> Vec<Rect> {
        self.game
            .get_balls()
            .iter()
            .map(get_rect_from_ball)
            .collect()
    }

    /// Collect all balls and paddles in the game into a vector of rectangles for further rendering.
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

    /// Update the internal game according to a sequence of operations.
    pub fn update_game(&mut self, ops: &mut Vec<Operation>) -> Option<Sides> {
        self.game.update(ops)
    }

    /// Render the content of the game onto the canvas.
    pub fn render_to_canvas(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.set_draw_color(DEFAULT_BACKGROUND_COLOR);
        self.canvas.clear();
        self.canvas.set_draw_color(DEFAULT_RECT_COLOR);
        self.canvas.fill_rects(&self.get_all_rects())?;
        self.canvas.present();
        Ok(())
    }

    /// Start a default game. The definition of a default game is given in main.rs.
    pub fn start_default_game_with_2_balls(&mut self) {
        self.game.start_default_game_with_2_balls();
    }

    /// Reset the game. This is unimplemented.
    pub fn reset_game(&mut self) {
        self.game.reset();
    }
}
