use crate::ncurses::{
    clearw, cursor_set, get_key_curses, get_window_size, init_curses, printw, set_cursor_pos,
    NcursesKeyCode::{KEY_DOWN, KEY_ENTER, KEY_UP},
};

use super::map_selection_menu::open_map_selection_menu;

use super::{Label, Menu, Position, Size};

const MAIN_MENU_ART_WIDTH: i32 = 61;
const MAIN_MENU_ART_HEIGHT: i32 = 8;

pub fn open_main_menu() {
    let mut width = 0;
    let mut height = 0;
    init_curses();
    get_window_size(&mut width, &mut height);
    cursor_set(0);

    let mut menu = Menu {
        labels: vec![
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Start"),
                callback: main_menu_on_start,
            },
            Label {
                position: Position { x: 0, y: 0 },
                text: String::from("Infos"),
                callback: main_menu_on_infos,
            },
        ],
        cursor: 0,
        size: Size { width, height },
        label_gap: 2,
        title_func: main_menu_print_title,
        handle_input: main_menu_input,
    };

    menu.Open();
}

fn main_menu_input(menu: &mut Menu) -> i32 {
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
        if (menu.labels[menu.cursor].callback)() == 0 {
            return 0;
        }
        clearw();
        menu.Draw();
    }
    return 1;
}

fn main_menu_on_start() -> i32 {
    open_map_selection_menu();
    1
}

fn main_menu_on_infos() -> i32 {
    panic!("on_main_menu_infos: not implemented.");
    1
}

fn main_menu_print_title(x: &mut i32, y: &mut i32, width: i32, height: i32) {
    *x = width / 2 - MAIN_MENU_ART_WIDTH / 2;
    *y = 5;
    set_cursor_pos(*x, *y);
    printw("___  ____            _____                                   ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("|  \\/  (_)          /  ___|                                  ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("| .  . |_ _ __   ___\\ `--.__      _____  ___ _ __   ___ _ __ ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("| |\\/| | | '_ \\ / _ \\`--. \\ \\ /\\ / / _ \\/ _ \\ '_ \\ / _ \\ '__|");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("| |  | | | | | |  __/\\__/ /\\ V  V /  __/  __/ |_) |  __/ |   ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("\\_|  |_/_|_| |_|\\___\\____/  \\_/\\_/ \\___|\\___| .__/ \\___|_|   ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("                                            | |              ");
    *y += 1;
    set_cursor_pos(*x, *y);
    printw("                                            |_|              ");
}
