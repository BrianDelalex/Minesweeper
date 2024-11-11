use crate::ncurses::PAIR_BLUE_WHITE;

use super::{inputs::actions::Click, Game, State};

pub mod generator;

pub fn process_left_click(click: &Click, game: &mut Game) {
    if click.x % 2 != 0 {
        return;
    }
    let x: usize = click.x as usize / 2;
    let y: usize = click.y as usize;
    uncover_cell(x, y, game);
}

fn uncover_cell(x: usize, y: usize, game: &mut Game) {
    let width: usize = game.width as usize;
    let index = y * width + x;
    if game.map[index].hidden {
        game.map[index].hidden = false;
    }
    if game.map[index].content == 'x' {
        game.state = State::GAME_OVER(false);
    } else {
        uncover_cells(index, game);
    }
}

pub fn process_right_click(click: &Click, game: &mut Game) {
    if click.x % 2 != 0 {
        return;
    }
    let x: usize = click.x as usize / 2;
    let y: usize = click.y as usize;
    flag_cell(x, y, game);
}

fn flag_cell(x: usize, y: usize, game: &mut Game) {
    let width: usize = game.width as usize;
    let flagged = game.map[y * width + x].flagged;
    game.map[y * width + x].flagged = !flagged;
}

fn uncover_cells(index: usize, game: &mut Game) {
    if game.map[index].content == '0' {
        uncover_zero_adjacent_cell(index, game);
    } else {
        match has_zero_adjacent_cell(index, game) {
            Some(idx) => uncover_zero_adjacent_cell(idx, game),
            None => {}
        };
    }
}

fn has_zero_adjacent_cell(index: usize, game: &mut Game) -> Option<usize> {
    let width = game.width as i32;
    let height = game.height as i32;
    let y: i32 = index as i32 / width;
    let x: i32 = index as i32 - y * width;
    let positions: Vec<(i32, i32)> = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    for pos in positions {
        if pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
            let index2 = (pos.1 * width + pos.0) as usize;
            let cell = game.map[index2];
            if cell.hidden {
                if cell.content == '0' {
                    return Some(index2);
                }
            }
        }
    }
    None
}

fn uncover_zero_adjacent_cell(index: usize, game: &mut Game) {
    let width = game.width as i32;
    let height = game.height as i32;
    let y: i32 = index as i32 / width;
    let x: i32 = index as i32 - y * width;
    let positions: Vec<(i32, i32)> = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    for pos in positions {
        if pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
            let index2 = (pos.1 * width + pos.0) as usize;
            let cell = game.map[index2];
            if cell.hidden {
                game.map[index2].hidden = false;
                game.map[index2].debug_color = PAIR_BLUE_WHITE;
                if cell.content == '0' {
                    uncover_zero_adjacent_cell(index2, game);
                }
            }
        }
    }
}
