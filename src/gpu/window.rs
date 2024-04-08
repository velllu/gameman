use crate::{
    common::Bit,
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::{LCDC, WX, WY},
    },
    GameBoy,
};

impl GameBoy {
    pub(crate) fn can_render_window(&self) -> bool {
        if !self.bus[LCDC].get_bit(0) || !self.bus[LCDC].get_bit(5) {
            return false;
        }

        true
    }

    /// Returns true if the window's WX and WY are inside the screen bounds
    pub(crate) fn is_window_visible(&self) -> bool {
        if !(0..(DISPLAY_SIZE_X + 7)).contains(&(self.bus[WX] as usize)) {
            return false;
        }

        if !(0..(DISPLAY_SIZE_Y)).contains(&(self.bus[WY] as usize)) {
            return false;
        }

        true
    }
}
