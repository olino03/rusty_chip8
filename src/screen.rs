use constants::{SCREEN_COLUMNS, SCREEN_ROWS};

pub struct Screen {
    // array for the set pixels (64 * 32 bits)
    pub pixels: [[bool; constants::SCREEN_COLUMNS]; constants::SCREEN_ROWS],
}

impl Screen {
    //initalize screen to 0
    pub fn new() {
        let pixels: [[bool; constants::SCREEN_COLUMNS]; constants::SCREEN_ROWS] =
            [[0; constants::SCREEN_COLUMNS]; constants::SCREEN_ROWS];
        Screen { pixels }
    }
}
