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
    registers::{ReadRegister, ReadWriteRegister, Registers},
};

pub(crate) fn increment_rr<R>(register: R) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u16>,
{
    let incremented = register.get().wrapping_add(1);
    register.set(incremented);
    (1, 2)
}

pub(crate) fn decrement_rr<R>(register: R) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u16>,
{
    let decremented = register.get().wrapping_sub(1);
    register.set(decremented);
    (1, 2)
}

pub(crate) fn add_rr_to_rr<R, R2>(register: R, to_add: R2, flags: &mut Flags) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u16>,
    R2: ReadRegister<u16>,
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

pub(crate) fn increment_r<R>(register: R, flags: &mut Flags) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u8>,
{
    let result = register.get().wrapping_add(1);

    update_half_carry_8bit(register.get(), Operation::Inc(1), flags);
    flags.zero = result == 0;
    flags.substraction = false;

    register.set(result);

    (1, 1)
}

pub(crate) fn decrement_r<R>(register: R, flags: &mut Flags) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u8>,
{
    let result = register.get().wrapping_sub(1);

    update_half_carry_8bit(register.get(), Operation::Sub(1), flags);
    flags.zero = result == 0;
    flags.substraction = false;

    register.set(result);

    (1, 1)
}

pub(crate) fn call(address: u16, registers: &mut Registers, bus: &mut Bus) -> (Bytes, Cycles) {
    // We add 3 because we need to push the address of the instruction next to the call,
    // and long is 3 bytes long
    let (p, c) = split_u16_into_two_u8s(registers.pc.wrapping_add(3));

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

    (2, 3)
}

pub(crate) fn return_(registers: &mut Registers, bus: &mut Bus) -> (Bytes, Cycles) {
    let c = bus[registers.sp];
    registers.sp = registers.sp.wrapping_add(1);
    let p = bus[registers.sp];
    registers.sp = registers.sp.wrapping_add(1);
    registers.pc = merge_two_u8s_into_u16(p, c);

    (0, 4)
}

pub(crate) fn push<R, R2>(
    register_h: &R,
    register_l: &R2,
    sp: &mut u16,
    bus: &mut Bus,
) -> (Bytes, Cycles)
where
    R: ReadRegister<u8>,
    R2: ReadRegister<u8>,
{
    *sp = sp.wrapping_sub(1);
    bus[*sp] = register_h.get();
    *sp = sp.wrapping_sub(1);
    bus[*sp] = register_l.get();

    (1, 4)
}

pub(crate) fn pop<R>(register: R, sp: &mut u16, bus: &mut Bus) -> (Bytes, Cycles)
where
    R: ReadWriteRegister<u16>,
{
    let low = bus[*sp];
    *sp = sp.wrapping_add(1);
    let high = bus[*sp];
    *sp = sp.wrapping_add(1);
    register.set(merge_two_u8s_into_u16(high, low));

    (1, 4)
}

pub(crate) fn load_ram_into_r_and_in<RR, R>(
    address_register: RR,
    register: R,
    amount: Operation,
    bus: &mut Bus,
) -> (Bytes, Cycles)
where
    RR: ReadWriteRegister<u16>,
    R: ReadWriteRegister<u8>,
{
    let address = address_register.get();

    register.set(bus[address]);

    address_register.set(match amount {
        Operation::Inc(amount) => address.wrapping_add(amount as u16),
        Operation::Sub(amount) => address.wrapping_sub(amount as u16),
    });

    (1, 2)
}

// Utilities
pub(crate) enum Operation {
    Inc(u8),
    Sub(u8),
}

fn update_half_carry_8bit(register_value: u8, amount: Operation, flags: &mut Flags) {
    let last_four_bits = register_value & 0x0F;
    let result = match amount {
        Operation::Inc(number) => last_four_bits.checked_add(number),
        Operation::Sub(number) => last_four_bits.checked_sub(number),
    };

    match result {
        Some(_) => flags.half_carry = false,
        None => flags.half_carry = true,
    }
}
