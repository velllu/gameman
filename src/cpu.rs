use crate::common::merge_two_u8s_into_u16;
use crate::common::split_u16_into_two_u8s;
use crate::registers::OneByteRegister;
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
// rr -> two byte register
// ii -> the two bytes of immediate data
// ii -> the first byte of immediate data

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

    // All this `set_rr()` functions are done because we cannot have a `get_rr` as
    // registers are stored as a one byte register

    pub(crate) fn set_bc(&mut self, value: u16) {
        let (register_b, register_c) = split_u16_into_two_u8s(value);
        self.registers.b = register_b;
        self.registers.c = register_c;
    }

    pub(crate) fn set_de(&mut self, value: u16) {
        let (register_d, register_e) = split_u16_into_two_u8s(value);
        self.registers.d = register_d;
        self.registers.e = register_e;
    }

    pub(crate) fn set_hl(&mut self, value: u16) {
        let (register_h, register_l) = split_u16_into_two_u8s(value);
        self.registers.h = register_h;
        self.registers.l = register_l;
    }

    pub(crate) fn update_zero_flag(&mut self, result: u8) {
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

        self.update_zero_flag(result);

        (1, 1)
    }

    pub(crate) fn decrement_8(&mut self, register: OneByteRegister, amount: u8) -> (Bytes, Cycles) {
        let register = self.get_r(register);
        *register = register.wrapping_sub(amount);
        let result = *register;

        self.update_zero_flag(result);

        (1, 1)
    }
}

// LD functions
impl GameBoy {
    pub(crate) fn load_r_into_r(
        &mut self,
        register_to_be_loaded: OneByteRegister,
        register: OneByteRegister,
    ) -> (Bytes, Cycles) {
        let register_to_be_loaded = *self.get_r(register_to_be_loaded);
        let register = self.get_r(register);

        *register = register_to_be_loaded;

        (1, 1)
    }

    pub(crate) fn load_i_into_r(&mut self, register: OneByteRegister) -> (Bytes, Cycles) {
        let i = self.next(1);

        let register = self.get_r(register);
        *register = i;

        (2, 2)
    }
}

// Bitwise operation functions (not all of them as of now)
impl GameBoy {
    pub(crate) fn xor_r(&mut self, register: OneByteRegister) -> (Bytes, Cycles) {
        let register_a = *self.get_r(OneByteRegister::A);
        let register = self.get_r(register);

        let result = *register ^ register_a;
        *register = result;

        self.update_zero_flag(result);

        (1, 1)
    }
}

