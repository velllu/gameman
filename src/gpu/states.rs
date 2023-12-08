use crate::{
    common::Bit,
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::{LY, SCX, SCY},
    },
    GameBoy,
};

use super::{oam_parser::SpriteData, tile_parser::Line, Color};

#[derive(PartialEq)]
pub enum GPUState {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

pub struct Gpu {
    pub state: GPUState,
    pub screen: [[Color; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],

    sprites: Vec<SpriteData>,

    // These represent the current position of the "cursor"
    x: u8,
    y: u8,

    /// This is the offset to add to the tile map address
    i: u16,

    /// A tick is 1/4 of a CPU cycle
    ticks: u32,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            state: GPUState::OAMSearch,
            screen: [[Color::Light; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
            sprites: Vec::new(),
            x: 0,
            y: 0,
            i: 0,
            ticks: 0,
        }
    }
}

// TODO: During pixel transfer mode, the vram should not be accessible
// TODO: During pixel transfer mode and oam search mode, the oam should not be accessible
// TODO: Document each step of the GPU

impl GameBoy {
    /// The OAM Search phase is where we search for the visible sprites in the current
    /// scanline in the OAM region of the RAM
    fn oam_search(&mut self) {
        // TODO: The timing on this is still to implement, this just checks *all* the
        // sprites all at once every OAM Search tick, which is both grossly inaccurate
        // and inefficient
        self.gpu.sprites.clear();
        for i in (0xFE00..0xFE9C).step_by(4) {
            self.gpu.sprites.push(self.get_sprite_data(i));
        }

        if self.gpu.ticks == 40 {
            self.gpu.state = GPUState::PixelTransfer;
        }
    }

    fn pixel_transfer(&mut self) {
        let mut tile_map_address: u16 = match self.bus[0xFF40].get_bit(3) {
            false => 0x9800,
            true => 0x9C00,
        };

        // Y Scrolling
        // The gameboy tilemap is 32x32 tiles, both `SCX` and `SCY` use pixels, not tiles
        // so we have to divide them by 8, skipping 32 tiles just means to set the
        // "cursor" on the line below
        for _ in 0..(self.bus[SCY] / 8) {
            tile_map_address += 32;
        }

        // X Scrolling
        // We add the number of tiles to skip to the adress
        tile_map_address += self.bus[SCX] as u16 / 8;

        // Adding i
        tile_map_address += self.gpu.i;

        let background_fifo = self.get_line(self.bus[tile_map_address], self.gpu.y as u16 % 8);

        // Now that we have the background line to render, we have to get the sprite one
        let mut sprite_fifo: Option<Line> = None;
        for sprite in &self.gpu.sprites {
            let sprite_y = sprite.y - 16;
            let sprite_x = sprite.x - 8;

            // We check if there is any sprite that is on the same x axis as our "cursor"
            let x_condition = sprite_x == self.gpu.x;

            // and we check if we also are on the same y axis, however, a sprite is 8
            // pixel long, so we check if we are anywhere between row 0 to 7
            let y_condition = ((sprite_y)..(sprite_y + 7)).contains(&self.gpu.y);

            if x_condition && y_condition {
                sprite_fifo = Some(self.get_line(sprite.tile_number, self.gpu.y as u16 % 8));
            }
        }

        // TODO: Implement fifo mixing
        if let Some(sprite_fifo) = sprite_fifo {
            self.draw_line(&sprite_fifo, self.gpu.x as usize, self.gpu.y as usize);
        } else {
            self.draw_line(&background_fifo, self.gpu.x as usize, self.gpu.y as usize);
        }

        self.gpu.i += 1;
        self.gpu.x += 8;

        if self.gpu.x == (DISPLAY_SIZE_X as u8) {
            // If we finished rendering all the 20 tiles, and we want to go to the next
            // set of tile, we skip 12, because the tile map is 32x32, and the viewport
            // is 20x18 (32 - 20), and if we haven't rendered the 20 tiles, we go back
            // to the first one
            if self.gpu.y % 8 == 7 {
                self.gpu.i += 12;
            } else {
                self.gpu.i -= 20;
            }

            self.gpu.y += 1;
            self.gpu.state = GPUState::HBlank;
        }
    }

    /// HBlank happens at the end of every scanline, this can either set the state to
    /// VBlank if all the lines have been rendered, or OAM Search if there's another
    /// line we need to render
    fn hblank(&mut self) {
        if self.gpu.ticks != 456 {
            return;
        }

        self.gpu.ticks = 0;
        self.gpu.x = 0;
        self.bus[LY] = self.bus[LY].wrapping_add(1);

        self.gpu.state = if self.bus[LY] == DISPLAY_SIZE_Y as u8 {
            GPUState::VBlank
        } else {
            GPUState::OAMSearch
        }
    }

    /// VBlank is when all the lines have been rendered, there's actually some
    /// "invisible" lines that are not seen on the screen, while we loop over this lines
    /// we are in VBlank territory
    fn vblank(&mut self) {
        if self.gpu.ticks != 456 {
            return;
        }

        self.gpu.ticks = 0;
        self.bus[LY] = self.bus[LY].wrapping_add(1);

        if self.bus[LY] == 153 {
            self.bus[LY] = 0;
            self.gpu.y = 0;
            self.gpu.i = 0;
            self.gpu.state = GPUState::OAMSearch;
        }
    }

    pub(crate) fn tick(&mut self) {
        match self.gpu.state {
            GPUState::OAMSearch => self.oam_search(),
            GPUState::PixelTransfer => self.pixel_transfer(),
            GPUState::HBlank => self.hblank(),
            GPUState::VBlank => self.vblank(),
        }

        self.gpu.ticks += 1;
    }
}
