#![allow(unused)]

use std::marker::Copy;

use crate::GameBoy;

#[derive(PartialEq)]
enum GPUState {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

#[derive(Clone, Copy)]
enum Source {
    Background,
    Window,
}

#[derive(Clone, Copy)]
enum Color {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy)]
struct PixelData {
    source: Source,
    color: Color,
}

pub struct GPU {
    state: GPUState,

    /// The buffer of the pixels to push
    fifo: [Option<PixelData>; 8],
}

impl GPU {
    pub(crate) fn new() -> Self {
        Self {
            state: GPUState::OAMSearch,
            fifo: [None; 8],
        }
    }
}

// TODO: During pixel transfer mode, the vram should not be accessible
// TODO: During pixel transfer mode and oam search mode, the oam should not be accessible

impl GameBoy {
    /// We need to search in the Object Attribute Memory the sprites that are actually
    /// visible in the current line
    fn oam_search(&mut self) {
        // TODO
        self.gpu.state = GPUState::PixelTransfer;
    }

    fn pixel_transfer(&mut self) {
        // TODO
        self.gpu.state = GPUState::HBlank;
    }

    fn hblank(&mut self) {
        // TODO
        self.gpu.state = GPUState::VBlank;
    }

    fn vblank(&mut self) {
        // TODO
        self.gpu.state = GPUState::OAMSearch;
    }

    #[rustfmt::skip]
    pub(crate) fn gpu_step(&mut self) {
        match self.gpu.state {
            GPUState::OAMSearch => self.oam_search(),
            GPUState::PixelTransfer => self.pixel_transfer(),
            GPUState::HBlank => self.hblank(),
            GPUState::VBlank => self.vblank(),
        }
    }
}
