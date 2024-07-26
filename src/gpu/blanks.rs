use crate::GameBoy;

use super::GpuState;

impl GameBoy {
    pub(super) fn hblank(&mut self) {
        if self.gpu.ticks == 0 {
            self.gpu.x = 0;
            self.gpu.fifo.clear();

            self.layers
                .iter_mut()
                .for_each(|layer| layer.at_hblank(&self.bus, &self.gpu));
        }

        self.switch_when_ticks(
            // This should be 376 - the duration of the pixel transfer, but pixel transfer is
            // hard-coded to 172 as of now
            376 - 172,
            if self.gpu.y == 144 {
                GpuState::VBlank
            } else {
                GpuState::PixelTransfer
            },
        );
    }

    pub(super) fn vblank(&mut self) {
        /// Amount of ticks needed to render a vblank line
        const VBLANK_LINE_TICKS: u16 = 456;

        if self.gpu.ticks == 0 {
            self.layers
                .iter_mut()
                .for_each(|layer| layer.at_vblank(&self.bus, &self.gpu));
        }

        // After every line
        if self.gpu.ticks % VBLANK_LINE_TICKS == 0 {
            self.gpu.y += 1;
        }

        // There are 10 lines of vblank
        self.switch_when_ticks(VBLANK_LINE_TICKS * 10, GpuState::OamSearch);
    }
}
