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
            // Instruction `RL r` - 00010rrr
            // Rotate the contents of register r to the left and set bit 0 to carry flag
            0x10..=0x17 => {
                let register_r = regs.get_register(opcode, bus);
                let mut new_value = register_r.rotate_left(1);
                new_value.set_bit(0, flags.carry);

                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

            // Instruction `RR r` - 00011rrr
            // Rotate the contents of register r to the right and set bit 7 to carry flag
            0x18..=0x1F => {
                let register_r = regs.get_register(opcode, bus);
                let mut new_value = register_r.rotate_right(1);
                new_value.set_bit(7, flags.carry);

                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

            // Instruction `SLA r` - 00100rrr
            // Shift the contents of register r to the left and store bit 7 in carry flag
            0x20..=0x27 => {
                let register_r = regs.get_register(opcode, bus);
                let new_value = register_r << 1;

                flags.carry = register_r.get_bit(7);
                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

            // Instruction `SRA r` - 00101rrr
            // Shift the contents of register r to the right and store bit 0 in carry flag
            // and mantain bit 7
            0x28..=0x2F => {
                let register_r = regs.get_register(opcode, bus);
                let bit_7 = register_r.get_bit(7);
                let mut new_value = register_r >> 1;
                new_value.set_bit(7, bit_7);

                flags.carry = register_r.get_bit(0);
                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

            // Instruction `SWAP r` - 00110rrr
            // Swap high four bits with low four bits
            0x30..=0x37 => {
                let register_r = regs.get_register(opcode, bus);
                let high = register_r >> 4;
                let low = register_r << 4;
                let new_value = high | low;

                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

            // Instruction `SRL r` - 00111rrr
            // Shift the contents of register r to the right and store bit 0 in carry flag
            0x38..=0x3F => {
                let register_r = regs.get_register(opcode, bus);
                let new_value = register_r >> 1;

                flags.carry = register_r.get_bit(0);
                flags.zero = new_value == 0;
                regs.set_register(opcode, new_value, bus);

                (1, 1)
            }

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
