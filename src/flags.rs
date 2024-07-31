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

    pub(crate) fn set_flags(
        &mut self,
        zero: bool,
        subtraction: bool,
        half_carry: bool,
        carry: bool,
    ) {
        self.zero = zero;
        self.substraction = subtraction;
        self.half_carry = half_carry;
        self.carry = carry;
    }
}
