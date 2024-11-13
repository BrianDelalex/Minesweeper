mod inputs;
use super::ncurses::{get_key_curses, get_window_size, init_curses};
use inputs::actions::Actions;
use inputs::handle_input;

pub mod logic;
use logic::generator::generate_map;
use logic::map::{Game, State};

mod render;
use logic::{process_left_click, process_right_click};
use render::{draw_game_over, draw_win, render};

pub fn start(width: u32, height: u32, number_of_mine: u32) {
    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut game = Game {
        map: generate_map(width, height, number_of_mine),
        width,
        height,
        x_offset: 0,
        y_offset: 0,
        state: State::Running,
        count: 0,
        number_of_mine,
    };
    init_curses();
    get_window_size(&mut screen_width, &mut screen_height);
    game.x_offset = screen_width as u32 / 2 - width;
    game.y_offset = screen_height as u32 / 2 - height / 2;
    game_loop(&mut game);
}

pub fn game_loop(game: &mut Game) {
    while matches!(game.state, State::Running) {
        render(&game);
        let code = get_key_curses() as u32;
        let action = handle_input(code, game);
        match action {
            Actions::LeftClick(ac) => process_left_click(&ac, game),
            Actions::RightClick(ac) => process_right_click(&ac, game),
            Actions::Exit => {}
            Actions::Invalid => {}
        }
        match game.state {
            State::GameOver(state) => {
                if state {
                    render(&game);
                    handle_win(game);
                    get_key_curses();
                } else {
                    render(&game);
                    handle_lose(game);
                    get_key_curses();
                }
            }
            State::Running => {}
        }
    }
}

fn handle_win(game: &mut Game) {
    draw_win(game);
}

fn handle_lose(game: &mut Game) {
    draw_game_over(game);
}
