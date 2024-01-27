use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sdl2::pixels::Color;

mod game;
mod math_utils;
mod render;

use crate::{
    game::{
        game::Game,
        operation::{Operation, OperationTypes},
        paddle::Sides,
        scene::{SCREEN_HEIGHT, SCREEN_WIDTH},
    },
    render::game_renderer::GameRenderer,
};

pub const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;

pub const FRAME_DURATION: Duration = Duration::from_millis(20);

// a default game is a game with 2 paddles on each side and 2 balls
// a custom game can be customized, but it is not yet implemented
enum GameMode {
    Default,
    Custom,
}

fn main() -> Result<(), Box<dyn Error>> {
    // These codes are copied from sdl2 docs
    // https://docs.rs/sdl2/latest/sdl2/
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(DEFAULT_BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();

    // let event_pump = sdl_context.event_pump()?;

    let game_mode = GameMode::Default;

    let mut game = Game::new();

    if let GameMode::Default = game_mode {
        game.start_default_game_with_2_balls();
    } else {
        unimplemented!("Custom games not yet implemented!")
    }

    let mut renderer = GameRenderer::new(game, canvas);

    let mut ops: Vec<Operation> = Vec::new();

    for _ in 0..10 {
        while let None = renderer.update_game(&mut ops) {
            ops.push(Operation::new(OperationTypes::Up, Sides::Right, 0));
            ops.push(Operation::new(OperationTypes::Down, Sides::Left, 1));
            renderer.render_to_canvas()?;
            renderer.present();
            sleep(FRAME_DURATION);
        }

        // renderer.reset_game();
        renderer.start_default_game_with_2_balls();
    }

    println!("Hello, world!");
    Ok(())
}
