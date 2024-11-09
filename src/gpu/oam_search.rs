use crate::{
    bus::Bus,
    common::Bit,
    consts::{cpu::IF, gpu::STAT},
};

use super::{
    pixel_transfer::sprite::{Palette, SpriteData},
    Gpu, GpuState, Priority,
};

impl Gpu {
    pub(super) fn oam_search(&mut self, bus: &mut Bus) {
        // Setting interrupts
        if self.ticks == 0 {
            let interrupt_flag = bus.read(IF);
            let stat = bus.read(STAT);

            // Stat interrupt. Stat.3 indicates OAM Search
            if stat.get_bit(5) {
                bus.write(IF, interrupt_flag | 0b00000010);
            }
        }

        if self.ticks == 0 {
            self.y = 0;
            self.sprites.clear();

            // We access sprites in reverse because the sprite with the lowest address has
            // the most priority
            for i in (0xFE00..0xFE9C).rev().step_by(4) {
                self.sprites.push(bus.get_sprite_data(i));
            }
        }

        self.switch_when_ticks(80, GpuState::PixelTransfer);
    }
}

impl Bus {
    fn get_sprite_data(&self, address: u16) -> SpriteData {
        let y = self.read(address - 3);
        let x = self.read(address - 2);
        let tile_number = self.read(address - 1);
        let flags = self.read(address);

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
