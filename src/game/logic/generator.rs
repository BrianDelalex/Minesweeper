use super::map::Cell;
use rand::Rng;

pub fn generate_map(width: u32, height: u32, number_of_mine: u32) -> Vec<Cell> {
    let mut map: Vec<Cell> = vec![
        Cell {
            hidden: true,
            flagged: 0,
            content: 'o',
        };
        (width * height) as usize
    ];
    generate_mines(&mut map, number_of_mine);
    generate_numbers(&mut map, width, height);
    map
}

fn generate_mines(map: &mut Vec<Cell>, number_of_mine: u32) {
    for _i in 0..number_of_mine {
        let mut placed = false;
        'placing: while !placed {
            let num: usize = rand::thread_rng().gen_range(0..map.len());
            if map[num].content == 'x' {
                continue 'placing;
            } else {
                map[num].content = 'x';
                placed = true;
            }
        }
    }
}

fn generate_numbers(map: &mut Vec<Cell>, width: u32, height: u32) {
    for i in 0..map.len() as i32 {
        if map[i as usize].content == 'x' {
            continue;
        }
        let y: i32 = i / width as i32;
        let x: i32 = i - y * width as i32;
        let mut count: u8 = 0;
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
            if pos.0 >= 0 && pos.0 < width as i32 && pos.1 >= 0 && pos.1 < height as i32 {
                if map[(pos.1 * (width as i32) + pos.0) as usize].content == 'x' {
                    count += 1;
                }
            }
        }
        map[i as usize].content = (count + 48) as char;
    }
}