// Other functions
impl GameBoy {
    pub(crate) fn jump(&mut self, address: u16) {
        self.registers.pc = address;
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
        // I did not choose macros because
        // - Each of them will expand, and make the binary "bigger" (by not a lot, but I
        // still find this less elegant)
        // - They are harder to debug, and harder to read for rust beginners

        match opcode {
            0x00 => (1, 1), // NOP, does nothing

            // Increment R
            0x04 => self.increment_8(OneByteRegister::B, 1),
            0x0C => self.increment_8(OneByteRegister::C, 1),
            0x14 => self.increment_8(OneByteRegister::D, 1),
            0x1C => self.increment_8(OneByteRegister::E, 1),
            0x24 => self.increment_8(OneByteRegister::H, 1),
            0x2C => self.increment_8(OneByteRegister::L, 1),
            0x3C => self.increment_8(OneByteRegister::A, 1),

            // Decrement R
            0x05 => self.decrement_8(OneByteRegister::B, 1),
            0x0D => self.decrement_8(OneByteRegister::C, 1),
            0x15 => self.decrement_8(OneByteRegister::D, 1),
            0x1D => self.decrement_8(OneByteRegister::E, 1),
            0x25 => self.decrement_8(OneByteRegister::H, 1),
            0x2D => self.decrement_8(OneByteRegister::L, 1),
            0x3D => self.decrement_8(OneByteRegister::A, 1),

            // Load R into R
            0x40 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::B),
            0x41 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::B),
            0x42 => self.load_r_into_r(OneByteRegister::D, OneByteRegister::B),
            0x43 => self.load_r_into_r(OneByteRegister::E, OneByteRegister::B),
            0x44 => self.load_r_into_r(OneByteRegister::H, OneByteRegister::B),
            0x45 => self.load_r_into_r(OneByteRegister::L, OneByteRegister::B),
            0x47 => self.load_r_into_r(OneByteRegister::A, OneByteRegister::B),
            0x48 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::C),
            0x49 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::C),
            0x4A => self.load_r_into_r(OneByteRegister::D, OneByteRegister::C),
            0x4B => self.load_r_into_r(OneByteRegister::E, OneByteRegister::C),
            0x4C => self.load_r_into_r(OneByteRegister::H, OneByteRegister::C),
            0x4D => self.load_r_into_r(OneByteRegister::L, OneByteRegister::C),
            0x4F => self.load_r_into_r(OneByteRegister::A, OneByteRegister::C),
            0x50 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::D),
            0x51 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::D),
            0x52 => self.load_r_into_r(OneByteRegister::D, OneByteRegister::D),
            0x53 => self.load_r_into_r(OneByteRegister::E, OneByteRegister::D),
            0x54 => self.load_r_into_r(OneByteRegister::H, OneByteRegister::D),
            0x55 => self.load_r_into_r(OneByteRegister::L, OneByteRegister::D),
            0x57 => self.load_r_into_r(OneByteRegister::A, OneByteRegister::D),
            0x58 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::E),
            0x59 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::E),
            0x5A => self.load_r_into_r(OneByteRegister::D, OneByteRegister::E),
            0x5B => self.load_r_into_r(OneByteRegister::E, OneByteRegister::E),
            0x5C => self.load_r_into_r(OneByteRegister::H, OneByteRegister::E),
            0x5D => self.load_r_into_r(OneByteRegister::L, OneByteRegister::E),
            0x5F => self.load_r_into_r(OneByteRegister::A, OneByteRegister::E),
            0x60 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::H),
            0x61 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::H),
            0x62 => self.load_r_into_r(OneByteRegister::D, OneByteRegister::H),
            0x63 => self.load_r_into_r(OneByteRegister::E, OneByteRegister::H),
            0x64 => self.load_r_into_r(OneByteRegister::H, OneByteRegister::H),
            0x65 => self.load_r_into_r(OneByteRegister::L, OneByteRegister::H),
            0x67 => self.load_r_into_r(OneByteRegister::A, OneByteRegister::H),
            0x68 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::L),
            0x69 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::L),
            0x6A => self.load_r_into_r(OneByteRegister::D, OneByteRegister::L),
            0x6B => self.load_r_into_r(OneByteRegister::E, OneByteRegister::L),
            0x6C => self.load_r_into_r(OneByteRegister::H, OneByteRegister::L),
            0x6D => self.load_r_into_r(OneByteRegister::L, OneByteRegister::L),
            0x6F => self.load_r_into_r(OneByteRegister::A, OneByteRegister::L),
            0x78 => self.load_r_into_r(OneByteRegister::B, OneByteRegister::A),
            0x79 => self.load_r_into_r(OneByteRegister::C, OneByteRegister::A),
            0x7A => self.load_r_into_r(OneByteRegister::D, OneByteRegister::A),
            0x7B => self.load_r_into_r(OneByteRegister::E, OneByteRegister::A),
            0x7C => self.load_r_into_r(OneByteRegister::H, OneByteRegister::A),
            0x7D => self.load_r_into_r(OneByteRegister::L, OneByteRegister::A),
            0x7F => self.load_r_into_r(OneByteRegister::A, OneByteRegister::A),

            // Load II into RR
            0x01 => { self.set_bc(self.next_two()); (3, 3) },
            0x11 => { self.set_de(self.next_two()); (3, 3) },
            0x21 => { self.set_hl(self.next_two()); (3, 3) },
            0x31 => { self.registers.sp = self.next_two(); (3, 3) },

            // Load I into R
            0x06 => self.load_i_into_r(OneByteRegister::B),
            0x0E => self.load_i_into_r(OneByteRegister::C),
            0x16 => self.load_i_into_r(OneByteRegister::D),
            0x1E => self.load_i_into_r(OneByteRegister::E),
            0x26 => self.load_i_into_r(OneByteRegister::H),
            0x2E => self.load_i_into_r(OneByteRegister::L),
            0x3E => self.load_i_into_r(OneByteRegister::A),

            // Jump
            // When we jump, we set 0 bytes, because if we returned the "correct" amount
            // of bytes, the program will add them to PC after the jump
            0xC3 => { self.jump(self.next_two()); (0, 4) },
            0xC2 =>
                if !self.flags.zero { self.jump(self.next_two()); (0, 4) }
                else { (3, 3) },
            0xCA =>
                if self.flags.zero { self.jump(self.next_two()); (0, 4) }
                else { (3, 3) },
            0xD2 =>
                if !self.flags.carry { self.jump(self.next_two()); (0, 4) }
                else { (3, 3) },
            0xDA =>
                if self.flags.carry { self.jump(self.next_two()); (0, 4) }
                else { (3, 3) },
            0xE9 => {
                self.jump(merge_two_u8s_into_u16(self.registers.h, self.registers.l));
                (0, 1)
            },

            // Bitwise operations
            0xA8 => self.xor_r(OneByteRegister::B),
            0xA9 => self.xor_r(OneByteRegister::C),
            0xAA => self.xor_r(OneByteRegister::D),
            0xAB => self.xor_r(OneByteRegister::E),
            0xAC => self.xor_r(OneByteRegister::H),
            0xAD => self.xor_r(OneByteRegister::L),
            0xAF => self.xor_r(OneByteRegister::A),

            _ => panic!("Opcode {:x} not implemented yet", opcode),
        }
    }
}
