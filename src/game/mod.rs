mod inputs;
use super::ncurses::{get_key_curses, get_window_size, init_curses};
use inputs::actions::Actions;
use inputs::handle_input;

mod logic;
use logic::generator::generate_map;

mod render;
use logic::{process_left_click, process_right_click};
use render::{draw_game_over, render};

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
    x_offset: u32,
    y_offset: u32,
    state: State,
}

pub fn start(width: u32, height: u32, number_of_mine: u32) {
    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut game = Game {
        map: generate_map(width, height, number_of_mine),
        width,
        height,
        x_offset: 0,
        y_offset: 0,
        state: State::RUNNING,
    };
    init_curses();
    get_window_size(&mut screen_width, &mut screen_height);
    game.x_offset = screen_width as u32 / 2 - width;
    game.y_offset = screen_height as u32 / 2 - height / 2;
    game_loop(&mut game);
}

pub fn game_loop(game: &mut Game) {
    while true {
        render(&game);
        let code = get_key_curses() as u32;
        let action = handle_input(code, game);
        match action {
            Actions::LEFT_CLICK(ac) => process_left_click(&ac, game),
            Actions::RIGHT_CLICK(ac) => process_right_click(&ac, game),
            Actions::EXIT => {}
            Actions::INVALID => {}
        }
        match game.state {
            State::GAME_OVER(state) => {
                if state {
                    handle_win()
                } else {
                    render(&game);
                    handle_lose(game);
                    get_key_curses();
                }
            }
            State::RUNNING => {}
        }
    }
}

fn handle_win() {}

fn handle_lose(game: &mut Game) {
    draw_game_over(game);
}
