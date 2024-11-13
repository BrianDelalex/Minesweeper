#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ptr;

use cty::{c_char, c_short};

static mut WINDOW: *mut cty::c_void = ptr::null_mut();

pub mod NcursesKeyCode {
    #![allow(unused_variables)]
    pub const KEY_DOWN: u32 = 258;
    pub const KEY_UP: u32 = 259;
    pub const KEY_LEFT: u32 = 260;
    pub const KEY_RIGHT: u32 = 261;
    pub const KEY_BACKSPACE: u32 = 263;
    pub const KEYS_CTRL_LEFT: u32 = 560;
    pub const KEYS_CTRL_RIGHT: u32 = 575;
    pub const KEY_MOUSE: u32 = 409;
}

pub mod Colors {
    /* COLOR CONSTANT */
    pub const COLOR_BLACK: i16 = 0;
    pub const COLOR_RED: i16 = 1;
    pub const COLOR_GREEN: i16 = 2;
    pub const COLOR_YELLOW: i16 = 3;
    pub const COLOR_BLUE: i16 = 4;
    pub const COLOR_MAGENTA: i16 = 5;
    pub const COLOR_CYAN: i16 = 6;
    pub const COLOR_WHITE: i16 = 7;

    /* COLOR PAIR CONSTANT */
    pub const PAIR_BLUE_WHITE: i32 = 1;
    pub const PAIR_MINE: i32 = 2;
    pub const PAIR_SAFE: i32 = 3;
    pub const PAIR_WARNING: i32 = 4;
    pub const PAIR_DANGER: i32 = 5;
}

pub const BUTTON_CLICKED: u64 = 004;

fn mouse_mask(b: u64, m: u64) -> u64 {
    (m) << (((b) - 1) * 5)
}

fn all_mouse_event() -> u64 {
    report_mouse_position() - 1
}

fn report_mouse_position() -> u64 {
    mouse_mask(6, 0010)
}

pub fn BUTTON1_CLICKED() -> u64 {
    mouse_mask(1, BUTTON_CLICKED)
}

pub fn BUTTON3_CLICKED() -> u64 {
    mouse_mask(3, BUTTON_CLICKED)
}

pub fn COLOR_PAIR(pair: i32) -> i32 {
    NCURSES_BITS(pair, 0) & NCURSES_BITS(((1) << 8) - 1, 0)
}

pub fn NCURSES_BITS(mask: i32, shift: i32) -> i32 {
    (mask) << ((shift) + 8)
}

extern "C" {
    fn wrefresh(window: *mut cty::c_void);
    fn wgetch(window: *mut cty::c_void) -> i32;
    fn delch();
    fn pechochar(window: *mut cty::c_void, ch: c_char);
    fn waddch(window: *mut cty::c_void, ch: c_char);
    fn initscr() -> *mut cty::c_void;
    fn noecho() -> i32;
    fn keypad(window: *mut cty::c_void, value: bool);
    fn wclear(window: *mut cty::c_void) -> i32;
    fn getcury(window: *mut cty::c_void) -> i32;
    fn getcurx(window: *mut cty::c_void) -> i32;
    fn wmove(window: *mut cty::c_void, y: cty::c_int, x: cty::c_int);
    fn getmaxx(window: *mut cty::c_void) -> i32;
    fn getmaxy(window: *mut cty::c_void) -> i32;
    fn mousemask(newmask: cty::c_ulong, oldmask: *mut cty::c_void);
    fn getmouse(event: *mut MEVENT) -> i32;
    fn start_color() -> i32;
    fn wattron(window: *mut cty::c_void, attrs: cty::c_int);
    fn wattroff(window: *mut cty::c_void, attrs: cty::c_int);
    fn init_pair(pair: cty::c_short, fg: cty::c_short, bg: c_short) -> i32;
}

#[repr(C)]
pub struct MEVENT {
    pub id: cty::c_short,
    pub x: cty::c_int,
    pub y: cty::c_int,
    pub z: cty::c_int,
    pub bstate: cty::c_ulong,
}

impl Default for MEVENT {
    fn default() -> MEVENT {
        MEVENT {
            id: 0,
            x: 0,
            y: 0,
            z: 0,
            bstate: 0,
        }
    }
}

pub fn init_curses() {
    unsafe {
        WINDOW = initscr();
        noecho();
        keypad(WINDOW, true);
        mousemask(all_mouse_event() | report_mouse_position(), ptr::null_mut());
        start_color();
        init_pair(
            Colors::PAIR_BLUE_WHITE as i16,
            Colors::COLOR_BLUE,
            Colors::COLOR_WHITE,
        );
        init_pair(
            Colors::PAIR_MINE as i16,
            Colors::COLOR_RED,
            Colors::COLOR_WHITE,
        );
        init_pair(
            Colors::PAIR_SAFE as i16,
            Colors::COLOR_GREEN,
            Colors::COLOR_BLACK,
        );
        init_pair(
            Colors::PAIR_WARNING as i16,
            Colors::COLOR_YELLOW,
            Colors::COLOR_BLACK,
        );
        init_pair(
            Colors::PAIR_DANGER as i16,
            Colors::COLOR_RED,
            Colors::COLOR_BLACK,
        );
    }
}

pub fn printw(text: &str) {
    for c in String::from(text).chars() {
        echochar(c as i8);
    }
}

pub fn printnlw(text: &str) {
    printw(text);
    echochar('\n' as i8);
}

pub fn echochar(ch: i8) {
    unsafe {
        pechochar(WINDOW, ch);
    }
}

pub fn addchar(ch: i8) {
    unsafe {
        waddch(WINDOW, ch);
    }
}

pub fn refreshw() {
    unsafe {
        wrefresh(WINDOW);
    }
}

pub fn clearw() -> i32 {
    unsafe { wclear(WINDOW) }
}

pub fn get_key_curses() -> i32 {
    unsafe { wgetch(WINDOW) }
}

pub fn delete_char() {
    unsafe {
        delch();
    }
}

pub fn delete_str(limit: i32) {
    let mut x = 0;
    let mut y = 0;
    get_cursor_pos(&mut x, &mut y);
    for i in (limit..x).rev() {
        set_cursor_pos(i, y);
        delete_char();
    }
}

pub fn get_cursor_pos(x: &mut i32, y: &mut i32) {
    unsafe {
        *x = getcurx(WINDOW);
        *y = getcury(WINDOW);
    }
}

pub fn get_cursor_x() -> i32 {
    unsafe { getcurx(WINDOW) }
}

pub fn set_cursor_pos(x: i32, y: i32) {
    unsafe {
        wmove(WINDOW, y, x);
    }
}

pub fn cursor_to_newline() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    get_cursor_pos(&mut x, &mut y);
    set_cursor_pos(0, y + 1);
}

pub fn get_window_size(x: &mut i32, y: &mut i32) {
    unsafe {
        *x = getmaxx(WINDOW);
        *y = getmaxy(WINDOW);
    }
}

pub fn get_mouse_event(event: &mut MEVENT) -> i32 {
    unsafe { getmouse(event) }
}

pub fn attron(attrs: i32) {
    unsafe {
        wattron(WINDOW, attrs);
    }
}

pub fn attroff(attrs: i32) {
    unsafe {
        wattroff(WINDOW, attrs);
    }
}

pub fn init_color_pair(pair: i16, fg: i16, bg: i16) -> i32 {
    unsafe { init_pair(pair, fg, bg) }
}
