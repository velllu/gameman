use crate::common::{merge_two_u8s_into_u16, Operator};
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

// INC/DEC function
impl GameBoy {
    pub(crate) fn increment_r(
        &mut self,
        register: OneByteRegister,
        operator: Operator,
        amount: u8,
    ) {
        let register = self.registers.get_r(register);

        *register = match operator {
            Operator::Inc => register.wrapping_add(amount),
            Operator::Sub => register.wrapping_sub(amount),
        };

        let result = *register;
        self.flags.update_zero_flag(result);
    }
}

// LD functions
impl GameBoy {
    pub(crate) fn load_r_into_r(
        &mut self,
        register_to_be_loaded: OneByteRegister,
        register: OneByteRegister,
    ) {
        let register_to_be_loaded = *self.registers.get_r(register_to_be_loaded);
        let register = self.registers.get_r(register);

        *register = register_to_be_loaded;
    }

    pub(crate) fn load_i_into_r(&mut self, register: OneByteRegister) {
        let i = self.next(1);

        let register = self.registers.get_r(register);
        *register = i;
    }
}

// Bitwise operation functions (not all of them as of now)
impl GameBoy {
    pub(crate) fn xor_r(&mut self, register: OneByteRegister) {
        let register_a = *self.registers.get_r(OneByteRegister::A);
        let register = self.registers.get_r(register);

        let result = *register ^ register_a;
        *register = result;

        self.flags.update_zero_flag(result);
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
            0x04 => { self.increment_r(OneByteRegister::B, Operator::Inc, 1); (1, 1) },
            0x0C => { self.increment_r(OneByteRegister::C, Operator::Inc, 1); (1, 1) },
            0x14 => { self.increment_r(OneByteRegister::D, Operator::Inc, 1); (1, 1) },
            0x1C => { self.increment_r(OneByteRegister::E, Operator::Inc, 1); (1, 1) },
            0x24 => { self.increment_r(OneByteRegister::H, Operator::Inc, 1); (1, 1) },
            0x2C => { self.increment_r(OneByteRegister::L, Operator::Inc, 1); (1, 1) },
            0x3C => { self.increment_r(OneByteRegister::A, Operator::Inc, 1); (1, 1) },

            // Decrement R
            0x05 => { self.increment_r(OneByteRegister::B, Operator::Sub, 1); (1, 1) },
            0x0D => { self.increment_r(OneByteRegister::C, Operator::Sub, 1); (1, 1) },
            0x15 => { self.increment_r(OneByteRegister::D, Operator::Sub, 1); (1, 1) },
            0x1D => { self.increment_r(OneByteRegister::E, Operator::Sub, 1); (1, 1) },
            0x25 => { self.increment_r(OneByteRegister::H, Operator::Sub, 1); (1, 1) },
            0x2D => { self.increment_r(OneByteRegister::L, Operator::Sub, 1); (1, 1) },
            0x3D => { self.increment_r(OneByteRegister::A, Operator::Sub, 1); (1, 1) },

            // Load R into R
            0x40 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::B); (1, 1) },
            0x41 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::B); (1, 1) },
            0x42 => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::B); (1, 1) },
            0x43 => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::B); (1, 1) },
            0x44 => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::B); (1, 1) },
            0x45 => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::B); (1, 1) },
            0x47 => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::B); (1, 1) },
            0x48 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::C); (1, 1) },
            0x49 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::C); (1, 1) },
            0x4A => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::C); (1, 1) },
            0x4B => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::C); (1, 1) },
            0x4C => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::C); (1, 1) },
            0x4D => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::C); (1, 1) },
            0x4F => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::C); (1, 1) },
            0x50 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::D); (1, 1) },
            0x51 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::D); (1, 1) },
            0x52 => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::D); (1, 1) },
            0x53 => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::D); (1, 1) },
            0x54 => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::D); (1, 1) },
            0x55 => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::D); (1, 1) },
            0x57 => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::D); (1, 1) },
            0x58 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::E); (1, 1) },
            0x59 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::E); (1, 1) },
            0x5A => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::E); (1, 1) },
            0x5B => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::E); (1, 1) },
            0x5C => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::E); (1, 1) },
            0x5D => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::E); (1, 1) },
            0x5F => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::E); (1, 1) },
            0x60 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::H); (1, 1) },
            0x61 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::H); (1, 1) },
            0x62 => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::H); (1, 1) },
            0x63 => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::H); (1, 1) },
            0x64 => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::H); (1, 1) },
            0x65 => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::H); (1, 1) },
            0x67 => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::H); (1, 1) },
            0x68 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::L); (1, 1) },
            0x69 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::L); (1, 1) },
            0x6A => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::L); (1, 1) },
            0x6B => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::L); (1, 1) },
            0x6C => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::L); (1, 1) },
            0x6D => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::L); (1, 1) },
            0x6F => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::L); (1, 1) },
            0x78 => { self.load_r_into_r(OneByteRegister::B, OneByteRegister::A); (1, 1) },
            0x79 => { self.load_r_into_r(OneByteRegister::C, OneByteRegister::A); (1, 1) },
            0x7A => { self.load_r_into_r(OneByteRegister::D, OneByteRegister::A); (1, 1) },
            0x7B => { self.load_r_into_r(OneByteRegister::E, OneByteRegister::A); (1, 1) },
            0x7C => { self.load_r_into_r(OneByteRegister::H, OneByteRegister::A); (1, 1) },
            0x7D => { self.load_r_into_r(OneByteRegister::L, OneByteRegister::A); (1, 1) },
            0x7F => { self.load_r_into_r(OneByteRegister::A, OneByteRegister::A); (1, 1) },

            // Load II into RR
            0x01 => { self.registers.set_bc(self.next_two()); (3, 3) },
            0x11 => { self.registers.set_de(self.next_two()); (3, 3) },
            0x21 => { self.registers.set_hl(self.next_two()); (3, 3) },
            0x31 => { self.registers.sp = self.next_two();    (3, 3) },

            // Load I into R
            0x06 => { self.load_i_into_r(OneByteRegister::B); (2, 2) },
            0x0E => { self.load_i_into_r(OneByteRegister::C); (2, 2) },
            0x16 => { self.load_i_into_r(OneByteRegister::D); (2, 2) },
            0x1E => { self.load_i_into_r(OneByteRegister::E); (2, 2) },
            0x26 => { self.load_i_into_r(OneByteRegister::H); (2, 2) },
            0x2E => { self.load_i_into_r(OneByteRegister::L); (2, 2) },
            0x3E => { self.load_i_into_r(OneByteRegister::A); (2, 2) },

            // Load R into ram
            0x02 => { self.bus.write_byte(self.registers.get_bc(), self.registers.a); (1, 2) },
            0x08 => todo!(), // this is a strange one,
            0x12 => { self.bus.write_byte(self.registers.get_de(), self.registers.a); (1, 2) },
            0x22 => { self.bus.write_byte(self.registers.get_hl(), self.registers.a.wrapping_add(1)); (1, 2) },
            0x32 => { self.bus.write_byte(self.registers.get_hl(), self.registers.a.wrapping_sub(1)); (1, 2) },
            0x36 => { self.bus.write_byte(self.registers.get_hl(), self.next(1)); (2, 3) },
            0x70 => { self.bus.write_byte(self.registers.get_hl(), self.registers.b); (1, 2) },
            0x71 => { self.bus.write_byte(self.registers.get_hl(), self.registers.c); (1, 2) },
            0x72 => { self.bus.write_byte(self.registers.get_hl(), self.registers.d); (1, 2) },
            0x73 => { self.bus.write_byte(self.registers.get_hl(), self.registers.e); (1, 2) },
            0x74 => { self.bus.write_byte(self.registers.get_hl(), self.registers.h); (1, 2) },
            0x75 => { self.bus.write_byte(self.registers.get_hl(), self.registers.l); (1, 2) },
            0x77 => { self.bus.write_byte(self.registers.get_hl(), self.registers.a); (1, 2) },

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
            0xA8 => { self.xor_r(OneByteRegister::B); (1, 1) },
            0xA9 => { self.xor_r(OneByteRegister::C); (1, 1) },
            0xAA => { self.xor_r(OneByteRegister::D); (1, 1) },
            0xAB => { self.xor_r(OneByteRegister::E); (1, 1) },
            0xAC => { self.xor_r(OneByteRegister::H); (1, 1) },
            0xAD => { self.xor_r(OneByteRegister::L); (1, 1) },
            0xAF => { self.xor_r(OneByteRegister::A); (1, 1) },

            _ => panic!("Opcode {:x} not implemented yet", opcode),
        }
    }
}
