use crate::{bus::Bus, common::Bit, consts::gpu::{LCDC, WY}};

use super::Layer;

struct WindowLayer {
    lcdc_6: bool,
}

impl Layer for WindowLayer {
    fn is_layer_enabled(&self, _bus: &Bus) -> bool {
        true
    }

    fn get_tile_step_1(&mut self, bus: &Bus) {
        self.lcdc_6 = bus[LCDC].get_bit(6);
    }

    fn get_tile_step_2(&mut self, _bus: &Bus) {
        let address = 0b10011 << 11 | (self.lcdc_6 as u16) << 10 | (bus[WY] as u16 / 8) << 5 | 
    }

    fn get_tile_data(&mut self, _is_high_part: bool, _bus: &Bus) {}

    fn push_pixels(&mut self, _bus: &Bus) -> Vec<crate::gpu::PixelData> {
        todo!()
    }

    fn at_new_scanline(&mut self, _fifo: &mut Vec<crate::gpu::PixelData>) {}
}
