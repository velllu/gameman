use crate::registers::OneByteRegister;
use crate::registers::TwoByteRegister;
use crate::GameBoy;

// My preferred opcode reference is: https://meganesu.github.io/generate-gb-opcodes/

/// The number of bytes an opcode needs, examples:
/// - NOP - 1 byte, since it just takes the "NOP" byte, so every opcode has *at least* 1
/// byte
/// - LD BC, d16 - 2 bytes, since it also requires the byte after the opcode
type Bytes = u8;

/// The amount of "steps" the gameboy needs to execute a specific instruction
type Cycles = u8;

// You might be wondering why I did not use rust enums to represent all opcodes,
// I originally did that, and it transforms into spaghetti code really quick, and this is
// far more readable in my opinion, both to rust users, and to anyone that doesn't know
// anything about rust

// NAMING CONVENTIONS:
// r -> one byte register

// Utility functions
impl GameBoy {
    pub(crate) fn get_r(&mut self, register: OneByteRegister) -> &mut u8 {
        match register {
            OneByteRegister::A => &mut self.registers.a,
            OneByteRegister::B => &mut self.registers.b,
            OneByteRegister::C => &mut self.registers.c,
            OneByteRegister::D => &mut self.registers.d,
            OneByteRegister::E => &mut self.registers.e,
            OneByteRegister::H => &mut self.registers.h,
            OneByteRegister::L => &mut self.registers.l,
        }
    }

    pub(crate) fn update_z_flag(&mut self, result: u8) {
        if result == 0 {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }
    }
}

// INC/DEC functions
impl GameBoy {
    // TODO: DRY functions

    pub(crate) fn increment_8(&mut self, register: OneByteRegister, amount: u8) -> (Bytes, Cycles) {
        let register = self.get_r(register);
        *register = register.wrapping_add(amount);
        let result = *register;

        self.update_z_flag(result);

        (1, 1)
    }

    pub(crate) fn decrement_8(&mut self, register: OneByteRegister, amount: u8) -> (Bytes, Cycles) {
        let register = self.get_r(register);
        *register = register.wrapping_sub(amount);
        let result = *register;

        self.update_z_flag(result);

        (1, 1)
    }
}

// LD functions
impl GameBoy {
    pub(crate) fn ld_r_into_r(
        &mut self,
        register_to_be_loaded: OneByteRegister,
        register: OneByteRegister,
    ) -> (Bytes, Cycles) {
        let register_to_be_loaded = *self.get_r(register_to_be_loaded);
        let register = self.get_r(register);

        *register = register_to_be_loaded;

        (1, 1)
    }
}

impl GameBoy {
    pub(crate) fn interpret_cb_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        match opcode {
            _ => todo!(),
        }
    }

    #[rustfmt::skip]
    pub(crate) fn interpret_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        // The functions actually return the `(Bytes, Cycles)`

        // I did not choose macros because
        // - Each of them will expand, and make the binary "bigger" (by not a lot, but I
        // still find this less elegant)
        // - They are harder to debug, and harder to read for rust beginners

        match opcode {
            0x00 => (1, 1), // NOP, does nothing

            // INC R
            0x04 => self.increment_8(OneByteRegister::B, 1),
            0x0C => self.increment_8(OneByteRegister::C, 1),
            0x14 => self.increment_8(OneByteRegister::D, 1),
            0x1C => self.increment_8(OneByteRegister::E, 1),
            0x24 => self.increment_8(OneByteRegister::H, 1),
            0x2C => self.increment_8(OneByteRegister::L, 1),
            0x3C => self.increment_8(OneByteRegister::A, 1),

            // DEC R
            0x05 => self.decrement_8(OneByteRegister::B, 1),
            0x0D => self.decrement_8(OneByteRegister::C, 1),
            0x15 => self.decrement_8(OneByteRegister::D, 1),
            0x1D => self.decrement_8(OneByteRegister::E, 1),
            0x25 => self.decrement_8(OneByteRegister::H, 1),
            0x2D => self.decrement_8(OneByteRegister::L, 1),
            0x3D => self.decrement_8(OneByteRegister::A, 1),

            // LD R into R
            0x40 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::B),
            0x41 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::B),
            0x42 => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::B),
            0x43 => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::B),
            0x44 => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::B),
            0x45 => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::B),
            0x47 => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::B),
            0x48 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::C),
            0x49 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::C),
            0x4A => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::C),
            0x4B => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::C),
            0x4C => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::C),
            0x4D => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::C),
            0x4F => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::C),
            0x50 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::D),
            0x51 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::D),
            0x52 => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::D),
            0x53 => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::D),
            0x54 => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::D),
            0x55 => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::D),
            0x57 => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::D),
            0x58 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::E),
            0x59 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::E),
            0x5A => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::E),
            0x5B => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::E),
            0x5C => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::E),
            0x5D => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::E),
            0x5F => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::E),
            0x60 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::H),
            0x61 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::H),
            0x62 => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::H),
            0x63 => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::H),
            0x64 => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::H),
            0x65 => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::H),
            0x67 => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::H),
            0x68 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::L),
            0x69 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::L),
            0x6A => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::L),
            0x6B => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::L),
            0x6C => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::L),
            0x6D => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::L),
            0x6F => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::L),
            0x78 => self.ld_r_into_r(OneByteRegister::B, OneByteRegister::A),
            0x79 => self.ld_r_into_r(OneByteRegister::C, OneByteRegister::A),
            0x7A => self.ld_r_into_r(OneByteRegister::D, OneByteRegister::A),
            0x7B => self.ld_r_into_r(OneByteRegister::E, OneByteRegister::A),
            0x7C => self.ld_r_into_r(OneByteRegister::H, OneByteRegister::A),
            0x7D => self.ld_r_into_r(OneByteRegister::L, OneByteRegister::A),
            0x7F => self.ld_r_into_r(OneByteRegister::A, OneByteRegister::A),

            _ => panic!("Opcode {:x} not implemented yet", opcode),
        }
    }
}
