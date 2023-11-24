use crate::{
    consts::{
        display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
        gpu::LY,
    },
    GameBoy,
};

#[derive(PartialEq)]
pub enum GPUState {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

pub struct Gpu {
    pub state: GPUState,

    /// This keeps track of the pixel outputted in the current scanline
    already_outputted_pixel: u32,

    /// A tick is 1/4 of a CPU cycle
    ticks: u32,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            state: GPUState::OAMSearch,
            already_outputted_pixel: 0,
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
        // TODO: Actually implement the sprite searching

        if self.gpu.ticks == 40 {
            self.gpu.state = GPUState::PixelTransfer;
        }
    }

    fn pixel_transfer(&mut self) {
        // TODO: Document and implement this

        self.gpu.already_outputted_pixel += 1;
        if self.gpu.already_outputted_pixel == DISPLAY_SIZE_X as u32 {
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
        self.gpu.already_outputted_pixel = 0;
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
