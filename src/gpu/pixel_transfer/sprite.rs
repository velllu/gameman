use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{LCDC, OBP0, OBP1},
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData, Priority},
};

use super::{bools_to_color, Layer};

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

    fn mix_with_layer_below(&self) -> Priority {
        if let Some(sprite_to_draw) = self.sprite_to_draw {
            return sprite_to_draw.priority;
        }

        Priority::TransparentLight
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
        let mut address = 0b1000 << 12
            | (sprite_to_draw.tile_number as u16) << 4
            | ((gpu.y.wrapping_sub(sprite_to_draw.y)) as u16 % 8) << 1
            | is_high_part as u16;

        // When vertically flipping we have to invert bits 1-3 of the address
        if sprite_to_draw.y_flip {
            address = address ^ 0b0000_0000_0000_1110;
        }

        let mut tile_data = bus[address];

        // When horizontally flipping we have to reverse the read byte
        if sprite_to_draw.x_flip {
            tile_data = tile_data.reverse_bits();
        }

        match is_high_part {
            false => self.tile_data_low = tile_data,
            true => self.tile_data_high = tile_data,
        };
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

        let sprite_to_draw = match self.sprite_to_draw {
            Some(sprite) => sprite,
            _ => unreachable!(),
        };

        let mut slice = bytes_to_slice(self.tile_data_low, self.tile_data_high);

        // Palette coloring (https://gbdev.io/pandocs/Palettes.html)
        let palette = match sprite_to_draw.palette {
            Palette::OBP0 => bus[OBP0],
            Palette::OBP1 => bus[OBP1],
        };

        let id_1 = bools_to_color(palette.get_bit(3), palette.get_bit(2));
        let id_2 = bools_to_color(palette.get_bit(5), palette.get_bit(4));
        let id_3 = bools_to_color(palette.get_bit(7), palette.get_bit(6));

        for pixel_data in &mut slice {
            pixel_data.color = match pixel_data.color {
                Color::MediumlyLight => id_1,
                Color::MediumlyDark => id_2,
                Color::Dark => id_3,

                Color::Light => Color::Light, // for sprites, light is transparent
            }
        }

        self.rendered_sprites += 1;
        slice
    }
}

#[derive(Clone, Copy)]
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
pub(crate) enum Palette {
    OBP0,
    OBP1,
}
