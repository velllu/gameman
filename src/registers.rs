use crate::{
    bus::Bus,
    common::{merge_two_u8s_into_u16, split_u16_into_two_u8s},
    flags::Flags,
};

#[derive(PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub(crate) fn get_register(&self, code: u8, bus: &Bus) -> u8 {
        match code & 0b00000111 {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => bus[merge_two_u8s_into_u16(self.h, self.l)],
            7 => self.a,

            _ => unreachable!(),
        }
    }

    pub(crate) fn set_register(&mut self, code: u8, value: u8, bus: &mut Bus) {
        match code & 0b00000111 {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => bus[merge_two_u8s_into_u16(self.h, self.l)] = value,
            7 => self.a = value,

            _ => unreachable!(),
        }
    }

    pub(crate) fn get_register_couple(&self, code: u8) -> u16 {
        match code & 0b00000011 {
            0 => merge_two_u8s_into_u16(self.b, self.c),
            1 => merge_two_u8s_into_u16(self.d, self.e),
            2 => merge_two_u8s_into_u16(self.h, self.l),
            3 => self.sp,

            _ => unreachable!(),
        }
    }

    /// Cases:
    /// - 0: Registers BC
    /// - 1: Registers DE
    /// - 2: Registers HL, and increment hl
    /// - 3: Registers HL, and decrement hl
    pub(crate) fn get_register_couple_with_increments(&mut self, code: u8) -> u16 {
        match code & 0b00000011 {
            0 => merge_two_u8s_into_u16(self.b, self.c),
            1 => merge_two_u8s_into_u16(self.d, self.e),

            2 => {
                let hl = merge_two_u8s_into_u16(self.h, self.l);
                (self.h, self.l) = split_u16_into_two_u8s(hl.wrapping_add(1));
                hl
            }

            3 => {
                let hl = merge_two_u8s_into_u16(self.h, self.l);
                (self.h, self.l) = split_u16_into_two_u8s(hl.wrapping_sub(1));
                hl
            }

            _ => unreachable!(),
        }
    }

    pub(crate) fn get_register_couple_with_flags(&self, code: u8, flags: &Flags) -> u16 {
        match code & 0b00000011 {
            0 => merge_two_u8s_into_u16(self.b, self.c),
            1 => merge_two_u8s_into_u16(self.d, self.e),
            2 => merge_two_u8s_into_u16(self.h, self.l),
            3 => merge_two_u8s_into_u16(self.a, flags.get_byte()),

            _ => unreachable!(),
        }
    }

    pub(crate) fn set_register_couple_with_flags(
        &mut self,
        code: u8,
        value: u16,
        flags: &mut Flags,
    ) {
        match code & 0b00000011 {
            0 => (self.b, self.c) = split_u16_into_two_u8s(value),
            1 => (self.d, self.e) = split_u16_into_two_u8s(value),
            2 => (self.h, self.l) = split_u16_into_two_u8s(value),
            3 => {
                let (a, flag) = split_u16_into_two_u8s(value);
                self.a = a;
                flags.set_from_byte(flag);
            }

            _ => unreachable!(),
        }
    }

    pub(crate) fn set_register_couple(&mut self, code: u8, value: u16) {
        match code & 0b00000011 {
            0 => (self.b, self.c) = split_u16_into_two_u8s(value),
            1 => (self.d, self.e) = split_u16_into_two_u8s(value),
            2 => (self.h, self.l) = split_u16_into_two_u8s(value),
            3 => self.sp = value,

            _ => unreachable!(),
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
