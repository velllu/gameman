use crate::common::{merge_two_u8s_into_u16, split_u16_into_two_u8s};
use std::fmt::Debug;

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
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "A: {:x} B: {:x} C: {:x} D: {:x}\n\
            E: {:x} H: {:x} L: {:x}\n\
            SP: {:x} PC: {:x}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc
        )
    }
}

pub(crate) enum OneByteRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Registers {
    pub(crate) fn get_r(&mut self, register: OneByteRegister) -> &mut u8 {
        match register {
            OneByteRegister::A => &mut self.a,
            OneByteRegister::B => &mut self.b,
            OneByteRegister::C => &mut self.c,
            OneByteRegister::D => &mut self.d,
            OneByteRegister::E => &mut self.e,
            OneByteRegister::H => &mut self.h,
            OneByteRegister::L => &mut self.l,
        }
    }

    // All this `set_rr()` functions are done because we cannot have a `get_rr` as
    // registers are stored as a one byte register

    pub(crate) fn set_bc(&mut self, value: u16) {
        let (register_b, register_c) = split_u16_into_two_u8s(value);
        self.b = register_b;
        self.c = register_c;
    }

    pub(crate) fn get_bc(&self) -> u16 {
        merge_two_u8s_into_u16(self.b, self.c)
    }

    pub(crate) fn set_de(&mut self, value: u16) {
        let (register_d, register_e) = split_u16_into_two_u8s(value);
        self.d = register_d;
        self.e = register_e;
    }

    pub(crate) fn get_de(&self) -> u16 {
        merge_two_u8s_into_u16(self.d, self.e)
    }

    pub(crate) fn set_hl(&mut self, value: u16) {
        let (register_h, register_l) = split_u16_into_two_u8s(value);
        self.h = register_h;
        self.l = register_l;
    }

    pub(crate) fn get_hl(&self) -> u16 {
        merge_two_u8s_into_u16(self.h, self.l)
    }
}
