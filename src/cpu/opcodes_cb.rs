use crate::{bus::Bus, common::Bit, flags::Flags, registers::Registers};

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
            // Instruction `BIT n, r` - 01nnnrrr
            // Set z flag to Nth bit of register R
            0x40..=0x7F => {
                let number = (opcode >> 3) & 0b00000111;
                let register = regs.get_register(opcode, bus);

                flags.zero = register.get_bit(number);

                (1, 1)
            }

            // Instruction `RES n, r` - 11nnnrrr
            // Set Nth bit of register R to 0
            0x80..=0xBF => {
                let number = (opcode >> 3) & 0b00000111;
                let mut register = regs.get_register(opcode, bus);

                register.set_bit(number, false);
                regs.set_register(opcode, register, bus);

                (1, 1)
            }

            // Instruction `SET n, r` - 11nnnrrr
            // Set Nth bit of register R to 1
            0xC0..=0xFF => {
                let number = (opcode >> 3) & 0b00000111;
                let mut register = regs.get_register(opcode, bus);

                register.set_bit(number, true);
                regs.set_register(opcode, register, bus);

                (1, 1)
            }

            _ => panic!("Unimplemented CB opcode: {:x}", opcode),
        }
    }
}
