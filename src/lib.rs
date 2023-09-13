#![forbid(unsafe_code)]

use bus::Bus;
use common::merge_two_u8s_into_u16;
use consts::bus::ROM_SIZE;
use errors::EmuError;
use flags::Flags;
use gpu::Gpu;
use registers::Registers;
use screen::Screen;

mod bus;
mod common;
mod consts;
mod cpu;
mod errors;
mod flags;
mod gpu;
mod interrupts;
mod registers;
mod screen;

#[cfg(test)]
mod tests;

pub struct GameBoy {
    pub bus: Bus,
    pub screen: Screen,
    pub registers: Registers,
    pub flags: Flags,
    pub gpu: Gpu,

    /// This is `None` when the `GameBoy.step()` function is never ran
    pub current_opcode: Option<u8>,
}

impl GameBoy {
    pub fn new(rom_path: &str) -> Result<Self, EmuError> {
        Ok(Self {
            bus: Bus::new(rom_path)?,
            screen: Screen::new(),
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            current_opcode: None,
        })
    }

    pub fn new_from_rom_array(rom: [u8; ROM_SIZE]) -> Self {
        Self {
            bus: Bus::new_from_rom_array(rom),
            screen: Screen::new(),
            registers: Registers::new(),
            flags: Flags::new(),
            gpu: Gpu::new(),
            current_opcode: None,
        }
    }

    /// Parse and run the next opcode
    pub fn step(&mut self) {
        let opcode = self.next(0);
        self.current_opcode = Some(opcode);

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
            // procedurally and call `gpu_step()` for every cycle from there
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
