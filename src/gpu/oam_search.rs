use crate::{common::Bit, GameBoy};

use super::{
    pixel_transfer::sprite::{Palette, SpriteData},
    GpuState, Priority,
};

impl GameBoy {
    pub(super) fn oam_search(&mut self) {
        self.gpu.has_just_entered_oam_scan = self.gpu.ticks == 0;

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
        let y = self.bus[address - 3];
        let x = self.bus[address - 2];
        let tile_number = self.bus[address - 1];
        let flags = self.bus[address];

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
