pub(crate) mod fifo;
pub(crate) mod states;
pub mod tile_parser;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Dark,
    MediumlyDark,
    MediumlyLight,
    Light,
}
