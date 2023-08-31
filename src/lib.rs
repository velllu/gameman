#![forbid(unsafe_code)]
#![allow(unused)]

use bus::Bus;
use common::merge_two_u8s_into_u16;
use errors::EmuError;
use flags::Flags;
use registers::Registers;
use screen::Screen;

mod bus;
mod common;
mod consts;
mod cpu;
mod errors;
mod flags;
mod macros;
mod registers;
mod screen;

pub struct GameBoy {
    pub bus: Bus,
    pub screen: Screen,
    pub registers: Registers,
    pub flags: Flags,

    /// Some gameboy opcodes have a prefix `0xCB`
    pub is_cb: bool,

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
            is_cb: false,
            current_opcode: None,
        })
    }

    /// Parse and run the next opcode
    pub fn step(&mut self) {
        let opcode = self.next(0);
        self.current_opcode = Some(opcode);

        if self.is_cb {
            self.interpret_cb_opcode(opcode);
        } else {
            self.interpret_opcode(opcode);
        }

        if opcode == 0xCB {
            self.is_cb = true;
            return;
        }

        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.is_cb = false;
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
