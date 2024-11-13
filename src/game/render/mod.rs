use crate::game::logic::map::Game;

use crate::ncurses::Colors::{PAIR_DANGER, PAIR_MINE, PAIR_SAFE, PAIR_WARNING};

use crate::ncurses::{addchar, attroff, attron, clearw, refreshw, set_cursor_pos, COLOR_PAIR};

pub fn set_color(c: char) {
    match c {
        'x' => attron(COLOR_PAIR(PAIR_MINE)),
        '0' => attron(COLOR_PAIR(PAIR_SAFE)),
        '1' | '2' => attron(COLOR_PAIR(PAIR_WARNING)),
        _ => attron(COLOR_PAIR(PAIR_DANGER)),
    };
}

pub fn unset_color(c: char) {
    match c {
        'x' => attroff(COLOR_PAIR(PAIR_MINE)),
        '0' => attroff(COLOR_PAIR(PAIR_SAFE)),
        '1' | '2' => attroff(COLOR_PAIR(PAIR_WARNING)),
        _ => attroff(COLOR_PAIR(PAIR_DANGER)),
    };
}

pub fn render(game: &Game) {
    clearw();
    for y in 0..((game.height) as usize) {
        set_cursor_pos(game.x_offset as i32, (game.y_offset + y as u32) as i32);
        for x in 0..(game.width as usize) {
            let cell = game.map[y * (game.width as usize) + x];
            if cell.hidden {
                if cell.flagged == 1 {
                    addchar('!' as i8);
                } else if cell.flagged == 2 {
                    addchar('?' as i8);
                } else {
                    addchar('#' as i8);
                }
            } else {
                set_color(cell.content);
                addchar(cell.content as i8);
                unset_color(cell.content);
            }
            addchar(32);
        }
    }
    refreshw();
}

pub fn draw_game_over(game: &Game) {
    let txt = String::from("Game Over !");
    set_cursor_pos(
        (game.x_offset as i32 + game.width as i32) - txt.len() as i32 / 2,
        game.y_offset as i32 / 2,
    );
    for x in txt.chars() {
        addchar(x as i8);
    }
    refreshw();
}

pub fn draw_win(game: &Game) {
    let txt = String::from("You win !");
    set_cursor_pos(
        (game.x_offset as i32 + game.width as i32) - txt.len() as i32 / 2,
        game.y_offset as i32 / 2,
    );
    for x in txt.chars() {
        addchar(x as i8);
    }
    refreshw();
}
