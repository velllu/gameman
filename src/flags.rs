pub struct Flags {
    /// This is set when the result is zero
    pub zero: bool,

    /// This is set when the operation is a subtraction
    pub substraction: bool,

    /// This is set if the lower 4 bits overflow
    pub half_carry: bool,

    /// This is set if a value overflows
    pub carry: bool,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            zero: true,
            substraction: false,
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
}
