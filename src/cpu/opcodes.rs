#![warn(clippy::match_same_arms)]

use crate::{
    bus::Bus,
    common::merge_two_u8s_into_u16,
    flags::Flags,
    registers::{Register, Registers},
};

use super::{
    instructions::{
        add_rr_to_rr, decrement_r, decrement_rr, increment_r, increment_rr, load_ii_into_rr,
        load_r_into_ram, Bytes, Cycles,
    },
    Cpu,
};

impl Cpu {
    #[rustfmt::skip]
    pub fn interpret_opcode(
        &mut self,
        opcode: u8,
        flags: &mut Flags,
        regs: &mut Registers,
        bus: &mut Bus,
    ) -> (Bytes, Cycles) {
        // This lambda fetches `offset` bytes from the bus
        let get_immediate_data = |offset| bus.read_from_rom(regs.pc.wrapping_add(offset));

        let i = get_immediate_data(1); // One byte after pc
        let ii = merge_two_u8s_into_u16(get_immediate_data(2), i); // The two bytes after pc

        match opcode {
            0x00 => (1, 0), // NOP

            0x01 => load_ii_into_rr(&mut (regs.b, regs.c), ii),
            0x11 => load_ii_into_rr(&mut (regs.d, regs.e), ii),
            0x21 => load_ii_into_rr(&mut (regs.h, regs.l), ii),
            0x31 => load_ii_into_rr(&mut regs.sp, ii),

            0x02 => load_r_into_ram((regs.b, regs.c), regs.a, bus),
            0x12 => load_r_into_ram((regs.d, regs.e), regs.a, bus),
            0x70 => load_r_into_ram((regs.h, regs.l), regs.b, bus),
            0x71 => load_r_into_ram((regs.h, regs.l), regs.c, bus),
            0x72 => load_r_into_ram((regs.h, regs.l), regs.d, bus),
            0x73 => load_r_into_ram((regs.h, regs.l), regs.e, bus),
            0x74 => load_r_into_ram((regs.h, regs.l), regs.h, bus),
            0x75 => load_r_into_ram((regs.h, regs.l), regs.l, bus),
            0x77 => load_r_into_ram((regs.h, regs.l), regs.a, bus),

            0x03 => increment_rr(&mut (regs.b, regs.c)),
            0x13 => increment_rr(&mut (regs.d, regs.e)),
            0x23 => increment_rr(&mut (regs.h, regs.l)),
            0x33 => increment_rr(&mut regs.sp),

            0x04 => increment_r(&mut regs.b, flags),
            0x14 => increment_r(&mut regs.d, flags),
            0x24 => increment_r(&mut regs.h, flags),
            0x0C => increment_r(&mut regs.c, flags),
            0x1C => increment_r(&mut regs.e, flags),
            0x2C => increment_r(&mut regs.l, flags),
            0x3C => increment_r(&mut regs.a, flags),

            0x05 => decrement_r(&mut regs.b, flags),
            0x15 => decrement_r(&mut regs.d, flags),
            0x25 => decrement_r(&mut regs.h, flags),
            0x0D => decrement_r(&mut regs.c, flags),
            0x1D => decrement_r(&mut regs.e, flags),
            0x2D => decrement_r(&mut regs.l, flags),
            0x3D => decrement_r(&mut regs.a, flags),

            0x06 => { regs.b = i; (2, 2) },
            0x16 => { regs.d = i; (2, 2) },
            0x26 => { regs.h = i; (2, 2) },
            0x0E => { regs.c = i; (2, 2) },
            0x1E => { regs.e = i; (2, 2) },
            0x2E => { regs.l = i; (2, 2) },
            0x3E => { regs.a = i; (2, 2) },

            0x09 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.b, regs.c), flags),
            0x19 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.d, regs.e), flags),
            0x29 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.h, regs.l), flags),
            0x39 => add_rr_to_rr(&mut (regs.h, regs.l), regs.sp, flags),

            0x0B => decrement_rr(&mut (regs.b, regs.c)),
            0x1B => decrement_rr(&mut (regs.d, regs.e)),
            0x2B => decrement_rr(&mut (regs.h, regs.l)),
            0x3B => decrement_rr(&mut regs.sp),

            0x40 => { regs.b = regs.b; (1, 1) },
            0x41 => { regs.b = regs.c; (1, 1) },
            0x42 => { regs.b = regs.d; (1, 1) },
            0x43 => { regs.b = regs.e; (1, 1) },
            0x44 => { regs.b = regs.h; (1, 1) },
            0x45 => { regs.b = regs.l; (1, 1) },
            0x47 => { regs.b = regs.a; (1, 1) },
            0x48 => { regs.c = regs.b; (1, 1) },
            0x49 => { regs.c = regs.c; (1, 1) },
            0x4A => { regs.c = regs.d; (1, 1) },
            0x4B => { regs.c = regs.e; (1, 1) },
            0x4C => { regs.c = regs.h; (1, 1) },
            0x4D => { regs.c = regs.l; (1, 1) },
            0x4F => { regs.c = regs.a; (1, 1) },
            0x50 => { regs.d = regs.b; (1, 1) },
            0x51 => { regs.d = regs.c; (1, 1) },
            0x52 => { regs.d = regs.d; (1, 1) },
            0x53 => { regs.d = regs.e; (1, 1) },
            0x54 => { regs.d = regs.h; (1, 1) },
            0x55 => { regs.d = regs.l; (1, 1) },
            0x57 => { regs.d = regs.a; (1, 1) },
            0x58 => { regs.e = regs.b; (1, 1) },
            0x59 => { regs.e = regs.c; (1, 1) },
            0x5A => { regs.e = regs.d; (1, 1) },
            0x5B => { regs.e = regs.e; (1, 1) },
            0x5C => { regs.e = regs.h; (1, 1) },
            0x5D => { regs.e = regs.l; (1, 1) },
            0x5F => { regs.e = regs.a; (1, 1) },
            0x60 => { regs.h = regs.b; (1, 1) },
            0x61 => { regs.h = regs.c; (1, 1) },
            0x62 => { regs.h = regs.d; (1, 1) },
            0x63 => { regs.h = regs.e; (1, 1) },
            0x64 => { regs.h = regs.h; (1, 1) },
            0x65 => { regs.h = regs.l; (1, 1) },
            0x67 => { regs.h = regs.a; (1, 1) },
            0x68 => { regs.l = regs.b; (1, 1) },
            0x69 => { regs.l = regs.c; (1, 1) },
            0x6A => { regs.l = regs.d; (1, 1) },
            0x6B => { regs.l = regs.e; (1, 1) },
            0x6C => { regs.l = regs.h; (1, 1) },
            0x6D => { regs.l = regs.l; (1, 1) },
            0x6F => { regs.l = regs.a; (1, 1) },
            0x78 => { regs.a = regs.b; (1, 1) },
            0x79 => { regs.a = regs.c; (1, 1) },
            0x7A => { regs.a = regs.d; (1, 1) },
            0x7B => { regs.a = regs.e; (1, 1) },
            0x7C => { regs.a = regs.h; (1, 1) },
            0x7D => { regs.a = regs.l; (1, 1) },
            0x7F => { regs.a = regs.a; (1, 1) },

            0xC2 => if !flags.zero { regs.pc = ii; (0, 4) } else { (3, 3) },
            0xD2 => if !flags.carry { regs.pc = ii; (0, 4) } else { (3, 3) },
            0xCA => if flags.zero { regs.pc = ii; (0, 4) } else { (3, 3) },
            0xDA => if flags.carry { regs.pc = ii; (0, 4) } else { (3, 3) },

            0xC3 => { regs.pc = ii; (0, 4) },
            0xE9 => { regs.pc = (regs.h, regs.l).get(); (0, 1) },

            0xF3 => { self.ime = false; (1, 1) },
            0xFB => { self.ime = true; (1, 1) },

            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}
