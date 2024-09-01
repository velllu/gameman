#![warn(clippy::match_same_arms)]

use crate::{bus::Bus, flags::Flags, registers::Registers};

use super::{Bytes, Cpu, Cycles};

impl Cpu {
    pub fn interpret_opcode(
        &mut self,
        opcode: u8,
        flags: &mut Flags,
        regs: &mut Registers,
        bus: &mut Bus,
    ) -> (Bytes, Cycles) {
        if self.is_cb {
            self.is_cb = false;
            return self.interpret_cb_opcode(opcode, flags, regs, bus);
        }

        match opcode {
            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}
