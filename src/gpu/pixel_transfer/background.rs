use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{LCDC, LY, SCX, SCY},
    gpu::{pixel_transfer::bytes_to_slice, Color, PixelData},
};

use super::Layer;

pub(crate) struct BackgroundLayer {
    lcdc_3: bool,
    number_of_slices_pushed: u8,
    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl BackgroundLayer {
    pub(crate) fn new() -> Self {
        Self {
            lcdc_3: false,
            number_of_slices_pushed: 1,
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Layer for BackgroundLayer {
    fn is_layer_enabled(&self, _bus: &Bus) -> bool {
        true
    }

    /// On the first dot the GPU gets LCDC.3
    fn get_tile_step_1(&mut self, bus: &Bus) {
        self.lcdc_3 = bus[LCDC].get_bit(3);
    }

    /// On the second dot the GPU calculates the tilemap address and fetches it
    fn get_tile_step_2(&mut self, bus: &Bus) {
        // TODO: Figure out why this is needed, it's driving me crazy
        if self.number_of_slices_pushed == 0 {
            return;
        }

        // This is where the X pointer would be if we always pushed 8 pixels at a
        // time (which happens when SCX is not a multiple of 8)
        let virtual_x = (self.number_of_slices_pushed - 1) * 8;
        println!("{}", virtual_x);

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#bg-fetcher
        let address = 0b10011 << 11
            | (self.lcdc_3 as u16) << 10
            | (bus[LY].wrapping_add(bus[SCY]) as u16 / 8) << 5
            | (virtual_x.wrapping_add(bus[SCX])) as u16 / 8;

        self.tile_id = bus[address];
    }

    fn get_tile_data(&mut self, is_high_part: bool, bus: &Bus) {
        /// Implementation of this gate:
        /// https://github.com/furrtek/DMG-CPU-Inside/blob/f0eda633eac24b51a8616ff782225d06fccbd81f/Schematics/25_VRAM_INTERFACE.png
        fn vuza_gate(x: u8, y: u8) -> u16 {
            !((x & 0x10) != 0 || (y & 0x80) != 0) as u16
        }

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#get-tile-row-low
        let address = 0b100 << 13
            | vuza_gate(bus[LCDC], self.tile_id) << 12
            | (self.tile_id as u16) << 4
            | (bus[LY].wrapping_add(bus[SCY]) as u16 % 8) << 1
            | is_high_part as u16;

        match is_high_part {
            false => self.tile_data_low = bus[address],
            true => self.tile_data_high = bus[address],
        }
    }

    fn push_pixels(&mut self, bus: &Bus) -> Vec<PixelData> {
        if !self.is_layer_enabled(bus) {
            // Return 8 blank pixels
            return vec![
                PixelData {
                    color: Color::Light,
                };
                8
            ];
        }

        let mut slice = bytes_to_slice(self.tile_data_low, self.tile_data_high);

        if self.number_of_slices_pushed == 0 {
            slice.clear(); // The first slice is always dumped for some reason
        }

        // X Scrolling, we remove pixels from the first slice of the line so all the next
        // tiles will be at an offset. It's important to clear the fifo when the line has
        // been rendered otherwise the offset could affect the next line too
        if self.number_of_slices_pushed == 1 {
            for _ in 0..(bus[SCX] % 8) {
                slice.pop();
            }
        }

        self.number_of_slices_pushed += 1;
        slice
    }

    fn at_new_scanline(&mut self, fifo: &mut Vec<PixelData>) {
        self.number_of_slices_pushed = 0;
        fifo.clear();
    }
}
