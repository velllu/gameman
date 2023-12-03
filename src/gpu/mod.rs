pub(crate) mod states;
pub mod tile_parser;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Dark,
    MediumlyDark,
    MediumlyLight,
    Light,
}
