use crate::{common::Bit, consts::gpu::LCDC, GameBoy};

use super::Color;

pub struct Tile {
    pub colors: [[Color; 8]; 8],
}

impl Tile {
    /// Returns an all-white tile
    pub const fn new_blank() -> Self {
        Self {
            colors: [[Color::Light; 8]; 8],
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
    fn draw_line(&mut self, num1: u8, num2: u8, line: usize) {
        for bit_offset in 0..=7 {
            // We take the 7th bit first, because I want `Tile.color` to start from the
            // leftmost bit
            let num1_bit = num1.get_bit(7 - bit_offset);
            let num2_bit = num2.get_bit(7 - bit_offset);

            self.colors[line][bit_offset as usize] = match (num1_bit, num2_bit) {
                (false, false) => Color::Light,
                (false, true) => Color::MediumlyLight,
                (true, false) => Color::MediumlyDark,
                (true, true) => Color::Dark,
            };
        }
    }
}

impl GameBoy {
    pub(crate) fn draw_tile(&mut self, tile_number: u8, x_offset: usize, y_offset: usize) {
        let mut tile = Tile::new_blank();

        // When the gameboy converts from u8 to u16, in this case, it adds 0s on the
        // right instead of the left, so `0xF8` becomes `0xF800` instead of `0x00F8` as
        // one might expect
        let tile_number: u16 = (tile_number as u16) << 4;

        let tile_data_address: u16 = match self.bus[LCDC].get_bit(4) {
            false => 0x8800,
            true => 0x8000,
        };

        for i in (0..16).step_by(2) {
            let low = self.bus[tile_data_address + tile_number + i];
            let high = self.bus[tile_data_address + tile_number + i + 1];

            tile.draw_line(high, low, (i / 2) as usize);
        }

        for (x_coordinate, x) in tile.colors.iter().enumerate() {
            for (y_coordinate, y) in x.iter().enumerate() {
                self.gpu.screen[x_coordinate + x_offset][y_coordinate + y_offset] = *y;
            }
        }
    }
}
