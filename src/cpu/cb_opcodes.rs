use crate::{common::Bit, GameBoy};

use super::{Bytes, Cycles};

impl GameBoy {
    fn bit_r(&mut self, value: u8, offset: u8) {
        self.flags.zero = value.get_bit(offset);

        self.flags
            .update_zero_flag(if self.flags.zero { 1 } else { 0 });
    }

    fn reset_ram(&mut self, address: u16, offset: u8) {
        let mut byte_from_ram = self.bus[address].clone();
        byte_from_ram.set_bit(offset, false);

        self.bus[address] = byte_from_ram;
    }
}

impl GameBoy {
    #[rustfmt::skip]
    pub(crate) fn interpret_cb_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        match opcode {
            // BIT R, <bit offset>
            0x40 => { self.bit_r(self.registers.b, 0); (1, 2) },
            0x41 => { self.bit_r(self.registers.c, 0); (1, 2) },
            0x42 => { self.bit_r(self.registers.d, 0); (1, 2) },
            0x43 => { self.bit_r(self.registers.e, 0); (1, 2) },
            0x44 => { self.bit_r(self.registers.h, 0); (1, 2) },
            0x45 => { self.bit_r(self.registers.l, 0); (1, 2) },
            0x47 => { self.bit_r(self.registers.a, 0); (1, 2) },
            0x48 => { self.bit_r(self.registers.b, 1); (1, 2) },
            0x49 => { self.bit_r(self.registers.c, 1); (1, 2) },
            0x4A => { self.bit_r(self.registers.d, 1); (1, 2) },
            0x4B => { self.bit_r(self.registers.e, 1); (1, 2) },
            0x4C => { self.bit_r(self.registers.h, 1); (1, 2) },
            0x4D => { self.bit_r(self.registers.l, 1); (1, 2) },
            0x4F => { self.bit_r(self.registers.a, 1); (1, 2) },
            0x50 => { self.bit_r(self.registers.b, 2); (1, 2) },
            0x51 => { self.bit_r(self.registers.c, 2); (1, 2) },
            0x52 => { self.bit_r(self.registers.d, 2); (1, 2) },
            0x53 => { self.bit_r(self.registers.e, 2); (1, 2) },
            0x54 => { self.bit_r(self.registers.h, 2); (1, 2) },
            0x55 => { self.bit_r(self.registers.l, 2); (1, 2) },
            0x57 => { self.bit_r(self.registers.a, 2); (1, 2) },
            0x58 => { self.bit_r(self.registers.b, 3); (1, 2) },
            0x59 => { self.bit_r(self.registers.c, 3); (1, 2) },
            0x5A => { self.bit_r(self.registers.d, 3); (1, 2) },
            0x5B => { self.bit_r(self.registers.e, 3); (1, 2) },
            0x5C => { self.bit_r(self.registers.h, 3); (1, 2) },
            0x5D => { self.bit_r(self.registers.l, 3); (1, 2) },
            0x5F => { self.bit_r(self.registers.a, 3); (1, 2) },
            0x60 => { self.bit_r(self.registers.b, 4); (1, 2) },
            0x61 => { self.bit_r(self.registers.c, 4); (1, 2) },
            0x62 => { self.bit_r(self.registers.d, 4); (1, 2) },
            0x63 => { self.bit_r(self.registers.e, 4); (1, 2) },
            0x64 => { self.bit_r(self.registers.h, 4); (1, 2) },
            0x65 => { self.bit_r(self.registers.l, 4); (1, 2) },
            0x67 => { self.bit_r(self.registers.a, 4); (1, 2) },
            0x68 => { self.bit_r(self.registers.b, 5); (1, 2) },
            0x69 => { self.bit_r(self.registers.c, 5); (1, 2) },
            0x6A => { self.bit_r(self.registers.d, 5); (1, 2) },
            0x6B => { self.bit_r(self.registers.e, 5); (1, 2) },
            0x6C => { self.bit_r(self.registers.h, 5); (1, 2) },
            0x6D => { self.bit_r(self.registers.l, 5); (1, 2) },
            0x6F => { self.bit_r(self.registers.a, 5); (1, 2) },
            0x70 => { self.bit_r(self.registers.b, 6); (1, 2) },
            0x71 => { self.bit_r(self.registers.c, 6); (1, 2) },
            0x72 => { self.bit_r(self.registers.d, 6); (1, 2) },
            0x73 => { self.bit_r(self.registers.e, 6); (1, 2) },
            0x74 => { self.bit_r(self.registers.h, 6); (1, 2) },
            0x75 => { self.bit_r(self.registers.l, 6); (1, 2) },
            0x77 => { self.bit_r(self.registers.a, 6); (1, 2) },
            0x78 => { self.bit_r(self.registers.b, 7); (1, 2) },
            0x79 => { self.bit_r(self.registers.c, 7); (1, 2) },
            0x7A => { self.bit_r(self.registers.d, 7); (1, 2) },
            0x7B => { self.bit_r(self.registers.e, 7); (1, 2) },
            0x7C => { self.bit_r(self.registers.h, 7); (1, 2) },
            0x7D => { self.bit_r(self.registers.l, 7); (1, 2) },
            0x7F => { self.bit_r(self.registers.a, 7); (1, 2) },

            // BIT RAM, <bit offset>
            0x46 => { self.bit_r(self.bus[self.registers.get_hl()], 0); (1, 3) },
            0x4E => { self.bit_r(self.bus[self.registers.get_hl()], 1); (1, 3) },
            0x56 => { self.bit_r(self.bus[self.registers.get_hl()], 2); (1, 3) },
            0x5E => { self.bit_r(self.bus[self.registers.get_hl()], 3); (1, 3) },
            0x66 => { self.bit_r(self.bus[self.registers.get_hl()], 4); (1, 3) },
            0x6E => { self.bit_r(self.bus[self.registers.get_hl()], 5); (1, 3) },
            0x76 => { self.bit_r(self.bus[self.registers.get_hl()], 6); (1, 3) },
            0x7E => { self.bit_r(self.bus[self.registers.get_hl()], 7); (1, 3) },

            // RESET RAM, <bit offset>
            0x86 => { self.reset_ram(self.registers.get_hl(), 0); (1, 4) }
            0x8E => { self.reset_ram(self.registers.get_hl(), 1); (1, 4) }
            0x96 => { self.reset_ram(self.registers.get_hl(), 2); (1, 4) }
            0x9E => { self.reset_ram(self.registers.get_hl(), 3); (1, 4) }
            0xA6 => { self.reset_ram(self.registers.get_hl(), 4); (1, 4) }
            0xAE => { self.reset_ram(self.registers.get_hl(), 5); (1, 4) }
            0xB6 => { self.reset_ram(self.registers.get_hl(), 6); (1, 4) }
            0xBE => { self.reset_ram(self.registers.get_hl(), 7); (1, 4) }

            _ => panic!("Opcode 0xcb{:x} not implemented yet", opcode),
        }
    }
}
