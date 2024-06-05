#![forbid(unsafe_code)]

use bus::{Bus, BusError};
use common::merge_two_u8s_into_u16;
use consts::bus::ROM_SIZE;
use flags::Flags;
use gpu::Gpu;
use registers::Registers;

mod bus;
pub mod common;
pub mod consts;
mod cpu;
mod flags;
pub mod gpu;
mod interrupts;
mod registers;

#[cfg(test)]
mod tests;

pub struct GameBoy {
    pub bus: Bus,
    pub registers: Registers,
    pub flags: Flags,
    pub gpu: Gpu,

    // TODO: Remove this, to make the code better. Check `interrupts.rs` for more
    // information on why this is needed
    previous_lcd: Option<bool>,
}

impl GameBoy {
    pub fn new(rom_path: &str) -> Result<Self, BusError> {
        Ok(Self {
            bus: Bus::new(rom_path)?,
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            previous_lcd: None,
        })
    }

    pub fn new_from_rom_array(rom: [u8; ROM_SIZE]) -> Self {
        Self {
            bus: Bus::new_from_rom_array(rom),
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            previous_lcd: None,
        }
    }

    /// Parse and run the next opcode
    pub fn step(&mut self) {
        let opcode = self.next(0);

        // CPU - Opcodes
        let (bytes, cycles) = self.interpret_opcode(opcode);
        self.registers.pc = self.registers.pc.wrapping_add(bytes as u16);

        // CPU - Interrupts
        self.execute_interrupts();

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

// TODO: Move this to a more appropriate space
impl GameBoy {
    /// Returns the byte X times after the `PC` register
    pub(crate) fn next(&self, offset: u16) -> u8 {
        self.bus
            .read_from_rom(self.registers.pc.wrapping_add(offset))
    }

    /// Returns the next two bytes from the `PC` register in little endian format
    pub(crate) fn next_two(&self) -> u16 {
        merge_two_u8s_into_u16(self.next(2), self.next(1))
    }
}
