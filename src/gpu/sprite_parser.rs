use crate::{common::Bit, GameBoy};

use super::{tile_parser::Line, Color};

#[derive(Clone, Copy)]
pub(crate) enum SpriteHeight {
    Short = 8,
    Tall = 16,
}

#[derive(PartialEq, Eq)]
pub(crate) enum Priority {
    AlwaysAbove,
    AboveLightColor,
}

pub(crate) enum Palette {
    OBP0,
    OBP1,
}

pub(crate) struct SpriteData {
    pub(crate) y: u8,
    pub(crate) x: u8,
    pub(crate) tile_number: u8,

    // Flags
    pub(crate) priority: Priority,
    pub(crate) palette: Palette,
    pub(crate) x_flip: bool,
    pub(crate) y_flip: bool,
}

impl GameBoy {
    pub(crate) fn get_sprite_data(&self, address: u16) -> SpriteData {
        let y = self.bus[address];
        let x = self.bus[address + 1];
        let tile_number = self.bus[address + 2];
        let flags = self.bus[address + 3];

        SpriteData {
            y,
            x,
            tile_number,
            priority: match flags.get_bit(7) {
                false => Priority::AboveLightColor,
                true => Priority::AlwaysAbove,
            },
            palette: match flags.get_bit(4) {
                false => Palette::OBP0,
                true => Palette::OBP1,
            },
            x_flip: flags.get_bit(5),
            y_flip: flags.get_bit(6),
        }
    }

    pub(crate) fn get_sprite_fifo(
        &self,
        x: u8,
        y: u8,
        sprite_height: &SpriteHeight,
    ) -> Option<(Line, &SpriteData)> {
        let mut sprite_fifo: Option<(Line, &SpriteData)> = None;

        for sprite in &self.gpu.sprites {
            if sprite.y < 16 || sprite.x < 8 {
                continue;
            }

            let sprite_y = sprite.y - 16;
            let sprite_x = sprite.x - 8;

            // We check if there is any sprite that is on the same x axis as our "cursor"
            let x_condition = sprite_x == x;

            // And we check if the sprite is contained in the range of the sprite's height
            let y_condition = ((sprite_y)..(sprite_y + *sprite_height as u8)).contains(&y);

            // This is the line of the sprite we are currently processing
            let current_line = y % *sprite_height as u8;

            // When the sprite is 8x8 we can just use the actual tile number, but when the
            // sprite is 8x16, the higher part of it needs to be ANDed with `0xFE` and the
            // lower part needs to be ORed with `0x01`, when y flipping is on, we do the
            // opposite
            let tile_number = match (sprite_height, sprite.y_flip) {
                (&SpriteHeight::Short, _) => sprite.tile_number,

                // High part
                (&SpriteHeight::Tall, false) if current_line >= 8 => sprite.tile_number & 0xFE,
                (&SpriteHeight::Tall, true) if current_line >= 8 => sprite.tile_number | 0x01,

                // Low part
                (&SpriteHeight::Tall, false) => sprite.tile_number | 0x01,
                (&SpriteHeight::Tall, true) => sprite.tile_number & 0xFE,
            };

            if x_condition && y_condition {
                sprite_fifo = Some((
                    self.get_line_from_tile_number_with_rotation(
                        tile_number,
                        y % 8,
                        sprite.x_flip,
                        sprite.y_flip,
                    ),
                    sprite,
                ));
            }
        }

        sprite_fifo
    }
}

impl Line {
    pub(crate) fn mix_with_background_tile(&mut self, background_tile: &Line, priority: &Priority) {
        if *priority == Priority::AlwaysAbove {
            return;
        }

        for (sprite_pixel, bg_pixel) in self.colors.iter_mut().zip(background_tile.colors) {
            if *sprite_pixel == Color::Light {
                *sprite_pixel = bg_pixel;
            }
        }
    }
}
