use std::error::Error;

use sdl2::pixels::Color;

mod game;
mod math_utils;

use crate::game::{
    game::Game,
    scene::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let game_mode = GameMode::Default;

    let game = if let GameMode::Default = game_mode {
        Game::new()
    } else {
        unimplemented!("Custom games are not yet implemented!")
    };

    println!("Hello, world!");
    Ok(())
}
