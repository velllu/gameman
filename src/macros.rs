#[macro_export]
macro_rules! update_zero_flag {
    (zero_flag: $expr:expr, result: $expr2:expr) => {{
        if result == 0 {
            $zero_flag = true;
        } else {
            $zero_flag = false;
        }

        $result
    }};
}
