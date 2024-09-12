#![warn(clippy::match_same_arms)]

use crate::{
    bus::Bus,
    common::{merge_two_u8s_into_u16, split_u16_into_two_u8s},
    flags::Flags,
    registers::Registers,
};

use super::{Bytes, Cpu, Cycles};

pub(crate) const CALL: u8 = 0xCD;
pub(crate) const JUMP: u8 = 0xC3;
pub(crate) const RELATIVE_JUMP: u8 = 0x18;
pub(crate) const RET: u8 = 0xC9;

impl Cpu {
    pub fn interpret_opcode(
        &mut self,
        opcode: u8,
        flags: &mut Flags,
        regs: &mut Registers,
        bus: &mut Bus,
    ) -> (Bytes, Cycles) {
        if self.halt {
            self.halt = false;
            return (0, 1);
        }

        // TODO: Fix timing on instruction with register `6`, they should have a clock more
        match opcode {
            // Instruction `NOP`
            0x00 => (1, 0),

            // Instruction `LD rr, immediate data` - 00rr0001
            // Loads immediate data into given register couple
            0x01 | 0x11 | 0x21 | 0x31 => {
                regs.set_register_couple(opcode >> 4, bus.next_two(regs));
                (3, 3)
            }

            // Instruction `LD (rr+/-), register A` - 00rr0010
            // Copy register A to address specified by register r (with increments)
            0x02 | 0x12 | 0x22 | 0x32 => {
                let address = regs.get_register_couple_with_increments(opcode >> 4);
                bus[address] = regs.a;

                (1, 2)
            }

            // Instruction `INC rr` - 00rr0011
            // Increments by one given register couple
            0x03 | 0x13 | 0x23 | 0x33 => {
                let new_value = regs.get_register_couple(opcode >> 4).wrapping_add(1);
                regs.set_register_couple(opcode >> 4, new_value);
                (1, 2)
            }

            // Instruction `DEC rr` - 00rr0011
            // Decrements by one given register couple
            0x0B | 0x1B | 0x2B | 0x3B => {
                let new_value = regs.get_register_couple(opcode >> 4).wrapping_sub(1);
                regs.set_register_couple(opcode >> 4, new_value);
                (1, 2)
            }

            // Instruction `INC r` - 00rrr100
            // Increments by one given register and update flags
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                let new_value = regs.get_register(opcode >> 3, bus).wrapping_add(1);
                regs.set_register(opcode >> 3, new_value, bus);
                flags.zero = new_value == 0;

                (1, 1)
            }

            // Instruction `DEC r` - 00rrr101
            // Decrements by one given register and update flags
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                let new_value = regs.get_register(opcode >> 3, bus).wrapping_sub(1);
                regs.set_register(opcode >> 3, new_value, bus);
                flags.zero = new_value == 0;

                (1, 1)
            }

            // Instruction `LD r, immediate data` - 00rrr110
            // Loads immediate data into register
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                regs.set_register(opcode >> 3, bus.next_one(regs), bus);

