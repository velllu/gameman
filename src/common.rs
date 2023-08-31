pub(crate) fn merge_two_u8s_into_u16(first: u8, second: u8) -> u16 {
    ((first as u16) << 8 as u16) | second as u16
}

pub(crate) fn split_u16_into_two_u8s(value: u16) -> (u8, u8) {
    let first_u8 = (value >> 8) as u8;
    let second_u8 = (value & 0xFF) as u8;

    (first_u8, second_u8)
}
