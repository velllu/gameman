#![allow(clippy::precedence)] // i think it's much less clear when formatting like clippy
                              // wants to

pub(crate) mod background;

use super::{Color, GpuState, PixelData};
use crate::{bus::Bus, common::Bit, consts::display::DISPLAY_SIZE_X, GameBoy};

/// The GameBoy's GPU works by having three "layers", the background layer, the window
/// layer and the sprite layer, this trait defines the parts that differ for every layer,
/// the common parts are defined in this file.
pub(crate) trait Layer: Send {
    fn is_layer_enabled(&self, bus: &Bus) -> bool;
    fn get_tile_step_1(&mut self, bus: &Bus);
    fn get_tile_step_2(&mut self, bus: &Bus);
    fn get_tile_data(&mut self, is_high_part: bool, bus: &Bus);
    fn push_pixels(&mut self, bus: &Bus) -> Vec<PixelData>;
    fn at_new_scanline(&mut self, fifo: &mut Vec<PixelData>);
}

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
        self.gpu.pixel_transfer_state = match self.gpu.pixel_transfer_state {
            PixelTransferState::GetTile => PixelTransferState::GetLowTileData,
            PixelTransferState::GetLowTileData => PixelTransferState::GetHighTileData,
            PixelTransferState::GetHighTileData => PixelTransferState::Sleep,
            PixelTransferState::Sleep => PixelTransferState::PushPixels,
            PixelTransferState::PushPixels => PixelTransferState::GetTile,
        };

        self.gpu.is_pixel_transfer_first_call = true;
    }

    /// Inverts the `is_first_call` field and returns the previous value, meant to be used
    /// in the pixel transfer states to check wheter to activate step 1 or step 2
    fn is_first_call(&mut self) -> bool {
        self.gpu.is_pixel_transfer_first_call = !self.gpu.is_pixel_transfer_first_call;
        !self.gpu.is_pixel_transfer_first_call
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
        match self.gpu.pixel_transfer_state {
            PixelTransferState::GetTile => self.get_tile(),
            PixelTransferState::GetLowTileData => self.get_tile_data(false),
            PixelTransferState::GetHighTileData => self.get_tile_data(true),
            PixelTransferState::Sleep => self.sleep(),
            PixelTransferState::PushPixels => self.push_pixels(),
        }

        // Pop one pixel and display it
        if let Some(pixel_data) = self.gpu.fifo.pop() {
            self.gpu.screen[self.gpu.y as usize][self.gpu.x as usize] = pixel_data.color;
            self.gpu.x += 1;
        }

        // TODO: Implement the OBJ penalty algorithm, relevant link:
        // https://gbdev.io/pandocs/Rendering.html

        if self.gpu.x == DISPLAY_SIZE_X as u8 {
            self.gpu.state = GpuState::HBlank;
            self.gpu.ticks = 0;
            self.gpu.x = 0;
            self.gpu.y += 1;

            self.gpu
                .layers
                .iter_mut()
                .for_each(|layer| layer.at_new_scanline(&mut self.gpu.fifo));

            return;
        }

        self.gpu.ticks += 1;
    }

    fn get_tile(&mut self) {
        match self.is_first_call() {
            true => self
                .gpu
                .layers
                .iter_mut()
                .for_each(|layer| layer.get_tile_step_1(&self.bus)),

            false => {
                self.gpu
                    .layers
                    .iter_mut()
                    .for_each(|layer| layer.get_tile_step_2(&self.bus));

                self.cycle_state();
            }
        }
    }

    fn get_tile_data(&mut self, is_high_part: bool) {
        // This instruction takes two ticks, cpus are so fast that we can just do it all
        // in the second tick
        if self.is_first_call() {
            return;
        }

        self.gpu
            .layers
            .iter_mut()
            .for_each(|layer| layer.get_tile_data(is_high_part, &self.bus));

        self.cycle_state();
    }

    fn push_pixels(&mut self) {
        if !self.gpu.fifo.is_empty() {
            return;
        }

        let mut slice = self.gpu.layers[0].push_pixels(&self.bus);
        self.gpu.fifo.append(&mut slice);

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