                (2, 2)
            }

            // Instruction `RLCA r` - 00000111
            // Exactly like the CB instruction `RLC a` which is the same opcode but resets
            // zero flag
            0x07 => {
                self.interpret_cb_opcode(opcode, flags, regs, bus);
                flags.zero = false;
                (1, 1)
            }

            // Instruction `LD (immediate data), SP` - 00001000
            // Load the lower part of SP into immediate data address and the higher part
            // in the next cell
            0x08 => {
                let (s, p) = split_u16_into_two_u8s(regs.sp);
                let address = bus.next_two(regs);

                bus[address] = p;
                bus[address.wrapping_add(1)] = s;

                (3, 5)
            }

            // Instruction `ADD HL, rr` - 00rr1001
            // Add given register to register HL. TODO: Handle carry flag
            0x09 | 0x19 | 0x29 | 0x39 => {
                let register_r = regs.get_register_couple(opcode >> 4);
                let register_hl = merge_two_u8s_into_u16(regs.h, regs.l);
                let (result, has_oveflown) = register_hl.overflowing_add(register_r);

                flags.carry = has_oveflown;
                (regs.h, regs.l) = split_u16_into_two_u8s(result);

                (1, 2)
            }

            // Instruction `LD register A, (rr+/-)` - 00rr0010
            // Copy register r (with increments) to register A
            0x0A | 0x1A | 0x2A | 0x3A => {
                let address = regs.get_register_couple_with_increments(opcode >> 4);
                regs.a = bus[address];

                (1, 2)
            }

            // Instruction `RRCA` - 00001111
            // Exactly like the CB instruction `RRC a` which is the same opcode but resets
            // zero flag
            0x0F => {
                self.interpret_cb_opcode(opcode, flags, regs, bus);
                flags.zero = false;
                (1, 1)
            }

            // Instruction `RLA` - 00010111
            // Exactly like the CB instruction `RL a` which is the same opcode but resets
            // zero flag
            0x17 => {
                self.interpret_cb_opcode(opcode, flags, regs, bus);
                flags.zero = false;
                (1, 1)
            }

            // Instruction `JR immediate data` - 00011000
            // Convert immediate data to signed 8 bit number and add it to the pc
            0x18 => {
                let jump_amount = bus.next_one(regs) as i8;
                regs.pc = add_i8_to_u16(regs.pc, jump_amount);

                (2, 2)
            }

            // Instruction `RRA` - 00011111
            // Exactly like the CB instruction `RR a` which is the same opcode but resets
            // zero flag
            0x1F => {
                self.interpret_cb_opcode(opcode, flags, regs, bus);
                flags.zero = false;
                (1, 1)
            }

            // Instruction `JR condition, immediate data` - 001cc000
            // Convert immediate data to signed 8 bit number and add it to the pc if
            // condition applies
            0x20 | 0x28 | 0x30 | 0x38 => {
                if flags.is_condition_valid(opcode >> 3) {
                    self.interpret_opcode(RELATIVE_JUMP, flags, regs, bus);
                    (2, 3)
                } else {
                    (2, 2)
                }
            }

            // TODO: Implement this
            0x27 => {
                regs.a = 0;

                (1, 1)
            }

            // Instruction `CPL` - 00101111
            // Flip the bits of register A
            0x2F => {
                regs.a = !regs.a;

                (1, 1)
            }

            // Instruction `SCF` - 00110111
            // Set carry flag
            0x37 => {
                flags.carry = true;

                (1, 1)
            }

            // Instruction `CCF` - 00111111
            // Flip carry flag
            0x3F => {
                flags.carry = !flags.carry;

                (1, 1)
            }

            // Instruction Halt
            // This is in the middle of the ld instructions, TODO: implement this fully
            0x76 => {
                self.halt = true;

                (1, 1)
            }

            // Instruction `LD x, y` - 01xxxyyy
            // Load the value of register y into register x
            0x40..=0x7F => {
                let register_y = regs.get_register(opcode, bus);
                regs.set_register(opcode >> 3, register_y, bus);

                (1, 1)
            }

            // Instructions `operation r` - 10ooorrr
            // Do operation "o" on register r and register A, store result in register A
            0x80..=0xB7 => {
                let register_r = regs.get_register(opcode, bus);
                let (result, carry) = do_operation(opcode >> 3, regs.a, register_r, flags);

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = carry;

                (1, 1)
            }

            // Instruction `CP r` - 10111rrr
            // Subtract register r from register A, update the flags, but dump the result
            0xB8..=0xBF => {
                let (result, has_overflown) =
                    regs.a.overflowing_sub(regs.get_register(opcode, bus));

                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            // Instruction `RET condition` - 110cc000
            // Like instruction ret but only if condition applies
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                if flags.is_condition_valid(opcode >> 3) {
                    self.interpret_opcode(RET, flags, regs, bus);
                    (0, 5)
                } else {
                    (1, 2)
                }
            }

            // Instruction `POP rrf` - 11rr0001
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let low = bus[regs.sp];
                regs.sp = regs.sp.wrapping_add(1);
                let high = bus[regs.sp];
                regs.sp = regs.sp.wrapping_add(1);

                let popped_value = merge_two_u8s_into_u16(high, low);
                regs.set_register_couple_with_flags(opcode >> 4, popped_value, flags);

                (1, 3)
            }

            // Instruction `JP condition, immediate data` - 110cc010
            // Sets pc to immediate data if given condition is valid
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                if flags.is_condition_valid(opcode >> 3) {
                    self.interpret_opcode(JUMP, flags, regs, bus);
                    (0, 6)
                } else {
                    (3, 3)
                }
            }

            // Instruction `JP immediate data` - 11000011
            // Sets pc to immediate data
            0xC3 => {
                let immediate_data = bus.next_two(regs);
                regs.pc = immediate_data;

                (0, 4)
            }

            // Instruction `CALL condition, immediate data`
            // Like the call instruction but only if the condition is valid
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                if flags.is_condition_valid(opcode >> 3) {
                    self.interpret_opcode(CALL, flags, regs, bus);
                    (0, 4)
                } else {
                    (3, 3)
                }
            }

            // Instruction `PUSH rrf` - 11rr0101
            // Subtract one to sp, store high part of the given register (with flags) at
            // address sp, subtract one to sp again, store low part of the given register
            // at address sp again
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                let (high, low) =
                    split_u16_into_two_u8s(regs.get_register_couple_with_flags(opcode >> 4, flags));

                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = high;
                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = low;

                (1, 4)
            }

            // Instruction `operation immediate_data` - 11ooo110
            // Do operation "o" on immediate data byte and register A, store result in
            // register A
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 => {
                let immediate_data = bus.next_one(regs);
                let (result, carry) = do_operation(opcode >> 3, regs.a, immediate_data, flags);

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = carry;

                (2, 2)
            }

            // Instruction `RET` - 11001001
            // Load address sp into lower part of pc, decrement sp, load address sp into
            // higher part of pc, and decrement sp again
            0xC9 => {
                let c = bus[regs.sp];
                regs.sp = regs.sp.wrapping_add(1);
                let p = bus[regs.sp];
                regs.sp = regs.sp.wrapping_add(1);
                regs.pc = merge_two_u8s_into_u16(p, c);

                (0, 4)
            }

            // Instruction `CB`
            0xCB => {
                let next_opcode = bus.next_one(regs);
                let (bytes, cycles) = self.interpret_cb_opcode(next_opcode, flags, regs, bus);

                (bytes + 1, cycles + 1)
            }

            // Instruction `CALL immediate data` - 11001101
            // Decrement sp by one, load high part of pc into address sp, decrement sp
            // again, load low part of pc into address sp, finally, load immediate data
            // into pc. Note: Increment the pc by 3 before pushing because it needs to be
            // the pc after the call instruction which lasts 3 bytes
            0xCD => {
                let (p, c) = split_u16_into_two_u8s(regs.pc.wrapping_add(3));
                let immediate_data = bus.next_two(regs);

                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = p;
                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = c;
                regs.pc = immediate_data;

                (0, 6)
            }

            // Instruction `RST n` - 11nnn111
            // Push the program counter, and load into pc, then set the program counter to
            // n multiplied by eight
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                // Pushing, NOTE: we add one because we push the pc of the next
                // instruction
                let (p, c) = split_u16_into_two_u8s(regs.pc.wrapping_add(1));
                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = p;
                regs.sp = regs.sp.wrapping_sub(1);
                bus[regs.sp] = c;

                // Jump table
                let n = (opcode >> 3) & 0b00000111;
                regs.pc = (n * 8) as u16;

                (0, 4)
            }

            // Instruction `RETI` - 11011001
            // Like the `RET` instruction but Interrupt Master Enable flag is set to true
            0xD9 => {
                self.interpret_opcode(RET, flags, regs, bus);
                self.ime = true;

                (0, 4)
            }

            // Instruction `LD io address, register A` - 11100000
            // Loads register a at address specified by immediate data with an offset of
            // `0xFF`
            0xE0 => {
                let address = 0xFF00 | (bus.next_one(regs) as u16);
                bus[address] = regs.a;

                (2, 3)
            }

            // Instruction `LD (register C), A` - 11100010
            // Loads register a at address specified by register C with an offset of `0xFF`
            0xE2 => {
                let address = 0xFF00 | (regs.c as u16);
                bus[address] = regs.a;

                (1, 2)
            }

            // Instruction `ADD SP, immediate data` - 11101000
            // Add immediate data to register SP
            0xE8 => {
                let immediate_data = bus.next_one(regs);
                let immediate_data_signed = immediate_data as i8;

                // In this instruction, the carry flag is set if the lower bits of sp and
                // the unsigned byte of immediate data overflows
                let p = (regs.sp & 0x00FF) as u8;
                flags.carry = p.checked_add(immediate_data).is_none();

                flags.zero = false;
                regs.sp = add_i8_to_u16(regs.sp, immediate_data_signed);

                (2, 4)
            }

            // Instruction `JP HL` - 11101001
            // Sets pc to register HL
            0xE9 => {
                regs.pc = merge_two_u8s_into_u16(regs.h, regs.l);

                (0, 1)
            }

            // Instruction `LD (immediate data), register A` - 11101010
            // Load register A at address specified by immediate data
            0xEA => {
                let immediate_data = bus.next_two(regs);
                bus[immediate_data] = regs.a;

                (3, 4)
            }

            // Instruction `LD register A, io address` - 11100000
            // Loads address specified by immediate data with an offset of `0xFF` into
            // register A
            0xF0 => {
                let address = 0xFF00 | (bus.next_one(regs) as u16);
                regs.a = bus[address];

                (2, 3)
            }

            // Instruction `LD register A, io address` - 11110010
            // Loads address specified by register C with an offset of `0xFF` into
            // register A
            0xF2 => {
                let address = 0xFF00 | (regs.c as u16);
                regs.a = bus[address];

                (1, 2)
            }

            // Disable Interrupt Master Enable flag
            0xF3 => {
                self.ime = false;
                (1, 1)
            }

            // Instruction `LD HL, SP + immediate data` - 11111000
            // Add SP to signed immediate data and copy the result to register HL
            0xF8 => {
                let immediate_data = bus.next_one(regs);
                let immediate_data_signed = immediate_data as i8;

                // In this instruction, the carry flag is set if the lower bits of sp and
                // the unsigned byte of immediate data overflows
                let p = (regs.sp & 0x00FF) as u8;
                flags.carry = p.checked_add(immediate_data).is_none();

                flags.zero = false;

                let new_value = add_i8_to_u16(regs.sp, immediate_data_signed);
                (regs.h, regs.l) = split_u16_into_two_u8s(new_value);

                (2, 3)
            }

            // Instruction `LD SP, HL` - 11111001
            // Copy register HL to SP
            0xF9 => {
                regs.sp = merge_two_u8s_into_u16(regs.h, regs.l);

                (1, 2)
            }

            // Instruction `LD register A, (immediate data)` - 11111010
            // Load specified address into register A
            0xFA => {
                let immediate_data = bus.next_two(regs);
                regs.a = bus[immediate_data];

                (3, 4)
            }

            // Enable Interrupt Master Enable flag
            0xFB => {
                self.ime = true;
                (1, 1)
            }

            // Instruction `CP immediate data` - 11111110
            // Subtract immediate data from register A, update the flags, but dump the
            // result
            0xFE => {
                let immediate_data = bus.next_one(regs);
                let (result, has_overflown) = regs.a.overflowing_sub(immediate_data);

                flags.zero = result == 0;
                flags.carry = has_overflown;

                (2, 2)
            }

            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}

