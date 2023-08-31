pub struct Flags {
    /// This gets set to true after operations like `INC R` result in a zero, note that
    /// for some reason, some operations aren't like this, for example, `INC RR` never
    /// sets the zero flag
    pub zero: bool,

    // TODO: Document this other flags
    pub substraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            zero: false,
            substraction: false,
            half_carry: false,
            carry: false,
        }
    }
}
