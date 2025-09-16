//! BBP (Bailey-Borwein-Plouffe) Formula Calculator for π
//! 
//! This module implements the BBP formula to calculate hexadecimal digits of π directly.
//! The BBP formula, discovered in 1995, allows computing the nth hexadecimal digit of π
//! without calculating all previous digits.
//!
//! The formula used is:
//! π = ∑[k=0 to ∞] 1/16^k * (4/(8k+1) - 2/(8k+4) - 1/(8k+5) - 1/(8k+6))
//!
//! For details on the formula and its derivation, see:
//! Bailey, D. H.; Borwein, P. B.; Plouffe, S. (1997). 
//! "On the Rapid Computation of Various Polylogarithmic Constants"
//! Mathematics of Computation. 66 (218): 903–913

/// Computes (base^exp) mod modulus efficiently using the square-and-multiply algorithm.
/// 
/// This function implements modular exponentiation without overflowing by taking the modulus
/// at each step of the calculation.
/// 
/// # Arguments
/// * `base` - The base number to be raised to a power
/// * `exp` - The exponent
/// * `modulus` - The modulus to apply during the calculation
/// 
/// # Returns
/// * The result of (base^exp) mod modulus as an i64
/// 
/// # Example
/// ```
/// let result = mod_pow(2, 10, 1000);  // Calculates 2^10 mod 1000
/// ```
fn mod_pow(base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    let mut base = base.rem_euclid(modulus);
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base).rem_euclid(modulus);
        }
        base = (base * base).rem_euclid(modulus);
        exp >>= 1;
    }
    result
}

/// Computes one of the four series in the BBP formula.
/// 
/// This function calculates the sum:
/// ∑[k=0 to ∞] 16^(n-k) / (8k + j)
/// 
/// The calculation is split into two parts:
/// 1. For k ≤ n: Uses modular arithmetic to handle large powers of 16
/// 2. For k > n: Uses floating-point arithmetic as the powers become small
/// 
/// # Arguments
/// * `n` - The position (0-based) of the hexadecimal digit to calculate
/// * `j` - The denominator offset (1, 4, 5, or 6 in the BBP formula)
/// 
/// # Returns
/// * The fractional part of the series sum as an f64
fn series_sum(n: i32, j: i32) -> f64 {
    let mut sum = 0.0;
    let mut k: i64 = 0;
    let n64 = n as i64;
    
    // First sum (k ≤ n)
    while k <= n64 {
        let r = 8 * k + j as i64;
        if r != 0 {  // Avoid division by zero
            let num = mod_pow(16, n64 - k, r);
            sum += (num as f64) / (r as f64);
            sum = sum.fract();
        }
        k += 1;
    }
    
    // Second sum (k > n)
    let terms = n64 + 100; // More terms for better precision
    while k <= terms {
        let r = 8 * k + j as i64;
        let exp = n64 - k;
        let t = if exp >= 0 {
            16.0_f64.powi(exp as i32) / r as f64
        } else {
            1.0 / (r as f64 * 16.0_f64.powi((-exp) as i32))
        };
        sum += t;
        sum = sum.fract();
        if t.abs() < 1e-17 { break; }
        k += 1;
    }
    
    sum.fract()
}

/// Calculates the nth hexadecimal digit of π using the BBP formula.
/// 
/// This implementation combines the four series in the BBP formula:
/// π = ∑[k=0 to ∞] 1/16^k * (4/(8k+1) - 2/(8k+4) - 1/(8k+5) - 1/(8k+6))
/// 
/// To maintain precision, the fractional part is taken after each series addition
/// to prevent floating-point overflow.
/// 
/// # Arguments
/// * `n` - The position of the hexadecimal digit to calculate (1-based indexing)
/// 
/// # Returns
/// * The nth hexadecimal digit of π as a u8 (0-15)
/// 
/// # Example
/// ```
/// let third_digit = bbp_pi_digit(3);  // Returns 3 (as π = 3.243F6A...)
/// ```
pub fn get_digit(n: i32) -> u8 {
    let n = n - 1; // Convert to 0-based index
    let mut sum = 4.0 * series_sum(n, 1);
    sum = sum.fract();
    sum -= 2.0 * series_sum(n, 4);
    sum = sum.fract();
    sum -= series_sum(n, 5);
    sum = sum.fract();
    sum -= series_sum(n, 6);
    sum = sum.fract();
    
    if sum < 0.0 {
        sum += 1.0;
    }
    ((16.0 * sum).floor() as u8) & 0xF
}

/// Returns a string containing the first n hexadecimal digits of π after the decimal point.
/// 
/// # Arguments
/// * `n` - The number of hexadecimal digits to calculate
/// 
/// # Returns
/// * A String containing the hexadecimal digits
/// 
/// # Example
/// ```
/// let pi = get_digits(5);  // Returns "243F6"
/// ```
pub fn get_digits(n: i32) -> String {
    (1..=n).map(|i| format!("{:X}", get_digit(i))).collect()
}