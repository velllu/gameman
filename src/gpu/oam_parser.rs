use crate::{common::Bit, GameBoy};

pub(crate) enum Priority {
    AlwaysAbove,
    AboveLightColor,
}

pub(crate) enum Palette {
    _0BP0,
    _0BP1,
}

pub(crate) struct SpriteData {
    pub(crate) y: u8,
    pub(crate) x: u8,
    pub(crate) tile_number: u8,

    // Flags
    pub(crate) priority: Priority,
    pub(crate) palette: Palette,
    pub(crate) x_flip: bool,
    pub(crate) y_flip: bool,
}

impl GameBoy {
    pub(crate) fn get_sprite_data(&self, address: u16) -> SpriteData {
        let y = self.bus[address];
        let x = self.bus[address + 1];
        let tile_number = self.bus[address + 2];
        let flags = self.bus[address + 3];

        SpriteData {
            y,
            x,
            tile_number,
            priority: match flags.get_bit(7) {
                false => Priority::AlwaysAbove,
                true => Priority::AboveLightColor,
            },
            palette: match flags.get_bit(4) {
                false => Palette::_0BP0,
                true => Palette::_0BP1,
            },
            x_flip: flags.get_bit(5),
            y_flip: flags.get_bit(6),
        }
    }
}
