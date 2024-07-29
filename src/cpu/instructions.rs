//! Naming conventions:
//! - R means register
//! - I means immediate data
//! - RR means two byte register
//! - II means two byte immediate data
//! - RAM means an address specified by a two byte register

/// The number of bytes to skip after interpreting the instruction, if the instruction is
/// 2 bytes long we will need to skip 2 bytes
pub type Bytes = u8;

/// The amounts of cycles and instruction takes
pub type Cycles = u8;

use crate::{
    bus::Bus,
    common::{merge_two_u8s_into_u16, split_u16_into_two_u8s},
    flags::Flags,
    registers::{Register, Registers},
};

pub(crate) fn increment_rr<R>(register: &mut R) -> (Bytes, Cycles)
where
    R: Register<u16>,
{
    register.set(register.get().wrapping_add(1));
    (1, 2)
}

pub(crate) fn decrement_rr<R>(register: &mut R) -> (Bytes, Cycles)
where
    R: Register<u16>,
{
    register.set(register.get().wrapping_sub(1));
    (1, 2)
}

pub(crate) fn add_rr_to_rr<R, R2>(
    register: &mut R,
    to_add: R2,
    flags: &mut Flags,
) -> (Bytes, Cycles)
where
    R: Register<u16>,
    R2: Register<u16>,
{
    let (result, has_overflown) = register.get().overflowing_add(to_add.get());
    let last_twelve_bits_register = register.get() & 0x0FFF;
    let last_twelve_bits_to_add = to_add.get() & 0x0FFF;

    match last_twelve_bits_register.checked_add(last_twelve_bits_to_add) {
        Some(_) => flags.half_carry = false,
        None => flags.half_carry = true,
    }

    flags.substraction = false;
    flags.carry = has_overflown;

    register.set(result);

    (1, 2)
}

pub(crate) fn increment_r<R>(register: &mut R, flags: &mut Flags) -> (Bytes, Cycles)
where
    R: Register<u8>,
{
    let result = register.get().wrapping_add(1);

    update_half_carry_8bit(register, Operation::Addition(1), flags);
    flags.zero = result == 0;
    flags.substraction = false;

    register.set(result);

    (1, 1)
}

pub(crate) fn decrement_r<R>(register: &mut R, flags: &mut Flags) -> (Bytes, Cycles)
where
    R: Register<u8>,
{
    let result = register.get().wrapping_sub(1);

    update_half_carry_8bit(register, Operation::Subtraction(1), flags);
    flags.zero = result == 0;
    flags.substraction = false;

    register.set(result);

    (1, 1)
}

pub(crate) fn call(address: u16, registers: &mut Registers, bus: &mut Bus) -> (Bytes, Cycles) {
    let (p, c) = split_u16_into_two_u8s(registers.pc);

    registers.sp = registers.sp.wrapping_sub(1);
    bus[registers.sp] = p;
    registers.sp = registers.sp.wrapping_sub(1);
    bus[registers.sp] = c;

    registers.pc = address;

    (0, 6)
}

pub(crate) fn relative_jump(amount: u8, registers: &mut Registers) -> (Bytes, Cycles) {
    let signed_amount = amount as i8;

    if signed_amount >= 0 {
        registers.pc = registers.pc.wrapping_add(signed_amount as u16);
    } else {
        registers.pc = registers
            .pc
            .wrapping_sub(signed_amount.unsigned_abs() as u16);
    }

    (0, 3)
}

pub(crate) fn return_(registers: &mut Registers, bus: &mut Bus) -> (Bytes, Cycles) {
    let c = bus[registers.sp];
    registers.sp = registers.sp.wrapping_add(1);
    let p = bus[registers.sp];
    registers.sp = registers.sp.wrapping_add(1);

    registers.pc = merge_two_u8s_into_u16(p, c);
    (0, 4)
}

// Utilities
enum Operation {
    Addition(u8),
    Subtraction(u8),
}

fn update_half_carry_8bit<R>(register: &R, amount: Operation, flags: &mut Flags)
where
    R: Register<u8>,
{
    let last_four_bits = register.get() & 0x0F;
    let result = match amount {
        Operation::Addition(number) => last_four_bits.checked_add(number),
        Operation::Subtraction(number) => last_four_bits.checked_sub(number),
    };

    match result {
        Some(_) => flags.half_carry = false,
        None => flags.half_carry = true,
    }
}
