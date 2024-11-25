use crate::{
    game::start,
    ncurses::{
        clearw, cursor_set, get_key_curses, get_window_size, init_curses, printw, set_cursor_pos,
        NcursesKeyCode::{KEY_DOWN, KEY_ENTER, KEY_UP},
    },
};

use super::{Label, Menu, Position, Size};

const MAP_SELECTION_MENU_ART_WIDTH: i32 = 61;
const MAP_SELECTION_MENU_ART_HEIGHT: i32 = 6;

pub fn map_selection_menu_print_title(x: &mut i32, y: &mut i32, width: i32, height: i32) {
    *x = width / 2 - MAP_SELECTION_MENU_ART_WIDTH / 2;
    *y = 5;
    set_cursor_pos(*x, *y);
    printw(" __  __                        _           _   _             ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("|  \\/  | __ _ _ __    ___  ___| | ___  ___| |_(_) ___  _ __  ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("| |\\/| |/ _` | '_ \\  / __|/ _ \\ |/ _ \\/ __| __| |/ _ \\| '_ \\ ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("| |  | | (_| | |_) | \\__ \\  __/ |  __/ (__| |_| | (_) | | | |");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("|_|  |_|\\__,_| .__/  |___/\\___|_|\\___|\\___|\\__|_|\\___/|_| |_|");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("             |_|                                             ");
}

pub fn open_map_selection_menu() {
    let mut width = 0;
    let mut height = 0;
    clearw();
    get_window_size(&mut width, &mut height);
    cursor_set(0);

    let mut menu = Menu {
        labels: vec![
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Easy (9x9)"),
                callback: map_selection_menu_open_easy,
            },
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Medium (16x16)"),
                callback: map_selection_menu_open_medium,
            },
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Hard (30x16)"),
                callback: map_selection_menu_open_hard,
            },
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Custom..."),
                callback: map_selection_menu_open_custom,
            },
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Back"),
                callback: map_selection_menu_on_back,
            },
        ],
        cursor: 0,
        size: Size { width, height },
        label_gap: 2,
        title_func: map_selection_menu_print_title,
        handle_input: map_selection_menu_handle_input,
    };
    menu.Open();
}

fn map_selection_menu_handle_input(menu: &mut Menu) -> i32 {
    let code = get_key_curses();
    if code as u32 == KEY_UP {
        if menu.cursor == 0 {
            menu.SetCursor(menu.labels.len() - 1);
        } else {
            menu.SetCursor(menu.cursor - 1);
        }
    }
    if code as u32 == KEY_DOWN {
        if menu.cursor == menu.labels.len() - 1 {
            menu.SetCursor(0);
        } else {
            menu.SetCursor(menu.cursor + 1);
        }
    }
    if code as u32 == KEY_ENTER {
        let ret = (menu.labels[menu.cursor].callback)();
        menu.SetCursor(0);
        clearw();
        menu.Draw();
        return ret;
    }
    return 1;
}

fn map_selection_menu_open_easy() -> i32 {
    start(9, 9, 10);
    1
}

fn map_selection_menu_open_medium() -> i32 {
    start(16, 16, 40);
    1
}

fn map_selection_menu_open_hard() -> i32 {
    start(30, 16, 99);
    1
}

fn map_selection_menu_open_custom() -> i32 {
    1
}

fn map_selection_menu_on_back() -> i32 {
    0
}
