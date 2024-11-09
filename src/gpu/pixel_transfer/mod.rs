#![allow(clippy::precedence)] // i think it's much less clear when formatting like clippy
                              // wants to

pub(crate) mod background;
pub(crate) mod sprite;
pub(crate) mod window;

use super::{Color, Gpu, GpuState, PixelData, Priority};
use crate::{bus::Bus, common::Bit, consts::display::DISPLAY_SIZE_X};

/// The GameBoy's GPU works by having three "layers", the background layer, the window
/// layer and the sprite layer, this trait defines the parts that differ for every layer,
/// the common parts are defined in this file.
pub(crate) type Layers = [Box<dyn Layer>; 3];

/// While being different, the layers all have the same interface
#[allow(unused_variables)]
pub(crate) trait Layer: Send {
    fn is_layer_enabled(&self, bus: &Bus) -> bool;
    fn mix_with_layer_below(&self) -> Priority;
    fn get_tile_step_1(&mut self, gpu: &Gpu, bus: &Bus);
    fn get_tile_step_2(&mut self, gpu: &Gpu, bus: &Bus);
    fn get_tile_data(&mut self, is_high_part: bool, gpu: &Gpu, bus: &Bus);
    fn push_pixels(&mut self, gpu: &Gpu, bus: &Bus) -> Vec<PixelData>;

    // Events
    fn at_hblank(&mut self, bus: &Bus, gpu: &Gpu) {}
    fn at_vblank(&mut self, bus: &Bus, gpu: &Gpu) {}
}

#[derive(PartialEq, Debug)]
pub(super) enum PixelTransferState {
    GetTile,
    GetLowTileData,
    GetHighTileData,
    Sleep,
    PushPixels,
}

impl Gpu {
    fn cycle_state(&mut self) {
        self.pixel_transfer_state = match self.pixel_transfer_state {
            PixelTransferState::GetTile => PixelTransferState::GetLowTileData,
            PixelTransferState::GetLowTileData => PixelTransferState::GetHighTileData,
            PixelTransferState::GetHighTileData => PixelTransferState::Sleep,
            PixelTransferState::Sleep => PixelTransferState::PushPixels,
            PixelTransferState::PushPixels => PixelTransferState::GetTile,
        };

        self.is_pixel_transfer_first_call = true;
    }

    /// Inverts the `is_first_call` field and returns the previous value, meant to be used
    /// in the pixel transfer states to check wheter to activate step 1 or step 2
    fn is_first_call(&mut self) -> bool {
        self.is_pixel_transfer_first_call = !self.is_pixel_transfer_first_call;
        !self.is_pixel_transfer_first_call
    }

    /// In this state the pixels are getting fetched and put into the screen, it has 5
    /// step, and except the fifth step, they last 2 dots
    /// 1. Get tile
    /// 2. Get low tile data
    /// 3. Get high tile data
    /// 4. Sleep
    /// 5. Pushing pixels
    ///
    /// This is a super helpful resource:
    /// https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md
    pub(super) fn pixel_transfer(&mut self, layers: &mut Layers, bus: &mut Bus) {
        match self.pixel_transfer_state {
            PixelTransferState::GetTile => self.get_tile(layers, bus),
            PixelTransferState::GetLowTileData => self.get_tile_data(false, layers, bus),
            PixelTransferState::GetHighTileData => self.get_tile_data(true, layers, bus),
            PixelTransferState::Sleep => self.sleep(),
            PixelTransferState::PushPixels => self.push_pixels(layers, bus),
        }

        // Pop one pixel and display it
        if let Some(pixel_data) = self.fifo.pop() {
            self.screen[self.y as usize][self.x as usize] = pixel_data.color;
            self.x += 1;
        }

        // TODO: Implement the OBJ penalty algorithm, relevant link:
        // https://gbdev.io/pandocs/Rendering.html

        if self.x == DISPLAY_SIZE_X as u8 {
            self.state = GpuState::HBlank;
            self.ticks = 0;
            self.number_of_slices_pushed = 0;

            return;
        }

        self.ticks += 1;
    }

