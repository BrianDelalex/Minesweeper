mod inputs;
use super::ncurses::{get_key_curses, init_curses, printw};
use inputs::actions::Actions;
use inputs::handle_input;

mod logic;
use logic::generator::generate_map;

mod render;
use render::render;

struct Cell {
    hidden: bool,
    content: char,
}

impl Copy for Cell {}

impl Clone for Cell {
    fn clone(&self) -> Self {
        *self
    }
}

struct Game {
    map: Vec<Cell>,
    width: u32,
    height: u32,
}

pub fn start(width: u32, height: u32, number_of_mine: u32) {
    let mut game = Game {
        map: generate_map(width, height, number_of_mine),
        width: width,
        height: height,
    };
    init_curses();
    game_loop(&mut game);
}

pub fn game_loop(game: &mut Game) {
    while true {
        render(&game);
        let code = get_key_curses() as u32;
        let action = handle_input(code);
        match action {
            Actions::LEFT_CLICK(ac) => printw(&format!("LEFT@{},{}", ac.x, ac.y)),
            Actions::RIGHT_CLICK(ac) => printw(&format!("RIGHT@{},{}", ac.x, ac.y)),
            Actions::EXIT => {}
            Actions::INVALID => {}
        }
    }
}
