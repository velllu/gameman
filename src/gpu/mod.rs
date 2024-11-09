mod blanks;
mod oam_search;
pub(crate) mod pixel_transfer;

use pixel_transfer::{sprite::SpriteData, Layers};

use crate::{
    bus::Bus,
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::LY,
    },
};

use self::pixel_transfer::PixelTransferState;

impl Gpu {
    fn switch_when_ticks(&mut self, ticks: u16, new_state: GpuState) {
        if self.ticks >= ticks {
            self.state = new_state;
            self.ticks = 0;
        } else {
            self.ticks += 1;
        }
    }

    pub(crate) fn tick(&mut self, layers: &mut Layers, bus: &mut Bus) {
        match self.state {
            GpuState::OamSearch => self.oam_search(bus),
            GpuState::PixelTransfer => self.pixel_transfer(layers, bus),
            GpuState::HBlank => self.hblank(layers, bus),
            GpuState::VBlank => self.vblank(layers, bus),
        }

        bus.write(LY, self.y);
    }
}

pub struct Gpu {
    pub screen: [[Color; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
    pub ticks: u16,
    pub state: GpuState,
    pub x: u8,
    pub y: u8,

    fifo: Vec<PixelData>,

    /// This is filled during OAM Search
    sprites: Vec<SpriteData>,

    /// The GPU has 4 states, one of which is pixel transfer, which also has many states
    pixel_transfer_state: PixelTransferState,

    /// The pixel transfer states happen in 2 dots, this is true whenever we are currently
    /// executing the first one, and false when execute the second one, since the gpu code
    /// is decoupled from the layers, the layer don't need to track this, it's handled by
    /// the `pixel_transfer/mod.rs` file
    is_pixel_transfer_first_call: bool,

    /// The first slice of pixels is always cut off, this boolean keeps track of wheter or
    /// not it has been dumped
    dump_slice: bool,

    /// The number of slices that have been pushed without counting the X Scrolling
    number_of_slices_pushed: u8,

    /// `number_of_slices_pushed` * 8
    virtual_x: u8,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            screen: [[Color::Light; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
            ticks: 0,
            state: GpuState::OamSearch,
            x: 0,
            y: 0,
            fifo: Vec::new(),
            sprites: Vec::new(),
            pixel_transfer_state: PixelTransferState::GetTile,
            is_pixel_transfer_first_call: true,
            dump_slice: true,
            number_of_slices_pushed: 0,
            virtual_x: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Light = 0,
    MediumlyLight = 1,
    MediumlyDark = 2,
    Dark = 3,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GpuState {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

/// The data of each pixel in the fifo
#[derive(Clone, Copy, Debug)]
pub(crate) struct PixelData {
    pub(crate) color: Color,

    /// This dictates the order of pixel drawing, check the code for sprite mixing in
    /// `pixel_transfer/mod.rs` for more info
    pub(crate) z_index: u8,
}

#[derive(Clone, Copy)]
pub(crate) enum Priority {
    AlwaysAbove,

    /// When the underlaying slice shows through the light pixels of the above slice
    TransparentLight,

    /// When the above slice's pixels are drawn only above light pixels
    AboveLight,
}
