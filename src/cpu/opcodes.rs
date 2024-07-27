use crate::{bus::Bus, common::merge_two_u8s_into_u16, flags::Flags, registers::Registers};

use super::{
    instructions::{add_rr_to_rr, increment_rr, load_ii_into_rr, load_r_into_ram, Bytes, Cycles},
    Cpu,
};

impl Cpu {
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
            0x00 => (0, 0), // NOP

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

            0x09 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.b, regs.c), flags),
            0x19 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.d, regs.e), flags),
            0x29 => add_rr_to_rr(&mut (regs.h, regs.l), (regs.h, regs.l), flags),
            0x39 => add_rr_to_rr(&mut (regs.h, regs.l), regs.sp, flags),

            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}
