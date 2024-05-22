mod blanks;
mod oam_search;
mod pixel_transfer;

use crate::{
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::LY,
    },
    GameBoy,
};

use self::pixel_transfer::{PixelTransferData, PixelTransferState};

use super::fifo::Fifo;

impl GameBoy {
    fn switch_when_ticks(&mut self, ticks: u16, new_state: GpuState) {
        if self.gpu.ticks >= ticks {
            self.gpu.state = new_state;
            self.gpu.ticks = 0;
        } else {
            self.gpu.ticks += 1;
        }
    }

    pub(crate) fn tick(&mut self) {
        match self.gpu.state {
            GpuState::OamSearch => self.oam_search(),
            GpuState::PixelTransfer => self.pixel_transfer(),
            GpuState::HBlank => self.hblank(),
            GpuState::VBlank => self.vblank(),
        }

        self.bus[LY] = self.gpu.y;
    }
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            screen: [[Color::Light; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
            ticks: 0,
            state: GpuState::OamSearch,
            pixel_transfer_data: PixelTransferData {
                state: PixelTransferState::GetTile,
                lcdc_3: false,
                tile_id: 0,
                tile_data_low: 0,
                tile_data_high: 0,
                is_first_call: true,
            },
            x: 0,
            y: 0,
            background_fifo: Fifo::new(),
        }
    }
}

pub struct Gpu {
    pub screen: [[Color; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
    pub ticks: u16,
    pub state: GpuState,

    // Pixel transfer
    pixel_transfer_data: PixelTransferData,

    x: u8,
    y: u8,

    background_fifo: Fifo<PixelData, 16>,
}

#[derive(Clone, Copy)]
pub enum Color {
    Light = 0,
    MediumlyLight = 1,
    MediumlyDark = 2,
    Dark = 3,
}

pub enum GpuState {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

/// The data of each pixel in the fifo
#[derive(Clone, Copy)]
pub(crate) struct PixelData {
    color: Color,
}
