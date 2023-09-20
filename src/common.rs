pub(crate) fn merge_two_u8s_into_u16(first: u8, second: u8) -> u16 {
    ((first as u16) << 8u16) | second as u16
}

pub(crate) fn split_u16_into_two_u8s(value: u16) -> (u8, u8) {
    let first_u8 = (value >> 8) as u8;
    let second_u8 = (value & 0xFF) as u8;

    (first_u8, second_u8)
}

pub(crate) enum Operator {
    Inc,
    Sub,
}

pub(crate) enum BitwiseOperation {
    And,
    Or,
    Xor,
}

/// Get a specific bit from a `u8`
/// # Examples
/// ```
/// use emulator::common::get_bit;
///
/// let x: u8 = 0b0010_0000;
/// assert_eq!(true, get_bit(0b0010_0000, 5));
/// assert_eq!(true, get_bit(0b0000_0001, 0));
/// assert_eq!(false, get_bit(0b0000_0001, 6));
/// ```
pub fn get_bit(value: u8, offset: u8) -> bool {
    (value >> offset) & 0b00000001 == 1
}