fn add_i8_to_u16(u16: u16, i8: i8) -> u16 {
    match i8 >= 0 {
        true => u16.wrapping_add(i8 as u16),
        false => u16.wrapping_sub(i8.unsigned_abs() as u16),
    }
}

/// Cases:
/// - 0: num1 + num2
/// - 1: num1 + num2 + carry flag
/// - 2: num1 - num2
/// - 3: num1 - num2 - carry flag
/// - 4: num1 & num2
/// - 5: num1 ^ num2
/// - 6: num1 | num2
fn do_operation(operation_num: u8, num1: u8, num2: u8, flags: &Flags) -> (u8, bool) {
    match operation_num & 0b00000111 {
        0 => num1.overflowing_add(num2),
        1 => {
            let (result_c, has_overflown_c) = num2.overflowing_add(flags.carry as u8);
            let (result, has_overflown) = num1.overflowing_add(result_c);
            (result, has_overflown || has_overflown_c)
        }
        2 => num1.overflowing_sub(num2),
        3 => {
            let (result_c, has_overflown_c) = num2.overflowing_add(flags.carry as u8);
            let (result, has_overflown) = num1.overflowing_sub(result_c);
            (result, has_overflown || has_overflown_c)
        }
        4 => (num1 & num2, false),
        5 => (num1 ^ num2, false),
        6 => (num1 | num2, false),
        _ => unreachable!(),
    }
}
