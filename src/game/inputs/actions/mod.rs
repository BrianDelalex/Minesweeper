pub struct Click {
    pub x: i32,
    pub y: i32,
}

pub enum Actions {
    LeftClick(Click),
    RightClick(Click),
    Exit,
    Invalid,
}
