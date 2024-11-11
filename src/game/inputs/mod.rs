pub mod actions;
use crate::ncurses::*;

use actions::{Actions, Click};

pub fn handle_input(code: u32) -> Actions {
    if code == KEY_MOUSE {
        return handle_mouse_event();
    }
    Actions::INVALID
}

fn handle_mouse_event() -> Actions {
    let mut event = MEVENT::default();
    if get_mouse_event(&mut event) == 0 {
        if event.bstate & BUTTON1_CLICKED() != 0 {
            return Actions::LEFT_CLICK(Click {
                x: event.x,
                y: event.y,
            });
        } else if event.bstate & BUTTON3_CLICKED() != 0 {
            return Actions::RIGHT_CLICK(Click {
                x: event.x,
                y: event.y,
            });
        } else {
            return Actions::INVALID;
        }
    } else {
        panic!("get_mouse_event error.");
    }
}
