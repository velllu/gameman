use crate::common::{merge_two_u8s_into_u16, split_u16_into_two_u8s, BitwiseOperation, Operator};
use crate::consts::bus::IO_START;
use crate::registers::OneByteRegister;
use crate::GameBoy;

use super::{Bytes, Cycles};

// INC/DEC function
impl GameBoy {
    fn increment_r(&mut self, register: OneByteRegister, operator: Operator, amount: u8) {
        let register = self.registers.get_r(register);

        *register = match operator {
            Operator::Inc => register.wrapping_add(amount),
            Operator::Sub => register.wrapping_sub(amount),
        };

        let result = *register;
        self.flags.update_zero_flag(result);
    }
}

impl GameBoy {
    fn add_r_to_ra(&mut self, register: OneByteRegister) {
        let register = self.registers.get_r(register);
        *register = register.wrapping_add(*register);

        self.flags.update_zero_flag(*register);
    }
}

// LD functions
impl GameBoy {
    fn load_r_into_r(&mut self, register_to_be_loaded: OneByteRegister, register: OneByteRegister) {
        let register_to_be_loaded = *self.registers.get_r(register_to_be_loaded);
        let register = self.registers.get_r(register);

        *register = register_to_be_loaded;
    }

    fn load_i_into_r(&mut self, register: OneByteRegister) {
        let i = self.next(1);

        let register = self.registers.get_r(register);
        *register = i;
    }

    fn load_ra_into_io(&mut self, address_offset: u8) {
        self.bus[(IO_START + address_offset as usize) as u16] = self.registers.a;
    }

    fn load_io_into_r(&mut self, register: OneByteRegister) {
        let i = self.next(1);

        let register = self.registers.get_r(register);
        *register = self.bus[(IO_START + i as usize) as u16]
    }

    fn load_ram_into_r(&mut self, address: u16, register: OneByteRegister) {
        let register = self.registers.get_r(register);
        *register = self.bus[address];
    }
}

// CP functions
impl GameBoy {
    fn compare_ra_to_i(&mut self) {
        self.flags
            .update_zero_flag(self.registers.a.wrapping_sub(self.next(1)));
    }

    fn compare_ra_to_r(&mut self, register: OneByteRegister) {
        let register = *self.registers.get_r(register);
        self.flags
            .update_zero_flag(self.registers.a.wrapping_sub(register));
    }

    fn compare_ra_to_ram(&mut self, address: u16) {
        let ram = self.bus[address];
        self.flags
            .update_zero_flag(self.registers.a.wrapping_sub(ram));
    }
}

// Jump Functions
impl GameBoy {
    fn jump(&mut self, address: u16) {
        self.registers.pc = address;
    }

    fn jump_relative(&mut self) {
        let jump_amount = self.next(1) as i8;

        if jump_amount >= 0 {
            self.registers.pc = self.registers.pc.wrapping_add(jump_amount as u16);
        } else {
            self.registers.pc = self
                .registers
                .pc
                .wrapping_sub(jump_amount.unsigned_abs() as u16);
        }
    }
}

// Call functions
impl GameBoy {
    pub(crate) fn call(&mut self, location: u16) {
        // We need to add 3 to the PC because the `call` instruction uses the PC of the
        // next instruction, a `call` instruction is 3 bytes long so we need to skip
        // 3 bytes. Please do not ask how much this took me to debug.
        let (p, c) = split_u16_into_two_u8s(self.registers.pc + 3);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus[self.registers.sp] = c;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus[self.registers.sp] = p;

        self.registers.pc = location;
    }
}

impl GameBoy {
    fn return_(&mut self) {
        let first_byte = self.bus[self.registers.sp];
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let second_byte = self.bus[self.registers.sp];
        self.registers.sp = self.registers.sp.wrapping_add(1);

        self.registers.pc = merge_two_u8s_into_u16(first_byte, second_byte);
    }
}

// Bitwise operation functions
impl GameBoy {
    fn bitwise_operation_r(&mut self, register: OneByteRegister, operation: BitwiseOperation) {
        let register = *self.registers.get_r(register);

        let result = match operation {
            BitwiseOperation::Or => register | self.registers.a,
            BitwiseOperation::Xor => register ^ self.registers.a,
        };

        self.registers.a = result;
        self.flags.update_zero_flag(result);
    }
}

impl GameBoy {
    #[rustfmt::skip]
    pub(crate) fn interpret_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        use BitwiseOperation as Bitwise;

        // I did not choose macros because
        // - Each of them will expand, and make the binary "bigger" (by not a lot, but I
        // still find this less elegant)
        // - They are harder to debug, and harder to read for rust beginners

