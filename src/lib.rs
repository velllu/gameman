#![forbid(unsafe_code)]

use bus::{Bus, BusError};
use consts::joypad::JOYP;
use cpu::Cpu;
use flags::Flags;
use gpu::{
    pixel_transfer::{
        background::BackgroundLayer, sprite::SpriteLayer, window::WindowLayer, Layer,
    },
    Gpu,
};
use joypad::Joypad;
use registers::Registers;

mod bus;
pub mod common;
pub mod consts;
mod cpu;
pub mod flags;
pub mod gpu;
mod joypad;
pub mod registers;

pub struct GameBoy {
    pub bus: Bus,
    pub cpu: Cpu,
    pub flags: Flags,
    pub gpu: Gpu,
    pub joypad: Joypad,
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
            flags: Flags::new(),
            gpu: Gpu::new(),
            joypad: Joypad::new(),
            registers: Registers::new(),
            layers: [
                Box::new(BackgroundLayer::new()),
                Box::new(WindowLayer::new()),
                Box::new(SpriteLayer::new()),
            ],
        })
    }

    pub fn new_from_rom_array(rom: Vec<u8>) -> Self {
        Self {
            bus: Bus::new_from_rom_array(rom),
            cpu: Cpu::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            joypad: Joypad::new(),
            registers: Registers::new(),
            layers: [
                Box::new(BackgroundLayer::new()),
                Box::new(WindowLayer::new()),
                Box::new(SpriteLayer::new()),
            ],
        }
    }

    /// Parse and run the next opcode
    pub fn step(&mut self) {
        let opcode = self.bus.next(0, &self.registers);

        // CPU - Opcodes
        let (bytes, cycles) =
            self.cpu
                .interpret_opcode(opcode, &mut self.flags, &mut self.registers, &mut self.bus);

        self.registers.pc = self.registers.pc.wrapping_add(bytes as u16);

        // CPU - Interrupts
        self.cpu
            .execute_interrupts(&self.gpu, &mut self.registers, &mut self.bus);

        // CPU - OAM DMA Transfer
        if self.bus.needs_to_dispatch_oam_dma {
            self.bus.dispatch_oam_transfer();
            self.bus.needs_to_dispatch_oam_dma = false;
        }

        // CPU - DIV Register
        self.cpu.update_div_register(&mut self.bus, cycles);

        // GPU
        for _ in 0..(cycles * 4) {
            // A GPU tick is 1/4 of a cycle, so it needs to be called 4 times for every
            // cycle. TODO: This is not actually all that accurate to the actual GameBoy
            // but to change this I will have to redesign the CPU to count cycles
            // procedurally and call `Gpu.tick()` for every cycle from there
            self.tick();
        }

        // JOYPAD
        self.bus.write(JOYP, self.joypad.to_byte(&self.bus));
    }
}
