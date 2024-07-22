use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{LCDC, WX, WY},
    gpu::{Color, Gpu, PixelData},
};

use super::{bytes_to_slice, vuza_gate, Layer};

pub(crate) struct WindowLayer {
    lcdc_6: bool,
    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl WindowLayer {
    pub(crate) fn new() -> Self {
        Self {
            lcdc_6: false,
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Layer for WindowLayer {
    fn is_layer_enabled(&self, bus: &Bus) -> bool {
        if bus[LCDC].get_bit(5) {
            return true;
        }

        false
    }

    fn mix_with_layer_below(&self) -> bool {
        true
    }

    fn get_tile_step_1(&mut self, _gpu: &Gpu, bus: &Bus) {
        self.lcdc_6 = bus[LCDC].get_bit(6);
    }

    fn get_tile_step_2(&mut self, gpu: &Gpu, bus: &Bus) {
        // This is `-7` because WX as an offset of 7, anything below that will not be
        // rendered
        let window_x: i32 = gpu.virtual_x as i32 - (bus[WX] as i32 - 7);
        let window_y: i32 = gpu.y as i32 - bus[WY] as i32;

        if window_x.is_negative() || window_y.is_negative() {
            self.tile_id = 0;
            return;
        }

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#bg-fetcher
        let address = 0b10011 << 11
            | (self.lcdc_6 as u16) << 10
            | (window_y as u16 / 8) << 5
            | window_x as u16 / 8;

        self.tile_id = bus[address];
    }

    fn get_tile_data(&mut self, is_high_part: bool, gpu: &Gpu, bus: &Bus) {
        let window_y: i32 = gpu.y as i32 - bus[WY] as i32;

        if window_y.is_negative() {
            self.tile_data_low = 0;
            self.tile_data_high = 0;
            return;
        }

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#get-tile-row-low
        let address = 0b100 << 13
            | vuza_gate(bus[LCDC], self.tile_id) << 12
            | (self.tile_id as u16) << 4
            | (window_y as u16 % 8) << 1
            | is_high_part as u16;

        match is_high_part {
            false => self.tile_data_low = bus[address],
            true => self.tile_data_high = bus[address],
        }
    }

    fn push_pixels(&mut self, _gpu: &Gpu, bus: &Bus) -> Vec<PixelData> {
        if !self.is_layer_enabled(bus) {
            // Return 8 blank pixels
            return vec![
                PixelData {
                    color: Color::Light,
                };
                8
            ];
        }

        bytes_to_slice(self.tile_data_low, self.tile_data_high)
    }
}
