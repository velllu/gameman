use crate::{consts::display::DISPLAY_SIZE_X, GameBoy};

#[derive(PartialEq)]
pub enum GPUState {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

pub struct Gpu {
    pub state: GPUState,
    already_outputted_pixel: u32,
    steps: u32,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        Self {
            state: GPUState::OAMSearch,
            already_outputted_pixel: 0,
            steps: 0,
        }
    }
}

// TODO: During pixel transfer mode, the vram should not be accessible
// TODO: During pixel transfer mode and oam search mode, the oam should not be accessible
// TODO: Document each step of the GPU

impl GameBoy {
    fn oam_search(&mut self) {
        if self.gpu.steps == 40 {
            self.gpu.state = GPUState::PixelTransfer;
        }
    }

    fn pixel_transfer(&mut self) {
        println!("{}", self.gpu.already_outputted_pixel);
        self.gpu.already_outputted_pixel += 1;
        if self.gpu.already_outputted_pixel == DISPLAY_SIZE_X as u32 {
            self.gpu.state = GPUState::HBlank;
        }
    }

    fn hblank(&mut self) {
        if self.gpu.steps == 456 {
            self.gpu.steps = 0;
            self.gpu.already_outputted_pixel = 0;
            self.bus
                .write_byte(0xFF44, self.bus.read(0xFF44).wrapping_add(1));

            if self.bus.read(0xFF44) == 144 {
                self.gpu.state = GPUState::VBlank;
            } else {
                self.gpu.state = GPUState::OAMSearch;
            }
        }
    }

    fn vblank(&mut self) {
        self.gpu.state = GPUState::OAMSearch;
        if self.gpu.steps == 456 {
            self.gpu.steps = 0;
            self.bus
                .write_byte(0xFF44, self.bus.read(0xFF44).wrapping_add(1));

            if self.bus.read(0xFF44) == 153 {
                self.bus.write_byte(0xFF44, 0);
                self.gpu.state = GPUState::OAMSearch;
            }
        }
    }

    pub(crate) fn gpu_step(&mut self) {
        match self.gpu.state {
            GPUState::OAMSearch => self.oam_search(),
            GPUState::PixelTransfer => self.pixel_transfer(),
            GPUState::HBlank => self.hblank(),
            GPUState::VBlank => self.vblank(),
        }

        self.gpu.steps += 1;
    }
}
