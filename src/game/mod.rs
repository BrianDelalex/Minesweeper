mod inputs;
use super::ncurses::{get_key_curses, init_curses, printw};
use inputs::actions::Actions;
use inputs::handle_input;

mod logic;
use logic::generator::generate_map;

mod render;
use logic::{process_left_click, process_right_click};
use render::render;

struct Cell {
    hidden: bool,
    flagged: bool,
    content: char,
    debug_color: i32,
}

impl Copy for Cell {}

impl Clone for Cell {
    fn clone(&self) -> Self {
        *self
    }
}

enum State {
    RUNNING,
    GAME_OVER(bool),
}

struct Game {
    map: Vec<Cell>,
    width: u32,
    height: u32,
    state: State,
}

pub fn start(width: u32, height: u32, number_of_mine: u32) {
    let mut game = Game {
        map: generate_map(width, height, number_of_mine),
        width: width,
        height: height,
        state: State::RUNNING,
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
            Actions::LEFT_CLICK(ac) => process_left_click(&ac, game),
            Actions::RIGHT_CLICK(ac) => process_right_click(&ac, game),
            Actions::EXIT => {}
            Actions::INVALID => {}
        }
    }
}