    fn get_tile(&mut self, layers: &mut Layers, bus: &mut Bus) {
        match self.is_first_call() {
            true => layers
                .iter_mut()
                .for_each(|layer| layer.get_tile_step_1(self, bus)),

            false => {
                // This is where the X pointer would be if we always pushed 8 pixels at a
                // time (which happens when SCX is not a multiple of 8)
                self.virtual_x = self.number_of_slices_pushed * 8;

                layers
                    .iter_mut()
                    .for_each(|layer| layer.get_tile_step_2(self, bus));

                self.cycle_state();
            }
        }
    }

    fn get_tile_data(&mut self, is_high_part: bool, layers: &mut Layers, bus: &mut Bus) {
        // This instruction takes two ticks, cpus are so fast that we can just do it all
        // in the second tick
        if self.is_first_call() {
            return;
        }

        for layer in layers.iter_mut() {
            layer.get_tile_data(is_high_part, self, bus);
        }

        self.cycle_state();
    }

    fn push_pixels(&mut self, layers: &mut Layers, bus: &mut Bus) {
        if !self.fifo.is_empty() {
            return;
        }

        let mut slice: Vec<PixelData> = EMPTY_SLICE.into();

        for layer in layers.iter_mut() {
            let new_slice = layer.push_pixels(self, bus);

            slice = match layer.mix_with_layer_below() {
                Priority::AlwaysAbove => new_slice,
                Priority::TransparentLight => mix_with_z_index(&slice, &new_slice),
                Priority::AboveLight => mix_above_light(&slice, &new_slice),
            };
        }

        if self.dump_slice {
            slice.clear();
            self.dump_slice = false;
        } else {
            self.number_of_slices_pushed += 1;
        }

        self.fifo.append(&mut slice);
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
            color: bools_to_color(high.get_bit(i as u8), low.get_bit(i as u8)),
            z_index: 0,
        });
    }

    pixel_data
}

pub(crate) fn bools_to_color(bool1: bool, bool2: bool) -> Color {
    match (bool1, bool2) {
        (false, false) => Color::Light,
        (false, true) => Color::MediumlyLight,
        (true, false) => Color::MediumlyDark,
        (true, true) => Color::Dark,
    }
}

/// Implementation of this gate:
/// https://github.com/furrtek/DMG-CPU-Inside/blob/f0eda633eac24b51a8616ff782225d06fccbd81f/Schematics/25_VRAM_INTERFACE.png
pub(super) fn vuza_gate(x: u8, y: u8) -> u16 {
    !((x & 0x10) != 0 || (y & 0x80) != 0) as u16
}

fn mix_with_z_index(below_slice: &[PixelData], above_slice: &[PixelData]) -> Vec<PixelData> {
    let mut new_slice: Vec<PixelData> = Vec::new();

    for (below_pixel, above_pixel) in below_slice.iter().zip(above_slice) {
        // We select the pixel based on the higher z index
        if above_pixel.z_index > below_pixel.z_index {
            new_slice.push(*above_pixel);
        } else {
            new_slice.push(*below_pixel);
        }
    }

    new_slice
}

fn mix_above_light(below_slice: &[PixelData], above_slice: &[PixelData]) -> Vec<PixelData> {
    let mut new_slice: Vec<PixelData> = Vec::new();

    for (below_pixel, above_pixel) in below_slice.iter().zip(above_slice) {
        // The above sprite will only show when the below sprite is light, no need to
        // check for priority
        if below_pixel.color == Color::Light {
            new_slice.push(*above_pixel);
        } else {
            new_slice.push(*below_pixel);
        }
    }

    new_slice
}

/// An eight pixel blank line
pub(super) const EMPTY_SLICE: [PixelData; 8] = [PixelData {
    color: Color::Light,
    z_index: 0,
}; 8];
