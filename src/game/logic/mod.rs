use super::inputs::actions::Click;

pub mod map;

use map::Game;

pub mod generator;

pub fn process_left_click(click: &Click, game: &mut Game) {
    if click.x % 2 != 0 {
        return;
    }
    let x: usize = click.x as usize / 2;
    let y: usize = click.y as usize;
    game.uncover_cell(x, y);
}

pub fn process_right_click(click: &Click, game: &mut Game) {
    if click.x % 2 != 0 {
        return;
    }
    let x: usize = click.x as usize / 2;
    let y: usize = click.y as usize;
    game.flag_cell(x, y);
}
