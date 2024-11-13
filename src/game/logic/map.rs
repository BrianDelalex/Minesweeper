use super::generator::generate_map;

pub struct Cell {
    pub hidden: bool,
    pub flagged: u32,
    pub content: char,
}

impl Cell {
    pub fn increment_flagged(&mut self) {
        if self.flagged < 2 {
            self.flagged += 1;
        } else {
            self.flagged = 0;
        }
    }
}

impl Copy for Cell {}

impl Clone for Cell {
    fn clone(&self) -> Self {
        *self
    }
}

pub enum State {
    Running,
    GameOver(bool),
}

pub struct Game {
    pub map: Vec<Cell>,
    pub width: u32,
    pub height: u32,
    pub number_of_mine: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub state: State,
    pub count: u32,
}

impl Game {
    pub fn new(width: u32, height: u32, number_of_mine: u32, x_offset: u32, y_offset: u32) -> Self {
        Game {
            map: generate_map(width, height, number_of_mine),
            width,
            height,
            number_of_mine,
            x_offset,
            y_offset,
            state: State::Running,
            count: 0,
        }
    }

    pub fn uncover_cell(&mut self, x: usize, y: usize) {
        let width: usize = self.width as usize;
        let index = y * width + x;
        self.map[index].hidden = false;
        self.count += 1;
        if self.map[index].content == 'x' {
            self.uncover_all_mine();
            self.state = State::GameOver(false);
        } else {
            self.uncover_cells(index);
        }
        if self.count == self.map.len() as u32 - self.number_of_mine {
            self.state = State::GameOver(true);
        }
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        let width: usize = self.width as usize;
        self.map[y * width + x].increment_flagged();
    }

    fn uncover_cells(&mut self, index: usize) {
        if self.map[index].content == '0' {
            self.uncover_zero_adjacent_cell(index);
        } else {
            match self.has_zero_adjacent_cell(index) {
                Some(idx) => {
                    self.count += 1;
                    self.map[idx].hidden = false;
                    self.uncover_zero_adjacent_cell(idx);
                }
                None => {}
            };
        }
    }

    fn has_zero_adjacent_cell(&mut self, index: usize) -> Option<usize> {
        let width = self.width as i32;
        let height = self.height as i32;
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
                let cell = self.map[index2];
                if cell.hidden {
                    if cell.content == '0' {
                        return Some(index2);
                    }
                }
            }
        }
        None
    }

    fn uncover_zero_adjacent_cell(&mut self, index: usize) {
        let width = self.width as i32;
        let height = self.height as i32;
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
                let cell = self.map[index2];
                if cell.hidden {
                    self.map[index2].hidden = false;
                    self.count += 1;
                    if cell.content == '0' {
                        self.uncover_zero_adjacent_cell(index2);
                    }
                }
            }
        }
    }

    fn uncover_all_mine(&mut self) {
        for idx in 0..self.map.len() {
            if self.map[idx].content == 'x' {
                self.map[idx].hidden = false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    #[test]
    pub fn check_maps() {
        let configs = [
            (10, 10, 10),
            (100, 100, 100),
            (1000, 1000, 1000),
            (50, 50, 50),
            (74, 74, 200),
            (4, 4, 1),
        ];
        for conf in configs {
            check_map(conf.0, conf.1, conf.2);
        }
    }

    fn check_map(width: u32, height: u32, number_of_mine: u32) {
        let game = Game::new(width, height, number_of_mine, 0, 0);
        check_number_of_bomb(&game, number_of_mine);
    }

    fn check_number_of_bomb(game: &Game, _number_of_mine: u32) {
        let mut count = 0;
        for i in 0..game.map.len() {
            if game.map[i].content == 'x' {
                count += 1;
            }
        }
        assert!(count == _number_of_mine);
    }
}
