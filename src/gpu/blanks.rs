use crate::{
    bus::Bus,
    common::Bit,
    consts::{
        cpu::IF,
        gpu::{LYC, STAT},
    },
};

use super::{Gpu, GpuState, Layers};

impl Gpu {
    pub(super) fn hblank(&mut self, layers: &mut Layers, bus: &mut Bus) {
        if self.ticks == 0 {
            self.x = 0;
            self.y += 1;
            self.dump_slice = true;
            self.fifo.clear();

            layers
                .iter_mut()
                .for_each(|layer| layer.at_hblank(bus, self));
        }

        // Setting interrupts
        if self.ticks == 0 {
            let mut interrupt_flag = bus.read(IF);
            let stat = bus.read(STAT);

            // Stat interrupt. Stat.3 indicates HBlank
            if stat.get_bit(3) {
                interrupt_flag |= 0b00000010;
            }

            // Stat interrupt. We need to check for LYC == LY
            if stat.get_bit(6) && self.y == bus.read(LYC) {
                interrupt_flag |= 0b00000010;
            }

            bus.write(IF, interrupt_flag);
        }

        self.switch_when_ticks(
            // This should be 376 - the duration of the pixel transfer, but pixel transfer is
            // hard-coded to 172 as of now
            376 - 172,
            if self.y == 144 {
                GpuState::VBlank
            } else {
                GpuState::PixelTransfer
            },
        );
    }

    pub(super) fn vblank(&mut self, layers: &mut Layers, bus: &mut Bus) {
        /// Amount of ticks needed to render a vblank line
        const VBLANK_LINE_TICKS: u16 = 456;

        if self.ticks == 0 {
            layers
                .iter_mut()
                .for_each(|layer| layer.at_vblank(bus, self));
        }

        // After every line
        if self.ticks % VBLANK_LINE_TICKS == 0 {
            self.y += 1;
        }

        // Setting interrupts
        if self.ticks == 0 {
            let mut interrupt_flag = bus.read(IF);
            let stat = bus.read(STAT);

            // VBlank interrupt
            interrupt_flag |= 0b00000001;

            // Stat interrupt. Stat.3 indicates VBlank
            if stat.get_bit(4) {
                interrupt_flag |= 0b00000010;
            }

            // Stat interrupt. We need to check for LYC == LY
            if stat.get_bit(6) && self.y == bus.read(LYC) {
                interrupt_flag |= 0b00000010;
            }

            bus.write(IF, interrupt_flag);
        }

        // There are 10 lines of vblank
        self.switch_when_ticks(VBLANK_LINE_TICKS * 10, GpuState::OamSearch);
    }
}
