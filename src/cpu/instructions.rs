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

use crate::{bus::Bus, flags::Flags, registers::Register};

pub(crate) fn load_ii_into_rr<R>(register: &mut R, data: u16) -> (Bytes, Cycles)
where
    R: Register<u16>,
{
    register.set(data);
    (3, 3)
}

pub(crate) fn load_r_into_ram<RAM, R>(address: RAM, register: R, bus: &mut Bus) -> (Bytes, Cycles)
where
    RAM: Register<u16>,
    R: Register<u8>,
{
    bus[address.get()] = register.get();
    bus[address.get()] = register.get();
    (1, 2)
}

pub(crate) fn increment_rr<R>(register: &mut R) -> (Bytes, Cycles)
where
    R: Register<u16>,
{
    register.set(register.get().wrapping_add(1));
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
