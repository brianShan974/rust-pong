use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod game_and_scene;
mod math_utils;
mod render;

use crate::{
    game_and_scene::{
        game::Game,
        game_mode::GameMode,
        operation::Operation,
        scene::{SCREEN_HEIGHT, SCREEN_WIDTH},
    },
    render::game_renderer::GameRenderer,
};

/// The default background color.
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;

/// The time interval between each frame.
pub const FRAME_DURATION: Duration = Duration::from_millis(20);
/// Whether the game runs at full speed. If true, `FRAME_DURATION` will be ignored and each frame
/// is rendered as soon as the calculation finished.
const FULL_SPEED: bool = false;

/// Whether it is a customized game or it is a default game. Customized games are not yet supported
/// as it involves mapping keyboard inputs to control multiple paddles.
const GAME_MODE: GameMode = GameMode::Default;
/// Whether the game is being played by humans. If true, it reads keyboard inputs to control the
/// paddles. If false, all input events are ignored except for
/// `Event::KeyDown {keycode: Some(Keycode::Escape),..}`.
const HUMAN_PLAYING: bool = true;

/// The default number of games. It is set to 10 by default, so the program will terminate after 10
/// games.
const DEFAULT_NUMBER_OF_GAMES: u32 = 10;

/// The main function. At this point, only the default game is implemented so the main function
/// starts a default game.
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
    let mut event_pump = sdl_context.event_pump()?;
    let mut should_quit = false;

    canvas.set_draw_color(DEFAULT_BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();

    // let event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    if let GameMode::Default = GAME_MODE {
        game.start_default_game_with_2_balls();
    } else {
        unimplemented!("Custom games not yet implemented!")
    }

    let mut renderer = GameRenderer::new(game, canvas);

    let mut ops: Vec<Operation> = Vec::new();

    let mut i = 0;
    let number_of_games = DEFAULT_NUMBER_OF_GAMES;
    loop {
        if should_quit || i >= number_of_games {
            break;
        }

        i += 1;
        // renderer.reset_game();
        renderer.start_default_game_with_2_balls();

        while renderer.update_game(&mut ops).is_none() {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => should_quit = true,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if HUMAN_PLAYING {
                            if let Some(op) = Operation::from_key_code(key) {
                                ops.push(op);
                            }
                        }
                    }
                    _ => {}
                }
            }
            // ops.push(Operation::new(OperationTypes::Up, Sides::Right, 0));
            // ops.push(Operation::new(OperationTypes::Down, Sides::Left, 1));
            renderer.render_to_canvas()?;
            if !FULL_SPEED {
                sleep(FRAME_DURATION);
            }
        }
    }

    println!("Hello, world!");
    Ok(())
}
