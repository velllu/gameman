use crate::{common::Bit, consts::gpu::LCDC, GameBoy};

use super::Color;

pub struct Line {
    pub colors: [Color; 8],
}

impl Line {
    /// Returns an all-white tile
    pub const fn new_blank() -> Self {
        Self {
            colors: [Color::Light; 8],
        }
    }

    /// # Description
    /// To draw a line we need two numbers, here's an example:
    /// ```no_rust
    /// 0   1   1   1   1   0   1   0  +
    /// 1   0   1   1   1   1   1   0  =
    /// 10  01  11  11  11  10  11  00
    /// ```
    /// where
    /// - 00 means Light,
    /// - 01 means MediumlyLight,
    /// - 10 means MediumlyDark,
    /// - 11 means Dark
    ///
    /// # Parameters and Panics
    /// - *line*: This is the row we need to change, can go from 0 to 7, it will crash if
    /// it's more then that.
    fn draw_line(&mut self, num1: u8, num2: u8) {
        for bit_offset in 0..=7 {
            // We take the 7th bit first, because I want `Tile.color` to start from the
            // leftmost bit
            let num1_bit = num1.get_bit(7 - bit_offset);
            let num2_bit = num2.get_bit(7 - bit_offset);

            self.colors[bit_offset as usize] = match (num1_bit, num2_bit) {
                (false, false) => Color::Light,
                (false, true) => Color::MediumlyLight,
                (true, false) => Color::MediumlyDark,
                (true, true) => Color::Dark,
            };
        }
    }
}

impl GameBoy {
    pub(crate) fn get_line(&self, tile_number: u8, y: u16) -> Line {
        let mut tile = Line::new_blank();

        // When the gameboy converts from u8 to u16, in this case, it adds 0s on the
        // right instead of the left, so `0xF8` becomes `0xF800` instead of `0x00F8` as
        // one might expect
        let tile_number: u16 = (tile_number as u16) << 4;

        let tile_data_address: u16 = match self.bus[LCDC].get_bit(4) {
            false => 0x8800,
            true => 0x8000,
        };

        let low = self.bus[tile_data_address + tile_number + y * 2];
        let high = self.bus[tile_data_address + tile_number + y * 2 + 1];

        tile.draw_line(high, low);
        tile
    }

    pub(crate) fn draw_line(&mut self, line: &Line, x: usize, y: usize) {
        for (i, pixel) in line.colors.iter().enumerate() {
            self.gpu.screen[y][x + i] = *pixel;
        }
    }
}
