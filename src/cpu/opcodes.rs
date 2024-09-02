#![warn(clippy::match_same_arms)]

use crate::{
    bus::Bus,
    common::{merge_two_u8s_into_u16, split_u16_into_two_u8s},
    flags::Flags,
    registers::Registers,
};

use super::{Bytes, Cpu, Cycles};

impl Cpu {
    pub fn interpret_opcode(
        &mut self,
        opcode: u8,
        flags: &mut Flags,
        regs: &mut Registers,
        bus: &mut Bus,
    ) -> (Bytes, Cycles) {
        if self.is_cb {
            self.is_cb = false;
            return self.interpret_cb_opcode(opcode, flags, regs, bus);
        }

        // TODO: Fix timing on instruction with register `6`, they should have a clock more
        match opcode {
            0x00 => (0, 0),

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
                (regs.h, regs.l) = split_u16_into_two_u8s(register_hl.wrapping_add(register_r));

                (1, 2)
            }

            // Instruction `LD register A, (rr+/-)` - 00rr0010
            // Copy register r (with increments) to register A
            0x0A | 0x1A | 0x2A | 0x3A => {
                let address = regs.get_register_couple_with_increments(opcode >> 4);
                regs.a = bus[address];

                (1, 2)
            }

            // Instruction `JR immediate data` - 00011000
            // Convert immediate data to signed 8 bit number and add it to the pc
            0x18 => {
                let jump_amount = bus.next_one(regs) as i8;
                add_i8_to_u16(&mut regs.pc, jump_amount);

                (2, 2)
            }

            // Instruction `JR condition, immediate data` - 001cc000
            // Convert immediate data to signed 8 bit number and add it to the pc if
            // condition applies
            0x20 | 0x28 | 0x30 | 0x38 => {
                if flags.is_condition_valid(opcode >> 3) {
                    let jump_amount = bus.next_one(regs) as i8;
                    add_i8_to_u16(&mut regs.pc, jump_amount);

                    (2, 3)
                } else {
                    (2, 2)
                }
            }

            // Instruction Halt
            // This is in the middle of the ld instructions, TODO: implement this fully
            0x76 => (0, 1),

            // Instruction `LD x, y` - 01xxxyyy
            // Load the value of register y into register x
            0x40..=0x7F => {
                let register_y = regs.get_register(opcode, bus);
                regs.set_register(opcode >> 3, register_y, bus);

                (1, 1)
            }

            // Instruction `ADD r` - 10000rrr
            // Adds register r to register A
            0x80..=0x87 => {
                let (result, has_overflown) =
                    regs.a.overflowing_add(regs.get_register(opcode, bus));

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            // Instruction `ADDC r` - 10001rrr
            // Adds register r to register A and add carry flag
            0x88..=0x8F => {
                let (result, has_overflown) = regs.a.overflowing_add(
                    regs.get_register(opcode, bus)
                        .wrapping_add(flags.carry as u8),
                );

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            // Instruction `SUB r` - 10000rrr
            // Subtracts register A from register r
            0x90..=0x97 => {
                let (result, has_overflown) =
                    regs.a.overflowing_sub(regs.get_register(opcode, bus));

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            // Instruction `SUBC r` - 10001rrr
            // Subtracts register A and the carry flag from register r
            0x98..=0x9F => {
                let (result, has_overflown) = regs.a.overflowing_sub(
                    regs.get_register(opcode, bus)
                        .wrapping_add(flags.carry as u8),
                );

                regs.a = result;
                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            // Instruction `AND r` - 10100rrr
            // Store the result of a logical and between register A and register R into
            // register A and set carry flag to 0
            0xA0..=0xA7 => {
                regs.a = regs.a & regs.get_register(opcode, bus);
                flags.zero = regs.a == 0;
                flags.carry = false;

                (1, 1)
            }

            // Instruction `XOR r` - 10101rrr
            // Store the result of a logical xor between register A and register R into
            // register A and set carry flag to 0
            0xA8..=0xAF => {
                regs.a = regs.a ^ regs.get_register(opcode, bus);
                flags.zero = regs.a == 0;
                flags.carry = false;

                (1, 1)
            }

            // Instruction `XOR r` - 10110rrr
            // Store the result of a logical or between register A and register R into
            // register A and set carry flag to 0
            0xB0..=0xB7 => {
                regs.a = regs.a | regs.get_register(opcode, bus);
                flags.zero = regs.a == 0;
                flags.carry = false;

                (1, 1)
            }

            // Instruction `CP r` - 10111rrr
            // This is like the `SUB r` instruction except the register a isn't actually
            // changed
            0xB8..=0xBF => {
                let (result, has_overflown) =
                    regs.a.overflowing_sub(regs.get_register(opcode, bus));

                flags.zero = result == 0;
                flags.carry = has_overflown;

                (1, 1)
            }

            _ => panic!("Unimplemented opcode: {:x}", opcode),
        }
    }
}

fn add_i8_to_u16(u16: &mut u16, i8: i8) {
    *u16 = match i8 >= 0 {
        true => u16.wrapping_add(i8 as u16),
        false => u16.wrapping_sub(i8.unsigned_abs() as u16),
    };
}
