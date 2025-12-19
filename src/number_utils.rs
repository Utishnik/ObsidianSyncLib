use crate::{debug::*, debug_eprintln_fileinfo};
use std::fmt;

#[derive(Debug,Clone)]
pub enum DivisionError {
    DivisionByZero,
    Overflow,
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DivisionError::DivisionByZero => write!(f, "Division By Zero"),
            DivisionError::Overflow => write!(f, "overlow"),
        }
    }
}

fn divide_with_remainder(dividend: u128, divisor: u128) -> (Option<u128>, Option<u128>) {
    let intpart: Option<u128> = dividend.checked_div(divisor);
    let rem: Option<u128> = dividend.checked_rem(divisor);
    let result: (Option<u128>, Option<u128>) = (intpart, rem);
    result
}

pub fn safe_divide_with_remainder(
    dividend: u128,
    divisor: u128,
) -> Result<(u128, u128), DivisionError> {
    let mut result: (u128, u128) = (0, 0);
    if divisor == 0 {
        debug_eprintln_fileinfo!("safe_divide_with_remainder divisor is Null");
        return Err(DivisionError::DivisionByZero);
    }
    let option_res: (Option<u128>, Option<u128>) = divide_with_remainder(dividend, divisor);
    if option_res.0.is_none() {
        debug_eprintln_fileinfo!("safe_divide_with_remainder intpart is None");
        return Err(DivisionError::Overflow);
    } else if option_res.1.is_none() {
        debug_eprintln_fileinfo!("safe_divide_with_remainder rem is None");
        return Err(DivisionError::Overflow);
    }
    result.0 = option_res.0.unwrap(); //safe
    result.1 = option_res.1.unwrap(); //safe
    Ok(result)
}
