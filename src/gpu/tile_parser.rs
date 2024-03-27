use crate::{common::Bit, consts::gpu::LCDC, GameBoy};

use super::{palette::bools_to_color, Color};

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

            self.colors[bit_offset as usize] = bools_to_color(num1_bit, num2_bit);
        }
    }
}

impl GameBoy {
    pub(crate) fn get_line(&self, tile_number: u8, y: u16) -> Line {
        let mut tile = Line::new_blank();

        // To calculate the address to fetch we calculate
        // `TileDataRegion + (TileNumber << 4)`

        // LCDC.4 is used to set the tile data region, when it's zero, the region is
        // between 8800-97FF and 8000-8FFF. So, if the tile number is 10, and LCDC.4 is
        // set, it will go look at 0x8100, but if LCDC.4 is not set, it will go look at
        // 0x9100. Both these region share a common ground
        let tile_data_address: u16 = match self.bus[LCDC].get_bit(4) {
            // If the tile number is above 0x80, it means that we are in the common
            // so we can just start counting from 0x8000, otherwise, we'll start from
            // 0x9000
            false if tile_number < 0x80 => 0x9000,
            false => 0x8000,

            true => 0x8000,
        };

        let tile_number: u16 = (tile_number as u16) << 4;

        let low = self.bus[tile_data_address + tile_number + y * 2];
        let high = self.bus[tile_data_address + tile_number + y * 2 + 1];

        tile.draw_line(high, low);
        tile
    }

    pub(crate) fn get_line_rotation(
        &self,
        tile_number: u8,
        y: u16,
        x_flip: bool,
        y_flip: bool,
    ) -> Line {
        let mut line = self.get_line(
            tile_number,
            match y_flip {
                false => y,
                true => 7 - y,
            },
        );

        if x_flip {
            line.colors.reverse();
        }

        line
    }

    pub(crate) fn draw_line(&mut self, line: &Line, x: usize, y: usize) {
        for (i, pixel) in line.colors.iter().enumerate() {
            self.gpu.screen[y][x + i] = *pixel;
        }
    }
}
