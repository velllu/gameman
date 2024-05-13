use crate::{
    common::Bit,
    consts::{
        display::DISPLAY_SIZE_X,
        gpu::{LCDC, SCX, SCY},
    },
    GameBoy,
};

use super::{Color, GpuState, PixelData};

/// A line of a tile
type Slice = [PixelData; 8];

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
        println!("{:?}", self.gpu.pixel_transfer_data.state);

        match self.gpu.pixel_transfer_data.state {
            PixelTransferState::GetTile => self.get_tile(),
            PixelTransferState::GetLowTileData => self.get_tile_data(false),
            PixelTransferState::GetHighTileData => self.get_tile_data(true),
            PixelTransferState::Sleep if self.gpu.ticks % 2 != 0 => self.cycle_state(),
            PixelTransferState::Sleep => {}
            PixelTransferState::PushPixels => self.push_pixels(),
        }

        // Pop one pixel and display it
        if let Some(color) = self.gpu.background_fifo.contents.pop() {
            self.gpu.screen[self.gpu.y as usize][self.gpu.x as usize] = color.color;
            self.gpu.x += 1;
        }

        // TODO: Implement the OBJ penalty algorithm, relevant link:
        // https://gbdev.io/pandocs/Rendering.html

        if self.gpu.x == DISPLAY_SIZE_X as u8 {
            self.gpu.y += 1;
            self.gpu.x = 0;
            self.gpu.state = GpuState::HBlank;
            self.gpu.ticks = 0;
        }

        self.gpu.ticks += 1;
    }

    /// Look at the link above to see the formula laid out in a table
    /// TODO: This can be *massively* improved when rust adds yielding
    fn get_tile(&mut self) {
        match self.gpu.ticks % 2 == 0 {
            // On the first dot the GPU gets LCDC.3
            true => {
                self.gpu.pixel_transfer_data.lcdc_3 = self.bus[LCDC].get_bit(3);
            }

            // On the second dot the GPU calculates the tilemap address and fetches it
            false => {
                let base: u16 = 0b10011;
                let lcdc_3: u16 = self.gpu.pixel_transfer_data.lcdc_3 as u16;
                let tile_y: u16 = (self.gpu.y as u16 + self.bus[SCY] as u16) / 8;
                let tile_x: u16 = (self.gpu.x as u16 + self.bus[SCX] as u16) / 8;

                let address = (base << 11) | (lcdc_3 << 10) | (tile_y << 5) | tile_x;
                self.gpu.pixel_transfer_data.tile_id = self.bus[address];

                self.cycle_state();
            }
        }
    }

    fn get_tile_data(&mut self, is_high_part: bool) {
        // This instruction takes two ticks, cpus are so fast that we can just do it all
        // in the second tick
        if self.gpu.ticks % 2 == 0 {
            return;
        }

        let base: u16 = 0b100;
        let bit_12 = !((self.bus[LCDC] & 0x10) != 0
            || (self.gpu.pixel_transfer_data.tile_id & 0x80) != 0) as u16;
        let tile_id = self.gpu.pixel_transfer_data.tile_id as u16;
        let ly_scy = (self.gpu.y as u16 + self.bus[SCY] as u16) % 8;

        let address =
            (base << 13) | (bit_12 << 12) | (tile_id << 4) | (ly_scy << 1) | is_high_part as u16;

        match is_high_part {
            false => self.gpu.pixel_transfer_data.tile_data_low = self.bus[address],
            true => self.gpu.pixel_transfer_data.tile_data_high = self.bus[address],
        }

        self.cycle_state();
    }

    fn push_pixels(&mut self) {
        if !self.gpu.background_fifo.contents.is_empty() {
            return;
        }

        self.gpu.background_fifo.append(&bytes_to_slice(
            self.gpu.pixel_transfer_data.tile_data_low,
            self.gpu.pixel_transfer_data.tile_data_high,
        ));

        self.cycle_state();
    }
}

fn bytes_to_slice(low: u8, high: u8) -> Slice {
    let mut new_slice = [PixelData {
        color: Color::Light,
    }; 8];

    for i in 0..7 {
        new_slice[i] = PixelData {
            color: match (low.get_bit(i as u8), high.get_bit(i as u8)) {
                (false, false) => Color::Light,
                (false, true) => Color::MediumlyLight,
                (true, false) => Color::MediumlyDark,
                (true, true) => Color::Dark,
            },
        }
    }

    new_slice
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
}
