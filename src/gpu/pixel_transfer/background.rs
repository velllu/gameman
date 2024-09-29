use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{BGP, LCDC, LY, SCX, SCY},
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData, Priority},
};

use super::{bools_to_color, vuza_gate, Layer};

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
        if bus.read(LCDC).get_bit(0) {
            return true;
        }

        false
    }

    fn mix_with_layer_below(&self) -> Priority {
        Priority::AlwaysAbove
    }

    fn get_tile_step_1(&mut self, _gpu: &Gpu, bus: &Bus) {
        self.lcdc_3 = bus.read(LCDC).get_bit(3);
    }

    fn get_tile_step_2(&mut self, gpu: &Gpu, bus: &Bus) {
        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#bg-fetcher
        let address = 0b10011 << 11
            | (self.lcdc_3 as u16) << 10
            | (bus.read(LY).wrapping_add(bus.read(SCY)) as u16 / 8) << 5
            | (gpu.virtual_x.wrapping_add(bus.read(SCX))) as u16 / 8;

        self.tile_id = bus.read(address);
    }

    fn get_tile_data(&mut self, is_high_part: bool, _gpu: &Gpu, bus: &Bus) {
        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#get-tile-row-low
        let address = 0b100 << 13
            | vuza_gate(bus.read(LCDC), self.tile_id) << 12
            | (self.tile_id as u16) << 4
            | (bus.read(LY).wrapping_add(bus.read(SCY)) as u16 % 8) << 1
            | is_high_part as u16;

        match is_high_part {
            false => self.tile_data_low = bus.read(address),
            true => self.tile_data_high = bus.read(address),
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
        if gpu.number_of_slices_pushed == 0 {
            for _ in 0..(bus.read(SCX) % 8) {
                slice.pop();
            }
        }

        // Palette coloring (https://gbdev.io/pandocs/Palettes.html)
        let palette = bus.read(BGP);
        let id_0 = bools_to_color(palette.get_bit(1), palette.get_bit(0));
        let id_1 = bools_to_color(palette.get_bit(3), palette.get_bit(2));
        let id_2 = bools_to_color(palette.get_bit(5), palette.get_bit(4));
        let id_3 = bools_to_color(palette.get_bit(7), palette.get_bit(6));

        for pixel_data in &mut slice {
            pixel_data.color = match pixel_data.color {
                Color::Light => id_0,
                Color::MediumlyLight => id_1,
                Color::MediumlyDark => id_2,
                Color::Dark => id_3,
            }
        }

        slice
    }
}
