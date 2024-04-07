use crate::{common::Bit, consts::gpu::LCDC, GameBoy};

impl GameBoy {
    pub(crate) fn can_render_window(&self) -> bool {
        if !self.bus[LCDC].get_bit(0) || !self.bus[LCDC].get_bit(5) {
            return false;
        }

        true
    }
}
