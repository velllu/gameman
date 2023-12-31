use crate::{common::Bit, GameBoy};

use super::{tile_parser::Line, Color};

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

    pub(crate) fn get_sprite_fifo(&self, x: u8, y: u8) -> Option<(Line, &SpriteData)> {
        let mut sprite_fifo: Option<(Line, &SpriteData)> = None;

        for sprite in &self.gpu.sprites {
            if sprite.y < 16 || sprite.x < 8 {
                continue;
            }

            let sprite_y = sprite.y - 16;
            let sprite_x = sprite.x - 8;

            // We check if there is any sprite that is on the same x axis as our "cursor"
            let x_condition = sprite_x == x;

            // and we check if we also are on the same y axis, however, a sprite is 8
            // pixel long, so we check if we are anywhere between row 1 to 8
            let y_condition = ((sprite_y)..(sprite_y + 8)).contains(&y);

            if x_condition && y_condition {
                sprite_fifo = Some((
                    self.get_line_rotation(
                        sprite.tile_number,
                        y as u16 % 8,
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
