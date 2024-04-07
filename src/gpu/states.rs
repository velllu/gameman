use crate::{
    common::Bit,
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::{LCDC, LY, SCX, SCY, WX, WY},
    },
    GameBoy,
};

use super::{
    sprite_parser::{SpriteData, SpriteHeight},
    tile_parser::Line,
    Color,
};

#[derive(PartialEq, Debug)]
pub enum GPUState {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

pub struct Gpu {
    pub state: GPUState,
    pub screen: [[Color; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
    pub(crate) sprites: Vec<SpriteData>,

    /// When the screen is off, the GPU is in a state of hybernation, as soon as this is
    /// flipped back, the GPU will reset itself. During this state, the screen will be
    /// blank
    pub hybernated: bool,

    /// We need to keep track of how many sprites we have rendered because there is a
    /// maximum of 10 sprites, after that, no more sprites can be rendered on a line
    rendered_sprites_on_line: u8,

    // These represent the current position of the "cursor"
    pub(crate) x: u8,
    pub(crate) y: u8,

    /// A dot is 1/4 of a CPU cycle
    pub dots: u16,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            state: GPUState::OAMSearch,
            screen: [[Color::Light; DISPLAY_SIZE_X]; DISPLAY_SIZE_Y],
            sprites: Vec::new(),
            hybernated: false,
            rendered_sprites_on_line: 0,
            x: 0,
            y: 0,
            dots: 0,
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

        if self.gpu.dots == 0 {
            self.bus[LY] = 0;
        }

        if self.gpu.dots == 80 {
            self.gpu.state = GPUState::PixelTransfer;
            self.gpu.dots = 0;
        } else {
            self.gpu.dots += 1;
        }
    }

    fn pixel_transfer(&mut self) {
        let sprite_height = match self.bus[LCDC].get_bit(2) {
            false => SpriteHeight::Short,
            true => SpriteHeight::Tall,
        };

        // There's a delay of 12 dots at the beginning of this mode due to the tile
        // fetching below
        if self.gpu.dots == 0 {
            self.gpu.dots += 12;
        }

        // TODO: Scrolling is actually handled a bit differently, read this
        // "https://gbdev.io/pandocs/Scrolling.html#mid-frame-behavior"

        // And we get the background fifo and the sprite fifo
        let background_fifo = match self.bus[LCDC].get_bit(0) {
            false => Line::new_blank(), // When LCDC.0 is 0, the background tile is white
            true => self.get_line_from_coordinates(
                self.gpu.x.wrapping_add(self.bus[SCX]),
                self.gpu.y.wrapping_add(self.bus[SCY]),
                false,
            ),
        };

        let mut sprite = self.get_sprite_fifo(self.gpu.x, self.gpu.y, &sprite_height);

        if let Some((sprite_fifo, sprite_data)) = &mut sprite {
            if self.gpu.rendered_sprites_on_line < 10 {
                self.apply_palette_to_sprite(sprite_fifo, &sprite_data.palette);
                sprite_fifo.mix_with_background_tile(&background_fifo, &sprite_data.priority);

                self.draw_line(sprite_fifo, self.gpu.x as usize, self.gpu.y as usize);
                self.gpu.rendered_sprites_on_line += 1;
            } else {
                self.draw_line(&background_fifo, self.gpu.x as usize, self.gpu.y as usize);
            }
        } else {
            self.draw_line(&background_fifo, self.gpu.x as usize, self.gpu.y as usize);
        }

        // Window rendering
        // `- 7` because WX is always 7 more then the actual value
        let window_x = self.gpu.x as i32 - (self.bus[WX] as i32 - 7);
        let window_y = self.gpu.y as i32 - self.bus[WY] as i32;

        // Window doesn't scroll so we don't wrap around like with the background
        if window_x >= 0 && window_y >= 0 && self.can_render_window() {
            let window_fifo = self.get_line_from_coordinates(window_x as u8, window_y as u8, true);
            self.draw_line(&window_fifo, self.gpu.x as usize, self.gpu.y as usize);
        }

        self.gpu.x += 8;

        if self.gpu.x == (DISPLAY_SIZE_X as u8) {
            self.gpu.y += 1;
        }

        // TODO: Implement the OBJ penalty algorithm, relevant link:
        // https://gbdev.io/pandocs/Rendering.html
        if self.gpu.dots >= 160 {
            self.gpu.state = GPUState::HBlank;
            self.gpu.dots = 0;
        } else {
            // Because one dot = one pixel
            self.gpu.dots += 8;
        }
    }

    /// HBlank happens at the end of every scanline, this can either set the state to
    /// VBlank if all the lines have been rendered, or OAM Search if there's another
    /// line we need to render
    fn hblank(&mut self) {
        if self.gpu.dots == 0 {
            self.gpu.x = 0;
            self.gpu.rendered_sprites_on_line = 0;
            self.bus[LY] = self.bus[LY].wrapping_add(1);
        }

        // This should be 376 - the duration of the pixel transfer, but pixel transfer is
        // hard-coded to 160 as of now
        if self.gpu.dots == 376 - 160 {
            self.gpu.state = if self.bus[LY] == 144 {
                GPUState::VBlank
            } else {
                GPUState::PixelTransfer
            };

            self.gpu.dots = 0;
        } else {
            self.gpu.dots += 1;
        }
    }

    /// VBlank is when all the lines have been rendered, there's actually some
    /// "invisible" lines that are not seen on the screen, while we loop over this lines
    /// we are in VBlank territory
    fn vblank(&mut self) {
        const VBLANK_LINE_DOTS: u16 = 456;

        if self.gpu.dots == 0 {
            self.gpu.y = 0;
        }

        // After every line
        if self.gpu.dots % VBLANK_LINE_DOTS == 0 {
            self.bus[LY] = self.bus[LY].wrapping_add(1);
        }

        // There are 10 lines of VBlank
        if self.gpu.dots == VBLANK_LINE_DOTS * 10 {
            self.gpu.state = GPUState::OAMSearch;
            self.gpu.dots = 0;
        } else {
            self.gpu.dots += 1;
        }
    }

    pub(crate) fn tick(&mut self) {
        // The seventh bith of LCDC controls wheter or not the display is on. If it is off
        // we reset the gpu state
        if !self.bus[LCDC].get_bit(7) {
            self.gpu.hybernated = true;
        }

        // Resetting the GPU at every tick is *very* expensive, so we reset LY,
        // because programs might need LY, and also the GPU state, otherwise the GPU will
        // be stuck at hblank, making the program crash because of addition overflow.
        if self.gpu.hybernated {
            self.bus[LY] = 0;
            self.gpu.state = GPUState::OAMSearch;
        }

        // We reset the GPU only when
        if self.gpu.hybernated && self.bus[LCDC].get_bit(7) {
            self.gpu.hybernated = false;
            self.gpu = Gpu::new();
        }

        match self.gpu.state {
            GPUState::OAMSearch => self.oam_search(),
            GPUState::PixelTransfer => self.pixel_transfer(),
            GPUState::HBlank => self.hblank(),
            GPUState::VBlank => self.vblank(),
        }
    }
}
