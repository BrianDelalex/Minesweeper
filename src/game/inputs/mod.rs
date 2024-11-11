pub mod actions;
use crate::ncurses::*;

use actions::{Actions, Click};

use super::Game;

pub fn handle_input(code: u32, game: &Game) -> Actions {
    if code == KEY_MOUSE {
        return handle_mouse_event(game);
    }
    Actions::INVALID
}

fn handle_mouse_event(game: &Game) -> Actions {
    let mut event = MEVENT::default();
    if get_mouse_event(&mut event) == 0 {
        if event.bstate & BUTTON1_CLICKED() != 0 {
            return Actions::LEFT_CLICK(Click {
                x: event.x - game.x_offset as i32,
                y: event.y - game.y_offset as i32,
            });
        } else if event.bstate & BUTTON3_CLICKED() != 0 {
            return Actions::RIGHT_CLICK(Click {
                x: event.x - game.x_offset as i32,
                y: event.y - game.y_offset as i32,
            });
        } else {
            return Actions::INVALID;
        }
    } else {
        panic!("get_mouse_event error.");
    }
}
