use crate::{
    common::Bit,
    consts::gpu::{OBP0, OBP1},
    GameBoy,
};

use super::{sprite_parser::Palette, tile_parser::Line, Color};

pub(crate) fn bools_to_color(bool1: bool, bool2: bool) -> Color {
    match (bool1, bool2) {
        (false, false) => Color::Light,
        (false, true) => Color::MediumlyLight,
        (true, false) => Color::MediumlyDark,
        (true, true) => Color::Dark,
    }
}

impl GameBoy {
    pub(crate) fn apply_palette_to_sprite(&self, line: &mut Line, palette: &Palette) {
        let palette = match palette {
            Palette::OBP0 => self.bus[OBP0],
            Palette::OBP1 => self.bus[OBP1],
        };

        let id_1 = bools_to_color(palette.get_bit(3), palette.get_bit(2));
        let id_2 = bools_to_color(palette.get_bit(5), palette.get_bit(4));
        let id_3 = bools_to_color(palette.get_bit(7), palette.get_bit(6));

        for pixel in line.colors.iter_mut() {
            *pixel = match pixel {
                Color::MediumlyLight => id_1,
                Color::MediumlyDark => id_2,
                Color::Dark => id_3,

                // Light will not change because light just means transparent in a sprite
                Color::Light => Color::Light,
            }
        }
    }
}
