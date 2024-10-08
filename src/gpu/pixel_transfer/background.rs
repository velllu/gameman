use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{BGP, LCDC, LY, SCX, SCY},
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData, Priority},
};

use super::{bools_to_color, vuza_gate, Layer, EMPTY_SLICE};

pub(crate) struct BackgroundLayer {
    lcdc_3: bool,
    tile_id: u8,
    tile_data_low: u16,
    tile_data_high: u16,

    // When scrolling the background layer with SCX, if SCX is not a multiple of 8, we
    // sometimes need to break multiple tiles in more parts and render them 8 pixels at a
    // time. These contain the data of the tiles we cut off
    leftover_low: u8,
    leftover_high: u8,
}

impl BackgroundLayer {
    pub(crate) fn new() -> Self {
        Self {
            lcdc_3: false,
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            leftover_low: 0,
            leftover_high: 0,
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

        let tile_data = bus.read(address);

        // We need to invert SCX otherwise it scrolls in the wrong direction
        let inverted_scx = 8 - (bus.read(SCX) % 8);
        let tile_data: u16 = (tile_data as u16).rotate_right(inverted_scx as u32);

        match is_high_part {
            false => self.tile_data_low = tile_data,
            true => self.tile_data_high = tile_data,
        }
    }

    fn push_pixels(&mut self, gpu: &Gpu, bus: &Bus) -> Vec<PixelData> {
        if !self.is_layer_enabled(bus) {
            return EMPTY_SLICE.into();
        }

        if gpu.number_of_slices_pushed == 0 {
            // We only need to "shift" for the first slice, we don't need to shift every
            // single tile otherwise the tiles won't look continous
            self.leftover_low = (self.tile_data_low >> 8) as u8;
            self.leftover_high = (self.tile_data_high >> 8) as u8;

            return vec![];
        }

        // We chain together the slices
        self.tile_data_low |= self.leftover_low as u16;
        self.tile_data_high |= self.leftover_high as u16;

        let mut slice = bytes_to_slice(
            (self.tile_data_low & 0xFF) as u8,
            (self.tile_data_high & 0xFF) as u8,
        );

        // The leftover bytes are just the parts of the slice we haven't rendered yet
        self.leftover_low = (self.tile_data_low >> 8) as u8;
        self.leftover_high = (self.tile_data_high >> 8) as u8;

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
