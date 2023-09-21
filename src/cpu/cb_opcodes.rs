use crate::{common::get_bit, GameBoy};

use super::{Bytes, Cycles};

impl GameBoy {
    fn bit(&mut self, value: u8, offset: u8) {
        self.flags.zero = get_bit(value, offset);
    }
}

impl GameBoy {
    #[rustfmt::skip]
    pub(crate) fn interpret_cb_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        match opcode {
            // BIT <bit offset>, R
            0x40 => { self.bit(self.registers.b, 0); (1, 2) },
            0x41 => { self.bit(self.registers.c, 0); (1, 2) },
            0x42 => { self.bit(self.registers.d, 0); (1, 2) },
            0x43 => { self.bit(self.registers.e, 0); (1, 2) },
            0x44 => { self.bit(self.registers.h, 0); (1, 2) },
            0x45 => { self.bit(self.registers.l, 0); (1, 2) },
            0x47 => { self.bit(self.registers.a, 0); (1, 2) },
            0x48 => { self.bit(self.registers.b, 1); (1, 2) },
            0x49 => { self.bit(self.registers.c, 1); (1, 2) },
            0x4A => { self.bit(self.registers.d, 1); (1, 2) },
            0x4B => { self.bit(self.registers.e, 1); (1, 2) },
            0x4C => { self.bit(self.registers.h, 1); (1, 2) },
            0x4D => { self.bit(self.registers.l, 1); (1, 2) },
            0x4F => { self.bit(self.registers.a, 1); (1, 2) },
            0x50 => { self.bit(self.registers.b, 2); (1, 2) },
            0x51 => { self.bit(self.registers.c, 2); (1, 2) },
            0x52 => { self.bit(self.registers.d, 2); (1, 2) },
            0x53 => { self.bit(self.registers.e, 2); (1, 2) },
            0x54 => { self.bit(self.registers.h, 2); (1, 2) },
            0x55 => { self.bit(self.registers.l, 2); (1, 2) },
            0x57 => { self.bit(self.registers.a, 2); (1, 2) },
            0x58 => { self.bit(self.registers.b, 3); (1, 2) },
            0x59 => { self.bit(self.registers.c, 3); (1, 2) },
            0x5A => { self.bit(self.registers.d, 3); (1, 2) },
            0x5B => { self.bit(self.registers.e, 3); (1, 2) },
            0x5C => { self.bit(self.registers.h, 3); (1, 2) },
            0x5D => { self.bit(self.registers.l, 3); (1, 2) },
            0x5F => { self.bit(self.registers.a, 3); (1, 2) },
            0x60 => { self.bit(self.registers.b, 4); (1, 2) },
            0x61 => { self.bit(self.registers.c, 4); (1, 2) },
            0x62 => { self.bit(self.registers.d, 4); (1, 2) },
            0x63 => { self.bit(self.registers.e, 4); (1, 2) },
            0x64 => { self.bit(self.registers.h, 4); (1, 2) },
            0x65 => { self.bit(self.registers.l, 4); (1, 2) },
            0x67 => { self.bit(self.registers.a, 4); (1, 2) },
            0x68 => { self.bit(self.registers.b, 5); (1, 2) },
            0x69 => { self.bit(self.registers.c, 5); (1, 2) },
            0x6A => { self.bit(self.registers.d, 5); (1, 2) },
            0x6B => { self.bit(self.registers.e, 5); (1, 2) },
            0x6C => { self.bit(self.registers.h, 5); (1, 2) },
            0x6D => { self.bit(self.registers.l, 5); (1, 2) },
            0x6F => { self.bit(self.registers.a, 5); (1, 2) },
            0x70 => { self.bit(self.registers.b, 6); (1, 2) },
            0x71 => { self.bit(self.registers.c, 6); (1, 2) },
            0x72 => { self.bit(self.registers.d, 6); (1, 2) },
            0x73 => { self.bit(self.registers.e, 6); (1, 2) },
            0x74 => { self.bit(self.registers.h, 6); (1, 2) },
            0x75 => { self.bit(self.registers.l, 6); (1, 2) },
            0x77 => { self.bit(self.registers.a, 6); (1, 2) },
            0x78 => { self.bit(self.registers.b, 7); (1, 2) },
            0x79 => { self.bit(self.registers.c, 7); (1, 2) },
            0x7A => { self.bit(self.registers.d, 7); (1, 2) },
            0x7B => { self.bit(self.registers.e, 7); (1, 2) },
            0x7C => { self.bit(self.registers.h, 7); (1, 2) },
            0x7D => { self.bit(self.registers.l, 7); (1, 2) },
            0x7F => { self.bit(self.registers.a, 7); (1, 2) },

            // BIT <bit offset>, RAM
            0x46 => { self.bit(self.bus[self.registers.get_hl()], 0); (1, 3) },
            0x4E => { self.bit(self.bus[self.registers.get_hl()], 1); (1, 3) },
            0x56 => { self.bit(self.bus[self.registers.get_hl()], 2); (1, 3) },
            0x5E => { self.bit(self.bus[self.registers.get_hl()], 3); (1, 3) },
            0x66 => { self.bit(self.bus[self.registers.get_hl()], 4); (1, 3) },
            0x6E => { self.bit(self.bus[self.registers.get_hl()], 5); (1, 3) },
            0x76 => { self.bit(self.bus[self.registers.get_hl()], 6); (1, 3) },
            0x7E => { self.bit(self.bus[self.registers.get_hl()], 7); (1, 3) },

            _ => panic!("Opcode 0xcb{:x} not implemented yet", opcode),
        }
    }
}
