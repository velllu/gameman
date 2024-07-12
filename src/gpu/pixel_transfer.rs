#![allow(clippy::precedence)] // i think it's much less clear when formatting like clippy
                              // wants to

use crate::{
    common::Bit,
    consts::{
        display::DISPLAY_SIZE_X,
        gpu::{LCDC, SCX, SCY},
    },
    GameBoy,
};

use super::{Color, GpuState, PixelData};

#[derive(PartialEq, Debug)]
pub(super) enum PixelTransferState {
    GetTile,
    GetLowTileData,
    GetHighTileData,
    Sleep,
    PushPixels,
}

impl GameBoy {
    fn cycle_state(&mut self) {
        self.gpu.pixel_transfer_data.state = match self.gpu.pixel_transfer_data.state {
            PixelTransferState::GetTile => PixelTransferState::GetLowTileData,
            PixelTransferState::GetLowTileData => PixelTransferState::GetHighTileData,
            PixelTransferState::GetHighTileData => PixelTransferState::Sleep,
            PixelTransferState::Sleep => PixelTransferState::PushPixels,
            PixelTransferState::PushPixels => PixelTransferState::GetTile,
        };

        self.gpu.pixel_transfer_data.is_first_call = true;
    }

    /// Inverts the `is_first_call` field and returns the previous value, meant to be used
    /// in the pixel transfer states to check wheter to activate step 1 or step 2
    fn is_first_call(&mut self) -> bool {
        self.gpu.pixel_transfer_data.is_first_call = !self.gpu.pixel_transfer_data.is_first_call;
        !self.gpu.pixel_transfer_data.is_first_call
    }

    /// In this state the pixels are getting fetched and put into the screen, it has 5
    /// step, and except the fifth step, they last 2 dots
    /// 1. Get tile
    /// 2. Get low tile data
    /// 3. Get high tile data
    /// 4. Sleep
    /// 5. Pushing pixels
    /// This is a super helpful resource:
    /// https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md
    pub(super) fn pixel_transfer(&mut self) {
        match self.gpu.pixel_transfer_data.state {
            PixelTransferState::GetTile => self.get_tile(),
            PixelTransferState::GetLowTileData => self.get_tile_data(false),
            PixelTransferState::GetHighTileData => self.get_tile_data(true),
            PixelTransferState::Sleep => self.sleep(),
            PixelTransferState::PushPixels => self.push_pixels(),
        }

        // Pop one pixel and display it
        if let Some(pixel_data) = self.gpu.background_fifo.pop() {
            self.gpu.screen[self.gpu.y as usize][self.gpu.x as usize] = pixel_data.color;
            self.gpu.x += 1;
        }

        // TODO: Implement the OBJ penalty algorithm, relevant link:
        // https://gbdev.io/pandocs/Rendering.html

        if self.gpu.x == DISPLAY_SIZE_X as u8 {
            self.gpu.pixel_transfer_data.number_of_slices_pushed = 0;
            self.gpu.state = GpuState::HBlank;
            self.gpu.ticks = 0;
            self.gpu.x = 0;
            self.gpu.y += 1;

            return;
        }

        self.gpu.ticks += 1;
    }

    fn get_tile(&mut self) {
        match self.is_first_call() {
            // On the first dot the GPU gets LCDC.3
            true => {
                self.gpu.pixel_transfer_data.lcdc_3 = self.bus[LCDC].get_bit(3);
            }

            // On the second dot the GPU calculates the tilemap address and fetches it
            false => {
                // This is where the X pointer would be if we always pushed 8 pixels at a
                // time (which happens when SCX is not a multiple of 8)
                let virtual_x = (self.gpu.pixel_transfer_data.number_of_slices_pushed - 1) * 8;

                // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#bg-fetcher
                let address = 0b10011 << 11
                    | (self.gpu.pixel_transfer_data.lcdc_3 as u16) << 10
                    | (self.gpu.y.wrapping_add(self.bus[SCY]) as u16 / 8) << 5
                    | (virtual_x.wrapping_add(self.bus[SCX])) as u16 / 8;

                self.gpu.pixel_transfer_data.tile_id = self.bus[address];
                self.cycle_state();
            }
        }
    }

    fn get_tile_data(&mut self, is_high_part: bool) {
        /// Implementation of this gate:
        /// https://github.com/furrtek/DMG-CPU-Inside/blob/f0eda633eac24b51a8616ff782225d06fccbd81f/Schematics/25_VRAM_INTERFACE.png
        fn vuza_gate(x: u8, y: u8) -> u16 {
            !((x & 0x10) != 0 || (y & 0x80) != 0) as u16
        }

        // This instruction takes two ticks, cpus are so fast that we can just do it all
        // in the second tick
        if self.is_first_call() {
            return;
        }

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#get-tile-row-low
        let address = 0b100 << 13
            | vuza_gate(self.bus[LCDC], self.gpu.pixel_transfer_data.tile_id) << 12
            | (self.gpu.pixel_transfer_data.tile_id as u16) << 4
            | (self.gpu.y.wrapping_add(self.bus[SCY]) as u16 % 8) << 1
            | is_high_part as u16;

        match is_high_part {
            false => self.gpu.pixel_transfer_data.tile_data_low = self.bus[address],
            true => self.gpu.pixel_transfer_data.tile_data_high = self.bus[address],
        }

        self.cycle_state();
    }

    fn push_pixels(&mut self) {
        if !self.gpu.background_fifo.is_empty() {
            return;
        }

        self.gpu.background_fifo.append(&mut bytes_to_slice(
            self.gpu.pixel_transfer_data.tile_data_low,
            self.gpu.pixel_transfer_data.tile_data_high,
        ));

        if self.gpu.pixel_transfer_data.number_of_slices_pushed == 0 {
            self.gpu.background_fifo.clear();
        }

        if self.gpu.pixel_transfer_data.number_of_slices_pushed == 1 {
            for _ in 0..(self.bus[SCX] % 8) {
                self.gpu.background_fifo.pop();
            }
        }

        self.gpu.pixel_transfer_data.number_of_slices_pushed += 1;

        self.cycle_state();
    }

    fn sleep(&mut self) {
        if !self.is_first_call() {
            self.cycle_state();
        }
    }
}

/// This function builds the line of a tile by the two bytes that represent it. The two
/// bits from both bytes dictate the color of a single pixel
fn bytes_to_slice(low: u8, high: u8) -> Vec<PixelData> {
    let mut pixel_data: Vec<PixelData> = Vec::new();

    for i in 0..8 {
        pixel_data.push(PixelData {
            color: match (low.get_bit(i as u8), high.get_bit(i as u8)) {
                (false, false) => Color::Light,
                (false, true) => Color::MediumlyLight,
                (true, false) => Color::MediumlyDark,
                (true, true) => Color::Dark,
            },
        });
    }

    pixel_data
}

/// Gpu Data that is specific to the pixel transfer process, to not clutter the main gpu
/// struct
pub(super) struct PixelTransferData {
    pub(super) state: PixelTransferState,

    // Get tile step
    pub(super) lcdc_3: bool,
    pub(super) tile_id: u8,

    // Get tile data
    pub(super) tile_data_low: u8,
    pub(super) tile_data_high: u8,

    /// Pixel transfer states usually happen in two steps, to track wheter to activate
    /// step 1 or step 2 we use this field
    pub(super) is_first_call: bool,

    /// The number of times that we added pixels to the fifo
    pub(super) number_of_slices_pushed: u8,
}
