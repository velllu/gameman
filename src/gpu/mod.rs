pub(crate) mod palette;
pub(crate) mod sprite_parser;
pub(crate) mod states;
pub mod tile_parser;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Dark,
    MediumlyDark,
    MediumlyLight,
    Light,
}
