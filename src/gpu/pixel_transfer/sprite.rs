use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{LCDC, OBP0, OBP1},
    gpu::{pixel_transfer::bytes_to_slice, Color, Gpu, PixelData, Priority},
};

use super::{bools_to_color, Layer, EMPTY_SLICE};

pub(crate) struct SpriteLayer {
    sprite_to_draw: Option<SpriteData>,
    rendered_sprites: u8,
    tile_data_low: u16,
    tile_data_high: u16,

    // When a sprite is cut off because it's not on the 8x8 grid, the data that got cut
    // off is stored in these variables so we can push it out later
    leftover_palette: Palette,
    leftover_low: u8,
    leftover_high: u8,

    /// When a sprite wraps around the rightmost part of the display and appears on the
    /// left, this is the number of pixels we have to remove to make it look like it's
    /// smoothly appearing
    left_side_shift: u8,

    /// If the sprite we are rendering is as said above, halfway hblank and the first
    /// pixels of the left side
    is_sprite_left_side: bool,
}

impl SpriteLayer {
    pub(crate) fn new() -> Self {
        Self {
            sprite_to_draw: None,
            rendered_sprites: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            leftover_palette: Palette::OBP0,
            leftover_low: 0,
            leftover_high: 0,
            left_side_shift: 0,
            is_sprite_left_side: false,
        }
    }
}

