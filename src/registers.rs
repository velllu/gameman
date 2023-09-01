use std::fmt::{write, Debug};

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
