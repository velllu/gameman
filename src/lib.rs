#![forbid(unsafe_code)]

use bus::{Bus, BusError};
use common::merge_two_u8s_into_u16;
use consts::bus::ROM_SIZE;
use cpu::Cpu;
use flags::Flags;
use gpu::{
    pixel_transfer::{
        background::BackgroundLayer, sprite::SpriteLayer, window::WindowLayer, Layer,
    },
    Gpu,
};
use registers::Registers;

mod bus;
pub mod common;
pub mod consts;
mod cpu;
mod flags;
pub mod gpu;
mod registers;

pub struct GameBoy {
    pub bus: Bus,
    pub cpu: Cpu,
    pub flags: Flags,
    pub gpu: Gpu,
    pub registers: Registers,

    /// The GameBoy's screen has three layers, this array houses those layers, they are
    /// decoupled from the other parts of the emulator
    layers: [Box<dyn Layer>; 3],
}

impl GameBoy {
    pub fn new(rom_path: &str) -> Result<Self, BusError> {
        Ok(Self {
            bus: Bus::new(rom_path)?,
            cpu: Cpu::new(),
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            layers: [
                Box::new(BackgroundLayer::new()),
                Box::new(WindowLayer::new()),
                Box::new(SpriteLayer::new()),
            ],
        })
    }

    pub fn new_from_rom_array(rom: [u8; ROM_SIZE]) -> Self {
        Self {
            bus: Bus::new_from_rom_array(rom),
            cpu: Cpu::new(),
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            layers: [
                Box::new(BackgroundLayer::new()),
                Box::new(WindowLayer::new()),
                Box::new(SpriteLayer::new()),
            ],
        }
    }

    /// Parse and run the next opcode
    pub fn step(&mut self) {
        let opcode = self.bus.next_one(&self.registers);

        // CPU - Opcodes
        let (bytes, cycles) =
            self.cpu
                .interpret_opcode(opcode, &mut self.flags, &mut self.registers, &mut self.bus);

        self.registers.pc = self.registers.pc.wrapping_add(bytes as u16);

        // CPU - Interrupts
        self.cpu.execute_interrupts(&mut self.bus);

        // GPU
        for _ in 0..(cycles * 4) {
            // A GPU tick is 1/4 of a cycle, so it needs to be called 4 times for every
            // cycle. TODO: This is not actually all that accurate to the actual GameBoy
            // but to change this I will have to redesign the CPU to count cycles
            // procedurally and call `Gpu.tick()` for every cycle from there
            self.tick();
        }
    }
}
