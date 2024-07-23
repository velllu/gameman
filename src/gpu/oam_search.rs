use crate::{common::Bit, GameBoy};

use super::{
    pixel_transfer::sprite::{Palette, Priority, SpriteData},
    GpuState,
};

impl GameBoy {
    pub(super) fn oam_search(&mut self) {
        if self.gpu.ticks == 0 {
            self.gpu.y = 0;
            self.gpu.sprites.clear();

            for i in (0xFE00..0xFE9C).step_by(4) {
                self.gpu.sprites.push(self.get_sprite_data(i));
            }
        }

        self.switch_when_ticks(80, GpuState::PixelTransfer);
    }

    fn get_sprite_data(&self, address: u16) -> SpriteData {
        let y = self.bus[address];
        let x = self.bus[address + 1];
        let tile_number = self.bus[address + 2];
        let flags = self.bus[address + 3];

        SpriteData {
            y,
            x,
            tile_number,
            priority: match flags.get_bit(7) {
                false => Priority::AboveLightColor,
                true => Priority::AlwaysAbove,
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
