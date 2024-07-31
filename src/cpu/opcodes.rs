#![warn(clippy::match_same_arms)]

use crate::{
    bus::Bus,
    common::merge_two_u8s_into_u16,
    flags::Flags,
    registers::{ReadRegister, ReadWriteRegister, Registers},
};

use super::{
    instructions::{
        add_rr_to_rr, call, decrement_r, decrement_rr, increment_r, increment_rr, relative_jump,
        return_, Bytes, Cycles,
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

            0x01 => { (&mut regs.b, &mut regs.c).set(ii); (3, 3) },
            0x11 => { (&mut regs.d, &mut regs.e).set(ii); (3, 3) },
            0x21 => { (&mut regs.h, &mut regs.l).set(ii); (3, 3) },
            0x31 => { regs.sp = ii; (3, 3) },

            0x02 => { bus[(regs.b, regs.c).get()] = regs.a; (1, 2) },
            0x12 => { bus[(regs.d, regs.e).get()] = regs.a; (1, 2) },
            0x70 => { bus[(regs.h, regs.l).get()] = regs.b; (1, 2) },
            0x71 => { bus[(regs.h, regs.l).get()] = regs.c; (1, 2) },
            0x72 => { bus[(regs.h, regs.l).get()] = regs.d; (1, 2) },
            0x73 => { bus[(regs.h, regs.l).get()] = regs.e; (1, 2) },
            0x74 => { bus[(regs.h, regs.l).get()] = regs.h; (1, 2) },
            0x75 => { bus[(regs.h, regs.l).get()] = regs.l; (1, 2) },
            0x77 => { bus[(regs.h, regs.l).get()] = regs.a; (1, 2) },

            0x03 => increment_rr((&mut regs.b, &mut regs.c)),
            0x13 => increment_rr((&mut regs.d, &mut regs.e)),
            0x23 => increment_rr((&mut regs.h, &mut regs.l)),
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

            0x09 => add_rr_to_rr((&mut regs.h, &mut regs.l), (regs.b, regs.c), flags),
            0x19 => add_rr_to_rr((&mut regs.h, &mut regs.l), (regs.d, regs.e), flags),
            0x29 => { let hl = (regs.h, regs.l); add_rr_to_rr((&mut regs.h, &mut regs.l), hl, flags) },
            0x39 => add_rr_to_rr((&mut regs.h, &mut regs.l), regs.sp, flags),

            0x0B => decrement_rr((&mut regs.b, &mut regs.c)),
            0x1B => decrement_rr((&mut regs.d, &mut regs.e)),
            0x2B => decrement_rr((&mut regs.h, &mut regs.l)),
            0x3B => decrement_rr(&mut regs.sp),

            0x18 => relative_jump(i, regs),
            0x20 => if !flags.zero { relative_jump(i, regs) } else { (2, 2) },
            0x30 => if !flags.carry { relative_jump(i, regs) } else { (2, 2) },
            0x28 => if flags.zero { relative_jump(i, regs) } else { (2, 2) },
            0x38 => if flags.carry { relative_jump(i, regs) } else { (2, 2) },

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

            0xC9 => return_(regs, bus),
            0xC0 => if !flags.zero { return_(regs, bus) } else { (2, 2) },
            0xD0 => if !flags.carry { return_(regs, bus) } else { (2, 2) },
            0xC8 => if flags.zero { return_(regs, bus) } else { (2, 2) },
            0xD8 => if flags.carry { return_(regs, bus) } else { (2, 2) },

            0xCD => call(ii, regs, bus),
            0xC4 => if !flags.zero { call(ii, regs, bus) } else { (3, 3) },
            0xD4 => if !flags.carry { call(ii, regs, bus) } else { (3, 3) },
            0xCC => if flags.zero { call(ii, regs, bus) } else { (3, 3) },
            0xDC => if flags.carry { call(ii, regs, bus) } else { (3, 3) },

            0xD6 => { regs.a = regs.a.wrapping_sub(i); (2, 2) },

            0xE0 => { bus[0xFF00 + i as u16] = regs.a; (2, 3) },
            0xE2 => { bus[0xFF00 + regs.c as u16] = regs.a; (2, 3) },

            0xEA => { bus[ii] = regs.a; (3, 4) },

            0xF3 => { self.ime = false; (1, 1) },
            0xFB => { self.ime = true; (1, 1) },

            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}
