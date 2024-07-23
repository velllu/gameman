use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::LCDC,
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData},
};

use super::Layer;

pub(crate) struct SpriteLayer {
    sprite_to_draw: Option<SpriteData>,
    rendered_sprites: u8,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl SpriteLayer {
    pub(crate) fn new() -> Self {
        Self {
            sprite_to_draw: None,
            rendered_sprites: 0,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Layer for SpriteLayer {
    fn is_layer_enabled(&self, bus: &Bus) -> bool {
        if bus[LCDC].get_bit(1) {
            return true;
        }

        false
    }

    fn mix_with_layer_below(&self) -> bool {
        true
    }

    /// TODO: This probably does nothing but i don't know for sure
    fn get_tile_step_1(&mut self, _gpu: &Gpu, _bus: &Bus) {}

    fn get_tile_step_2(&mut self, gpu: &Gpu, _bus: &Bus) {
        self.sprite_to_draw = None;

        if gpu.x == 0 {
            self.rendered_sprites = 0;
        }

        for sprite in &gpu.sprites {
            if sprite.x < 16 || sprite.y < 16 {
                continue;
            }

            let sprite_x = sprite.x - 16;
            let sprite_y = sprite.y - 16;

            if (sprite_x..(sprite_x + 8)).contains(&gpu.x)
                && (sprite_y..(sprite_y + 8)).contains(&gpu.y)
            {
                self.sprite_to_draw = Some(*sprite);
            }
        }
    }

    fn get_tile_data(&mut self, is_high_part: bool, gpu: &Gpu, bus: &Bus) {
        let Some(sprite_to_draw) = self.sprite_to_draw else {
            return;
        };

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#get-tile-row-low
        let address = 0b1000 << 12
            | (sprite_to_draw.tile_number as u16) << 4
            | ((gpu.y.wrapping_sub(sprite_to_draw.y)) as u16 % 8) << 1
            | is_high_part as u16;

        match is_high_part {
            false => self.tile_data_low = bus[address],
            true => self.tile_data_high = bus[address],
        }
    }

    fn push_pixels(&mut self, _gpu: &Gpu, bus: &Bus) -> Vec<PixelData> {
        if !self.is_layer_enabled(bus)
            || self.sprite_to_draw.is_none()
            || self.rendered_sprites >= 10
        {
            // Return 8 blank pixels
            return vec![
                PixelData {
                    color: Color::Light,
                };
                8
            ];
        }

        self.rendered_sprites += 1;

        bytes_to_slice(self.tile_data_low, self.tile_data_high)
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
pub(crate) struct SpriteData {
    pub(crate) y: u8,
    pub(crate) x: u8,
    pub(crate) tile_number: u8,
    pub(crate) priority: Priority,
    pub(crate) palette: Palette,
    pub(crate) x_flip: bool,
    pub(crate) y_flip: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum Priority {
    AlwaysAbove,
    AboveLightColor,
}

#[derive(Clone, Copy)]
pub(crate) enum Palette {
    OBP0,
    OBP1,
}
