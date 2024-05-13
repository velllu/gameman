use crate::GameBoy;

use super::GpuState;

impl GameBoy {
    pub(super) fn oam_search(&mut self) {
        if self.gpu.ticks == 0 {
            self.gpu.y = 0;
            self.gpu.background_fifo.clear();
        }

        self.switch_when_ticks(80, GpuState::PixelTransfer);
    }
}
