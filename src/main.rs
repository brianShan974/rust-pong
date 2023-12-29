mod game;
mod math_utils;

fn main() -> Result<(), String> {
    let _context = sdl2::init()?;
    println!("Hello, world!");
    Ok(())
}
