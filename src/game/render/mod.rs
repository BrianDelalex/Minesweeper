use crate::game::Game;

use crate::ncurses::{clearw, echochar, printw, set_cursor_pos};

pub fn render(game: &Game) {
    clearw();
    for y in 0..((game.height) as usize) {
        for x in 0..((game.width - 1) as usize) {
            echochar(game.map[y * (game.width as usize) + x].content as i8);
            echochar(32);
        }
        echochar('\n' as i8);
    }
}
