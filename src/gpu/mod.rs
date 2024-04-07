pub(crate) mod palette;
pub(crate) mod sprite_parser;
pub mod states;
pub mod tile_parser;
pub(crate) mod window;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Dark,
    MediumlyDark,
    MediumlyLight,
    Light,
}
