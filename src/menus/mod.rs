use crate::{
    game,
    ncurses::{attroff, attron, echochar, printw, refreshw, set_cursor_pos, A_BLINK},
};

pub mod main_menu;
pub mod map_selection_menu;

struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: i32,
    height: i32,
}

struct Label {
    position: Position,
    text: String,
    callback: fn() -> i32,
}

struct Menu {
    labels: Vec<Label>,
    cursor: usize,
    size: Size,
    label_gap: i32,
    title_func: fn(&mut i32, &mut i32, i32, i32),
    handle_input: fn(&mut Menu) -> i32,
}

impl Menu {
    fn SetCursor(&mut self, new_cursor: usize) {
        set_cursor_pos(
            self.labels[self.cursor].position.x - 3,
            self.labels[self.cursor].position.y,
        );
        echochar(32);
        set_cursor_pos(
            self.labels[self.cursor].position.x - 2,
            self.labels[self.cursor].position.y,
        );
        echochar(32);
        self.cursor = new_cursor;
    }
    fn Open(&mut self) {
        self.Draw();
        while (true) {
            self.DrawCursor();
            if (self.handle_input)(self) == 0 {
                return;
            };
            refreshw();
        }
    }

    fn Draw(&mut self) {
        let mut x = 0;
        let mut y = 0;
        (self.title_func)(&mut x, &mut y, self.size.width, self.size.height);
        y += 5;

        for i in 0..self.labels.len() {
            x = self.size.width / 2 - self.labels[i].text.len() as i32 / 2;
            self.labels[i].position.x = x;
            self.labels[i].position.y = y;
            set_cursor_pos(x, y);
            printw(&self.labels[i].text);
            y += self.label_gap;
        }
    }

    fn DrawCursor(&self) {
        attron(A_BLINK());
        set_cursor_pos(
            self.labels[self.cursor].position.x - 3,
            self.labels[self.cursor].position.y,
        );
        echochar('-' as i8);
        set_cursor_pos(
            self.labels[self.cursor].position.x - 2,
            self.labels[self.cursor].position.y,
        );
        echochar('>' as i8);
        attroff(A_BLINK());
    }
}
