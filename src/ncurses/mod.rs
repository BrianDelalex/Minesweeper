use std::ffi::CString;
use std::ptr;

use cty::c_char;

static mut WINDOW: *mut cty::c_void = ptr::null_mut();

pub const KEY_DONW: u32 = 258;
pub const KEY_UP: u32 = 259;
pub const KEY_LEFT: u32 = 260;
pub const KEY_RIGHT: u32 = 261;
pub const KEY_BACKSPACE: u32 = 263;
pub const KEYS_CTRL_LEFT: u32 = 560;
pub const KEYS_CTRL_RIGHT: u32 = 575;
pub const KEY_MOUSE: u32 = 409;

pub const BUTTON_CLICKED: u64 = 004;

fn mouse_mask(b: u64, m: u64) -> u64 {
    ((m) << (((b) - 1) * 5))
}

fn all_mouse_event() -> u64 {
    report_mouse_position() - 1
}

fn report_mouse_position() -> u64 {
    mouse_mask(6, 0010)
}

pub fn BUTTON1_PRESSED() -> u64 {
    mouse_mask(1, 002)
}

pub fn BUTTON1_CLICKED() -> u64 {
    mouse_mask(1, BUTTON_CLICKED)
}

pub fn BUTTON2_CLICKED() -> u64 {
    mouse_mask(2, BUTTON_CLICKED)
}

extern "C" {
    fn wrefresh(window: *mut cty::c_void);
    fn wgetch(window: *mut cty::c_void) -> i32;
    fn delch();
    fn pechochar(window: *mut cty::c_void, ch: c_char);
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
        println!("\033[?1003h");
    }
}

pub fn printw(text: &str) {
    unsafe {
        for c in String::from(text).chars() {
            echochar(c as i8);
        }
    }
}

pub fn printnlw(text: &str) {
    unsafe {
        printw(text);
    }
    echochar('\n' as i8);
}

pub fn echochar(ch: i8) {
    unsafe {
        pechochar(WINDOW, ch);
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
