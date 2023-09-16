use crate::common::{merge_two_u8s_into_u16, split_u16_into_two_u8s, Operator};
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

macro_rules! set_rr {
    ($name:ident, $first_r:ident, $second_r:ident) => {
        pub(crate) fn $name(&mut self, value: u16) {
            let (register1, register2) = split_u16_into_two_u8s(value);
            self.$first_r = register1;
            self.$second_r = register2;
        }
    };
}

macro_rules! get_rr {
    ($name:ident, $first_r:ident, $second_r:ident) => {
        pub(crate) fn $name(&self) -> u16 {
            merge_two_u8s_into_u16(self.$first_r, self.$second_r)
        }
    };
}

macro_rules! increment_rr {
    ($name:ident, $first_r:ident, $second_r:ident) => {
        pub(crate) fn $name(&mut self, value: u16, operation: Operator) {
            let rr = merge_two_u8s_into_u16(self.$first_r, self.$second_r);
            let rr = match operation {
                Operator::Inc => rr.wrapping_add(value),
                Operator::Sub => rr.wrapping_sub(value),
            };

            let (r1, r2) = split_u16_into_two_u8s(rr);

            self.$first_r = r1;
            self.$second_r = r2;
        }
    };
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

    // All this `set_rr()`, `get_rr()`, `increment_rr()` functions are done because we
    // cannot have a `get_rr` as registers are stored as a one byte register

    set_rr! {set_bc, b, c}
    get_rr! {get_bc, b, c}
    increment_rr! {increment_bc, b, c}

    set_rr! {set_de, d, e}
    get_rr! {get_de, d, e}
    increment_rr! {increment_de, d, e}

    set_rr! {set_hl, h, l}
    get_rr! {get_hl, h, l}
    increment_rr! {increment_hl, h, l}
}
