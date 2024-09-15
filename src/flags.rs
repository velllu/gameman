use crate::common::Bit;

#[derive(PartialEq, Eq)]
pub struct Flags {
    /// This is set when the result is zero
    pub zero: bool,

    /// This is set when the operation is a subtraction
    pub subtraction: bool,

    /// This is set if the lower 4 bits overflow
    pub half_carry: bool,

    /// This is set if a value overflows
    pub carry: bool,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            zero: true,
            subtraction: false,
            half_carry: true,
            carry: true,
        }
    }

    pub(crate) fn is_condition_valid(&self, condition_num: u8) -> bool {
        match condition_num & 0b00000011 {
            0 => !self.zero,
            1 => self.zero,
            2 => !self.carry,
            3 => self.carry,
            _ => unreachable!(),
        }
    }

    pub(crate) fn get_byte(&self) -> u8 {
        let mut byte = 0;

        byte |= (self.zero as u8) << 7;
        byte |= (self.subtraction as u8) << 6;
        byte |= (self.half_carry as u8) << 5;
        byte |= (self.carry as u8) << 4;

        byte
    }

    pub(crate) fn set_from_byte(&mut self, byte: u8) {
        self.zero = byte.get_bit(7);
        self.subtraction = byte.get_bit(6);
        self.half_carry = byte.get_bit(5);
        self.carry = byte.get_bit(4);
    }
}

/// Operations that work like the wrapping ones but also give you the flags
pub(crate) trait FlagOperations {
    fn flag_add(&self, rhs: Self) -> (Self, Flags)
    where
        Self: Sized;

    fn flag_sub(&self, rhs: Self) -> (Self, Flags)
    where
        Self: Sized;
}

impl FlagOperations for u8 {
    fn flag_add(&self, rhs: u8) -> (Self, Flags) {
        let (result, has_overflown) = self.overflowing_add(rhs);

        (
            result,
            Flags {
                zero: result == 0,
                subtraction: false,
                half_carry: (self & 0x0F).wrapping_add(rhs & 0x0F) > 0x0F,
                carry: has_overflown,
            },
        )
    }

    fn flag_sub(&self, rhs: u8) -> (Self, Flags) {
        let (result, has_overflown) = self.overflowing_sub(rhs);

        (
            result,
            Flags {
                zero: result == 0,
                subtraction: true,
                half_carry: (self & 0x0F).wrapping_sub(rhs & 0x0F) > 0x0F,
                carry: has_overflown,
            },
        )
    }
}
