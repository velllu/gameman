use crate::{bus::Bus, flags::Flags, registers::Registers};

use super::{Bytes, Cpu, Cycles};

impl Cpu {
    pub fn interpret_cb_opcode(
        &mut self,
        opcode: u8,
        flags: &mut Flags,
        regs: &mut Registers,
        bus: &mut Bus,
    ) -> (Bytes, Cycles) {
        match opcode {
            _ => panic!("Unimplemented CB opcode: {:x}", opcode),
        }
    }
}
