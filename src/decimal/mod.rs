//! Decimal converter for π digits
//! 
//! This module provides functionality to convert hexadecimal digits of π
//! to their decimal representation.

use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

/// Converts hexadecimal digits of π to decimal representation with specified precision.
/// 
/// This function uses `BigDecimal` for high-precision arithmetic to avoid floating-point
/// inaccuracies.
/// 
/// # Arguments
/// * `hex_digits` - A string of hexadecimal digits (after the decimal point)
/// * `precision` - Number of decimal places desired in the output
/// 
/// # Returns
/// * A string containing π in decimal format with the specified precision
/// 
/// # Example
/// ```
/// let hex = "243F6A8885A3";
/// let decimal = hex_pi_to_decimal(hex, 10);
/// assert_eq!(&decimal[0..12], "3.1415926535");
/// ```
pub fn hex_pi_to_decimal(hex_digits: &str, precision: usize) -> String {
    let mut fraction = BigDecimal::zero();
    let mut power_of_16 = BigDecimal::from(1);

    // Process each hexadecimal digit
    for c in hex_digits.chars() {
        if let Some(digit_val) = c.to_digit(16) {
            power_of_16 *= 16;
            let digit = BigDecimal::from(digit_val);
            fraction += digit / &power_of_16;
        }
    }
    
    let three = BigDecimal::from(3);
    let pi = three + fraction;
    
    // Format the result with specified precision
    let pi_str = pi.to_string();
    if let Some(dot_index) = pi_str.find('.') {
        let int_part = &pi_str[..dot_index];
        let frac_part = &pi_str[dot_index+1..];
        if frac_part.len() > precision {
            format!("{}.{}", int_part, &frac_part[..precision])
        } else {
            pi_str
        }
    } else {
        pi_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digits() {
        let hex = "243F6A8885A3";
        let decimal = hex_pi_to_decimal(hex, 12);
        assert_eq!(&decimal, "3.141592653589");
    }

    #[test]
    fn test_more_digits() {
        let hex = "243F6A8885A308D313198A2E0370734";
        let decimal = hex_pi_to_decimal(hex, 20);
        assert_eq!(&decimal, "3.14159265358979323846");
    }

    #[test]
    fn test_single_digit() {
        let hex = "2";
        let decimal = hex_pi_to_decimal(hex, 3);
        assert_eq!(&decimal, "3.125");
    }
}