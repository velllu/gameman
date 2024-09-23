use crate::{
    common::Bit,
    consts::{cpu::IF, gpu::STAT},
    GameBoy,
};

use super::{
    pixel_transfer::sprite::{Palette, SpriteData},
    GpuState, Priority,
};

impl GameBoy {
    pub(super) fn oam_search(&mut self) {
        // Setting interrupts
        if self.gpu.ticks == 0 {
            let interrupt_flag = self.bus.read(IF);
            let stat = self.bus.read(STAT);

            // Stat interrupt. Stat.3 indicates OAM Search
            if stat.get_bit(5) {
                self.bus.write(IF, interrupt_flag | 0b00000010);
            }
        }

        if self.gpu.ticks == 0 {
            self.gpu.y = 0;
            self.gpu.sprites.clear();

            // We access sprites in reverse because the sprite with the lowest address has
            // the most priority
            for i in (0xFE00..0xFE9C).rev().step_by(4) {
                self.gpu.sprites.push(self.get_sprite_data(i));
            }
        }

        self.switch_when_ticks(80, GpuState::PixelTransfer);
    }

    fn get_sprite_data(&self, address: u16) -> SpriteData {
        let y = self.bus.read(address - 3);
        let x = self.bus.read(address - 2);
        let tile_number = self.bus.read(address - 1);
        let flags = self.bus.read(address);

        SpriteData {
            y,
            x,
            tile_number,
            priority: match flags.get_bit(7) {
                false => Priority::TransparentLight,
                true => Priority::AboveLight,
            },
            palette: match flags.get_bit(4) {
                false => Palette::OBP0,
                true => Palette::OBP1,
            },
            x_flip: flags.get_bit(5),
            y_flip: flags.get_bit(6),
        }
    }
}
