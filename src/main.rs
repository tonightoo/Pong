extern crate sdl2;
mod game;
use game::*;

use sdl2::log;

fn main() -> Result<(), String> {
    let mut game: Game = match Game::initialize() {
        Ok(g) => g,
        Err(msg) => {
            log::log(&msg);
            return Err(msg);
        }
    };

    game.run_loop();

    Ok(())
}
