pub(crate) fn merge_two_u8s_into_u16(first: u8, second: u8) -> u16 {
    ((first as u16) << 8u16) | second as u16
}

pub(crate) fn split_u16_into_two_u8s(value: u16) -> (u8, u8) {
    let first_u8 = (value >> 8) as u8;
    let second_u8 = (value & 0xFF) as u8;

    (first_u8, second_u8)
}

pub(crate) trait Bit {
    fn get_bit(&self, offset: Self) -> bool;
    fn set_bit(&mut self, offset: Self, value: bool);
}

impl Bit for u8 {
    fn get_bit(&self, offset: u8) -> bool {
        (self >> offset) & 0b00000001 != 0
    }

    fn set_bit(&mut self, offset: u8, value: bool) {
        if value {
            *self |= 1 << offset;
        } else {
            *self &= !(1 << offset);
        }
    }
}
