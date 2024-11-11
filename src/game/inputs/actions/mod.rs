pub struct Click {
    pub x: i32,
    pub y: i32,
}

pub enum Actions {
    LEFT_CLICK(Click),
    RIGHT_CLICK(Click),
    EXIT,
    INVALID,
}
