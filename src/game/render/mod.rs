use crate::game::Game;

use crate::ncurses::{
    addchar, attroff, attron, clearw, echochar, printw, refreshw, set_cursor_pos, COLOR_PAIR,
    PAIR_BLUE_WHITE,
};

use super::Cell;

pub fn render(game: &Game) {
    clearw();
    for y in 0..((game.height) as usize) {
        for x in 0..((game.width - 1) as usize) {
            let cell = game.map[y * (game.width as usize) + x];
            if cell.debug_color != 0 {
                attron(COLOR_PAIR(cell.debug_color));
            }
            if cell.hidden {
                if cell.flagged {
                    addchar('?' as i8);
                } else {
                    addchar('#' as i8);
                }
            } else {
                addchar(cell.content as i8);
            }
            addchar(32);
            if cell.debug_color != 0 {
                attroff(COLOR_PAIR(cell.debug_color));
            }
        }

        addchar('\n' as i8);
    }
    refreshw();
}
