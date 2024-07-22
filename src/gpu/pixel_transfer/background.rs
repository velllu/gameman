use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{LCDC, LY, SCX, SCY},
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData},
};

use super::{vuza_gate, Layer};

pub(crate) struct BackgroundLayer {
    lcdc_3: bool,
    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl BackgroundLayer {
    pub(crate) fn new() -> Self {
        Self {
            lcdc_3: false,
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Layer for BackgroundLayer {
    fn is_layer_enabled(&self, bus: &Bus) -> bool {
        if bus[LCDC].get_bit(0) {
            return true;
        }

        false
    }

    fn mix_with_layer_below(&self) -> bool {
        true
    }

    fn get_tile_step_1(&mut self, _gpu: &Gpu, bus: &Bus) {
        self.lcdc_3 = bus[LCDC].get_bit(3);
    }

    fn get_tile_step_2(&mut self, gpu: &Gpu, bus: &Bus) {
        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#bg-fetcher
        let address = 0b10011 << 11
            | (self.lcdc_3 as u16) << 10
            | (bus[LY].wrapping_add(bus[SCY]) as u16 / 8) << 5
            | (gpu.virtual_x.wrapping_add(bus[SCX])) as u16 / 8;

        self.tile_id = bus[address];
    }

    fn get_tile_data(&mut self, is_high_part: bool, _gpu: &Gpu, bus: &Bus) {
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

    fn push_pixels(&mut self, gpu: &Gpu, bus: &Bus) -> Vec<PixelData> {
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

        // X Scrolling, we remove pixels from the first slice of the line so all the next
        // tiles will be at an offset. It's important to clear the fifo when the line has
        // been rendered otherwise the offset could affect the next line too
        if gpu.number_of_slices_pushed == 1 {
            for _ in 0..(bus[SCX] % 8) {
                slice.pop();
            }
        }

        slice
    }
}
