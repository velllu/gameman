pub struct Flags {
    /// This gets set to true after operations like `INC R` result in a zero, note that
    /// for some reason, some operations aren't like this, for example, `INC RR` never
    /// sets the zero flag
    pub zero: bool,

    /// IME, standing for "Interrupt Master Enable" is basically a switch on whether
    /// interrupts should be handled or not
    pub ime: bool,

    // TODO: Document this other flags
    pub substraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            zero: false,
            ime: false,
            substraction: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl Flags {
    pub(crate) fn update_zero_flag(&mut self, result: u8) {
        self.zero = result == 0;
    }
}
