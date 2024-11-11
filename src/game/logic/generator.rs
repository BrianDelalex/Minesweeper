use crate::{game::Cell, ncurses::printw};
use rand::Rng;

pub fn generate_map(width: u32, height: u32, number_of_mine: u32) -> Vec<Cell> {
    let mut map: Vec<Cell> = vec![
        Cell {
            hidden: true,
            content: '#',
        };
        (width * height) as usize
    ];
    generate_mines(&mut map, number_of_mine);
    map
}

fn generate_mines(map: &mut Vec<Cell>, number_of_mine: u32) {
    for i in 0..number_of_mine {
        let mut placed = false;
        while !placed {
            let num: usize = rand::thread_rng().gen_range(0..map.len());
            if map[num].content == 'x' {
                continue;
            } else {
                map[num].content = 'x';
                placed = true;
            }
        }
    }
}