impl Layer for SpriteLayer {
    fn is_layer_enabled(&self, bus: &Bus) -> bool {
        if bus.read(LCDC).get_bit(1) {
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

    fn get_tile_step_2(&mut self, gpu: &Gpu, bus: &Bus) {
        self.sprite_to_draw = None;

        // Sprites can either be 8 pixels high or 16 pixels high
        let sprite_height: u8 = match bus.read(LCDC).get_bit(2) {
            false => 8,
            true => 16,
        };

        // TODO: This is kinda ugly
        for sprite in &gpu.sprites {
            let mut sprite_to_draw = *sprite;

            if sprite.y < 16 {
                continue;
            }

            // After coordinates `0xF9` the sprite starts to wrap around the screen
            if sprite_to_draw.x >= 0xF9 {
                // we just handle it as if it was on coordinate 0, and we shift the pixels
                // later
                self.left_side_shift = 0xFF - sprite_to_draw.x + 1;
                self.is_sprite_left_side = true;
            }

            // TODO: I don't know why it's specifically 7 and not 8, but if I put 8 in
            // this it becomes very jittery
            let sprite_x = sprite_to_draw.x.saturating_sub(7);
            let sprite_y = sprite_to_draw.y - 16;

            if (sprite_x..(sprite_x.wrapping_add(8))).contains(&gpu.virtual_x)
                && (sprite_y..(sprite_y + sprite_height)).contains(&gpu.y)
            {
                // We are rendering the top of a tall sprite
                if (sprite_y..(sprite_y + 8)).contains(&gpu.y) && sprite_height == 16 {
                    render_top_tall_sprite(&mut sprite_to_draw);
                }

                // We are rendering the bottom of a tall sprite
                if ((sprite_y + 8)..(sprite_y + 16)).contains(&gpu.y) && sprite_height == 16 {
                    render_bottom_tall_sprite(&mut sprite_to_draw);
                }

                self.sprite_to_draw = Some(match self.sprite_to_draw {
                    Some(existing_sprite) => sprite_priority(existing_sprite, sprite_to_draw),
                    None => sprite_to_draw,
                });
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
            address ^= 0b0000_0000_0000_1110;
        }

        let mut tile_data = bus.read(address);

        // When horizontally flipping we have to reverse the read byte
        if sprite_to_draw.x_flip {
            tile_data = tile_data.reverse_bits();
        }

        let mut tile_data = tile_data as u16;

        // We remove the first pixels to make the illusion of the sprite being halfway
        // on the screen TODO: This does not work whenever there's two sprite next to each
        // other that are both wrapping on the screen on the same line
        if !self.is_sprite_left_side {
            tile_data <<= self.left_side_shift;
        }

        // This is the offset to make sprite that are not on a 8x8 grid render correctly.
        // So for example if a sprite is on coordinate "17", we need to move it 1 pixel to
        // the right then how it already is
        tile_data = tile_data.rotate_right(sprite_to_draw.x as u32 % 8);

        match is_high_part {
            false => self.tile_data_low = tile_data,
            true => self.tile_data_high = tile_data,
        };
    }

    fn push_pixels(&mut self, _gpu: &Gpu, bus: &Bus) -> Vec<PixelData> {
        if !self.is_layer_enabled(bus) || self.rendered_sprites > 10 {
            return EMPTY_SLICE.into();
        }

        // When we render a part of a sprite but not all of it and there's no sprite left
        // to render, we just push the other half of the sprite
        if self.sprite_to_draw.is_none() && (self.leftover_high != 0 || self.leftover_low != 0) {
            let mut leftover_slice = bytes_to_slice(
                (self.tile_data_low >> 8) as u8,
                (self.tile_data_high >> 8) as u8,
            );

            apply_palette_to_slice(&mut leftover_slice, self.leftover_palette, bus);

            self.leftover_low = 0;
            self.leftover_high = 0;

            self.rendered_sprites += 1;
            return leftover_slice;
        }

        let Some(sprite_to_draw) = self.sprite_to_draw else {
            return EMPTY_SLICE.into();
        };

        // When we render a part of a sprite but not all of it but we also have another
        // sprite to render on the same line, in that case we OR them together so the
        // other half of the sprite and the other sprite we have to render look seamless
        self.tile_data_low |= self.leftover_low as u16;
        self.tile_data_high |= self.leftover_high as u16;

        let mut slice = bytes_to_slice(
            (self.tile_data_low & 0xFF) as u8,
            (self.tile_data_high & 0xFF) as u8,
        );

        self.leftover_palette = sprite_to_draw.palette;
        apply_palette_to_slice(&mut slice, sprite_to_draw.palette, bus);

        // The leftover bytes are just the parts of the slice we haven't rendered yet
        self.leftover_low = (self.tile_data_low >> 8) as u8;
        self.leftover_high = (self.tile_data_high >> 8) as u8;

        self.rendered_sprites += 1;
        slice
    }

    fn at_hblank(&mut self, _bus: &Bus, _gpu: &Gpu) {
        self.rendered_sprites = 0;

        // This makes it so sprite don't reappear from the left side when they are clipped
        // on the right side of the screen
        self.leftover_low = 0;
        self.leftover_high = 0;
        self.tile_data_low = 0;
        self.tile_data_high = 0;
    }
}

/// Takes in a slice and colors it according to a palette
fn apply_palette_to_slice(slice: &mut Vec<PixelData>, palette: Palette, bus: &Bus) {
    let palette = match palette {
        Palette::OBP0 => bus.read(OBP0),
        Palette::OBP1 => bus.read(OBP1),
    };

    let id_1 = bools_to_color(palette.get_bit(3), palette.get_bit(2));
    let id_2 = bools_to_color(palette.get_bit(5), palette.get_bit(4));
    let id_3 = bools_to_color(palette.get_bit(7), palette.get_bit(6));

    for pixel_data in slice {
        // The Light pixels are transparent, so we give higher priority to every other
        // color
        if pixel_data.color != Color::Light {
            pixel_data.z_index = 2;
        }

        // Sprites can still show Light pixels by mapping another color to Light
        pixel_data.color = match pixel_data.color {
            Color::Light => Color::Light,
            Color::MediumlyLight => id_1,
            Color::MediumlyDark => id_2,
            Color::Dark => id_3,
        };
    }
}

/// This functions gets called before rendering the top of a 16 pixel high sprite
fn render_top_tall_sprite(sprite: &mut SpriteData) {
    // ... and when it is, the sprite's bottom bit must be set to 0, and 1 if y flipping
    sprite.tile_number = match sprite.y_flip {
        false => sprite.tile_number & 0b1111_1110,
        true => sprite.tile_number | 0b0000_0001,
    };
}

/// This functions gets called before rendering the bottom of a 16 pixel high sprite
fn render_bottom_tall_sprite(sprite: &mut SpriteData) {
    // ... and when it is, the sprite's bottom bit must be set to 1, and 0 if y flipping
    sprite.tile_number = match sprite.y_flip {
        false => sprite.tile_number | 0b0000_0001,
        true => sprite.tile_number & 0b1111_1110,
    };
}

/// Returns the sprite with the highest priority
fn sprite_priority(sprite1: SpriteData, sprite2: SpriteData) -> SpriteData {
    if sprite1.x < sprite2.x {
        return sprite1;
    }

    sprite2
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
