// i have yet to implement the actual screen
#![allow(unused)]

use crate::consts::display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y};

pub struct Screen {
    display: [[bool; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
}

impl Screen {
    pub(crate) fn new() -> Self {
        Self {
            display: [[false; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
        }
    }
}