        match opcode {
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

            // Increment RR
            0x03 => { self.registers.increment_bc(1, Operator::Inc); (1, 2) },
            0x13 => { self.registers.increment_de(1, Operator::Inc); (1, 2) },
            0x23 => { self.registers.increment_hl(1, Operator::Inc); (1, 2) },
            0x33 => { self.registers.sp = self.registers.sp.wrapping_add(1); (1, 2) },

            // Decrement RR
            0x0B => { self.registers.increment_bc(1, Operator::Sub); (1, 2) }
            0x1B => { self.registers.increment_de(1, Operator::Sub); (1, 2) }
            0x2B => { self.registers.increment_hl(1, Operator::Sub); (1, 2) }
            0x3B => { self.registers.sp = self.registers.sp.wrapping_sub(1); (1, 2) },

            // Add R to Ra
            0x80 => { self.add_r_to_ra(OneByteRegister::B); (1, 1) },
            0x81 => { self.add_r_to_ra(OneByteRegister::C); (1, 1) },
            0x82 => { self.add_r_to_ra(OneByteRegister::D); (1, 1) },
            0x83 => { self.add_r_to_ra(OneByteRegister::E); (1, 1) },
            0x84 => { self.add_r_to_ra(OneByteRegister::H); (1, 1) },
            0x85 => { self.add_r_to_ra(OneByteRegister::L); (1, 1) },
            0x87 => { self.add_r_to_ra(OneByteRegister::A); (1, 1) },

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
            0x02 => { self.bus[self.registers.get_bc()] = self.registers.a; (1, 2) },
            0x12 => { self.bus[self.registers.get_de()] = self.registers.a; (1, 2) },
            0x36 => { self.bus[self.registers.get_hl()] = self.next(1); (2, 3) },
            0x70 => { self.bus[self.registers.get_hl()] = self.registers.b; (1, 2) },
            0x71 => { self.bus[self.registers.get_hl()] = self.registers.c; (1, 2) },
            0x72 => { self.bus[self.registers.get_hl()] = self.registers.d; (1, 2) },
            0x73 => { self.bus[self.registers.get_hl()] = self.registers.e; (1, 2) },
            0x74 => { self.bus[self.registers.get_hl()] = self.registers.h; (1, 2) },
            0x75 => { self.bus[self.registers.get_hl()] = self.registers.l; (1, 2) },
            0x77 => { self.bus[self.registers.get_hl()] = self.registers.a; (1, 2) },

            0x22 => {
                self.bus[self.registers.get_hl()] = self.registers.a;
                self.registers.increment_hl(1, Operator::Inc);
                (1, 2)
            },

            0x32 => {
                self.bus[self.registers.get_hl()] = self.registers.a;
                self.registers.increment_hl(1, Operator::Sub);
                (1, 2)
            },

            // Load R into IO
            0xE0 => { self.load_ra_into_io(self.next(1)); (2, 3) },
            0xE2 => { self.load_ra_into_io(self.registers.c); (2, 3) },

            // Load IO into R (only one of this)
            0xF0 => { self.load_io_into_r(OneByteRegister::A); (2, 3) },

            // Load R into RAM, with address specified by II
            0xEA => { let ii = self.next_two(); self.bus[ii] = self.registers.a; (3, 4) },
            0x08 => {
                // This is a bit different from `0xEA`, because we need to load SP in,
                // which is 2 bytes long
                let (s, p) = split_u16_into_two_u8s(self.registers.sp);

                let ii = self.next_two();
                self.bus[ii] = s;
                self.bus[ii.wrapping_add(1)] = p;

                (3, 5)
            }

            // Load RAM into R
            0x0A => { self.load_ram_into_r(self.registers.get_bc(), OneByteRegister::A); (1, 2) },
            0x1A => { self.load_ram_into_r(self.registers.get_de(), OneByteRegister::A); (1, 2) },
            0x46 => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::B); (1, 2) },
            0x4E => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::C); (1, 2) },
            0x56 => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::D); (1, 2) },
            0x5E => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::E); (1, 2) },
            0x66 => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::H); (1, 2) },
            0x6E => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::L); (1, 2) },
            0x7E => { self.load_ram_into_r(self.registers.get_hl(), OneByteRegister::A); (1, 2) },
            0x2A => {
                self.registers.a = self.bus[self.registers.get_hl()];
                self.registers.increment_hl(1, Operator::Inc);
                (1, 2)
            },
            0x3A => {
                self.registers.a = self.bus[self.registers.get_hl()];
                self.registers.increment_hl(1, Operator::Sub);
                (1, 2)
            },

            // Compare
            0xB8 => { self.compare_ra_to_r(OneByteRegister::B); (1, 1) },
            0xB9 => { self.compare_ra_to_r(OneByteRegister::C); (1, 1) },
            0xBA => { self.compare_ra_to_r(OneByteRegister::D); (1, 1) },
            0xBB => { self.compare_ra_to_r(OneByteRegister::E); (1, 1) },
            0xBC => { self.compare_ra_to_r(OneByteRegister::H); (1, 1) },
            0xBD => { self.compare_ra_to_r(OneByteRegister::L); (1, 1) },
            0xBE => { self.compare_ra_to_ram(self.registers.get_hl()); (1, 1) },
            0xBF => { self.compare_ra_to_r(OneByteRegister::A); (1, 1) },
            0xFE => { self.compare_ra_to_i(); (2, 2) },

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

            // Relative Jumps
            0x18 => { self.jump_relative(); (2, 3) },

            0x20 =>
                if !self.flags.zero { self.jump_relative(); (2, 3) }
                else { (2, 2) }

            0x28 =>
                if self.flags.zero { self.jump_relative(); (2, 3) }
                else { (2, 2) }

            0x30 =>
                if !self.flags.carry { self.jump_relative(); (2, 3) }
                else { (2, 2) }

            0x38 =>
                if self.flags.carry { self.jump_relative(); (2, 3) }
                else { (2, 2) }

            // Calls
            0xCD => { self.call(self.next_two()); (0, 6) },

            0xC4 =>
                if !self.flags.zero { self.call(self.next_two()); (0, 6) }
                else { (3, 3) }

            0xCC =>
                if self.flags.zero { self.call(self.next_two()); (0, 6) }
                else { (3, 3) }

            0xD4 =>
                if !self.flags.carry { self.call(self.next_two()); (0, 6) }
                else { (3, 3) }

            0xDC =>
                if self.flags.carry { self.call(self.next_two()); (0, 6) }
                else { (3, 3) }

            // RET
            0xC9 => { self.return_(); (0, 4) },

            0xC0 =>
                if !self.flags.zero { self.return_(); (0, 5) }
                else { (1, 2) }

            0xC8 =>
                if self.flags.zero { self.return_(); (0, 5) }
                else { (1, 2) }

            0xD0 =>
                if !self.flags.carry { self.return_(); (0, 5) }
                else { (1, 2) }

            0xD8 =>
                if self.flags.carry { self.return_(); (0, 5) }
                else { (1, 2) }

            // XOR
            0xA8 => { self.bitwise_operation_r(OneByteRegister::B, Bitwise::Xor); (1, 1) },
            0xA9 => { self.bitwise_operation_r(OneByteRegister::C, Bitwise::Xor); (1, 1) },
            0xAA => { self.bitwise_operation_r(OneByteRegister::D, Bitwise::Xor); (1, 1) },
            0xAB => { self.bitwise_operation_r(OneByteRegister::E, Bitwise::Xor); (1, 1) },
            0xAC => { self.bitwise_operation_r(OneByteRegister::H, Bitwise::Xor); (1, 1) },
            0xAD => { self.bitwise_operation_r(OneByteRegister::L, Bitwise::Xor); (1, 1) },
            0xAF => { self.bitwise_operation_r(OneByteRegister::A, Bitwise::Xor); (1, 1) },

            // OR
            0xB0 => { self.bitwise_operation_r(OneByteRegister::B, Bitwise::Or); (1, 1) },
            0xB1 => { self.bitwise_operation_r(OneByteRegister::C, Bitwise::Or); (1, 1) },
            0xB2 => { self.bitwise_operation_r(OneByteRegister::D, Bitwise::Or); (1, 1) },
            0xB3 => { self.bitwise_operation_r(OneByteRegister::E, Bitwise::Or); (1, 1) },
            0xB4 => { self.bitwise_operation_r(OneByteRegister::H, Bitwise::Or); (1, 1) },
            0xB5 => { self.bitwise_operation_r(OneByteRegister::L, Bitwise::Or); (1, 1) },
            0xB7 => { self.bitwise_operation_r(OneByteRegister::A, Bitwise::Or); (1, 1) },

            // Misc
            0x00 => (1, 1), // NOP, does nothing
            0x76 => { (0, 1) }, // TODO: Actually implement this properly
            0xCB => {
                self.registers.pc = self.registers.pc.wrapping_add(1);
                self.interpret_cb_opcode(self.next(0))
            },

            // Interrupt stuff
            0xF3 => { self.flags.ime = false; (1, 1) },
            0xFB => { self.flags.ime = true; (1, 1) },

            _ => panic!("Opcode {:x} not implemented yet", opcode),
        }
    }
}
