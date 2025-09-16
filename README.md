# BBP Formula π Calculator

[中文版本](README-zh.md) | [Русский версия](README-ru.md) | [日本語版](README-ja.md) | [Español](README-es.md) | [Français](README-fr.md)

This project is a Rust implementation of the Bailey-Borwein-Plouffe (BBP) formula for calculating the digits of π. Its most remarkable feature is the ability to compute the *n*th hexadecimal digit of π directly, without needing to calculate all the preceding digits.

## The BBP Formula

The BBP formula was discovered in 1995 by Simon Plouffe in collaboration with David H. Bailey and Peter Borwein. It provides a way to calculate π in base 16.

The formula is:
![BBP formula](https://latex.codecogs.com/png.latex?\pi%20=%20\sum_{k=0}^{\infty}%20\frac{1}{16^k}%20\left(%20\frac{4}{8k+1}%20-%20\frac{2}{8k+4}%20-%20\frac{1}{8k+5}%20-%20\frac{1}{8k+6}%20\right))

This spigot algorithm allows for the extraction of individual hexadecimal digits of π.

## Mathematical Logic: A Student's Guide

Imagine you want to find the 100th digit of π. Normally, you would have to calculate all 99 digits before it. The BBP formula is like a magic trick that lets you find the 100th hexadecimal digit of π all by itself, without needing the ones that come before it.

Here’s how the magic trick works, broken down into simple steps.

### The Core Idea: Shifting the Decimal Point

The number π in hexadecimal starts as `3.243F6A...`.

- The 1st digit after the point is `2`.
- The 2nd is `4`.
- The 3rd is `3`.

Let's say we want to find the **2nd** digit (`4`). If we multiply π by 16 (which is `10` in hexadecimal), the hexadecimal point shifts one place to the right:
`π * 16^1 = 32.43F6A...`

Now, the digit we want (`4`) is the first one after the new hexadecimal point.

If we want the **4th** digit (`F`), we multiply by `16^3`:
`π * 16^3 = 3243.F6A...`

The digit we want (`F`) is now the first one after the point.

So, the main idea is: **To find the *n*th hexadecimal digit of π, we can calculate `16^(n-1) * π` and look at the first digit after the hexadecimal point.**

### The Problem: We Can't Use π Directly

This sounds easy, but there's a catch: to calculate `16^(n-1) * π`, we need to know π. But π has infinite digits! We can't store it in a computer.

This is where the BBP formula comes to the rescue. It gives us a way to calculate the *fractional part* of `16^(n-1) * π` without ever needing to know all of π.

### The Solution: Splitting the BBP Formula

The BBP formula is a giant sum of many small fractions. When we multiply it by `16^(n-1)`, we get a new giant sum:
![Multiplied BBP formula](https://latex.codecogs.com/png.latex?16^{n-1}\pi%20=%20\sum_{k=0}^{\infty}%20\left(%20\frac{4\cdot%2016^{n-1-k}}{8k+1}%20-%20\frac{2\cdot%2016^{n-1-k}}{8k+4}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+5}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+6}%20\right))

We only care about the fractional part of this sum. The key insight is that we can split this sum into two parts:

1.  **The "Head" (for `k` from 0 to `n-1`)**:
    These are terms where the power of 16 is positive (`16^(n-1-k)`). These create large integer numbers. Since we only care about the fractional part, we can use a clever math trick called **modular exponentiation**. It lets us find the fractional part of each term without ever calculating the huge numbers involved. It's like finding the remainder of a division without doing the full division.

2.  **The "Tail" (for `k` from `n` to infinity)**:
    These are terms where the power of 16 is negative. These create very small numbers (e.g., `1/16`, `1/256`). We can add these up using standard computer floating-point numbers (`f64`). We don't need to sum to infinity; we just add terms until they become so small they don't affect our result.

By adding the fractional part from the "Head" and the full value of the "Tail", we get the total fractional part of `16^(n-1) * π`.

### Prerequisites (quick)

To get the most from this explanation you should be comfortable with:
- reading numbers in base 16 (hexadecimal),
- basic infinite sums / series (what it means to split a sum), and
- the idea of remainders or `mod` arithmetic (we use modular exponentiation as a tool).

If some of those are new, the short primer below will help.

### A very short modular-arithmetic primer

Modular arithmetic is about remainders. For example, "a mod m" is the remainder when `a` is divided by `m`. A useful identity is:

16^e mod m can be computed efficiently even when `e` is large — this is called modular exponentiation. Example: `16^3 = 4096`, and `4096 mod 7 = 4096 - 7*585 = 1`, so `16^3 mod 7 = 1`.

Why this matters: when we compute the head terms (those with non-negative powers of 16) we only need their fractional part. Using modular arithmetic we can compute that fractional contribution without forming enormous integers.

### Worked numeric example (n = 2)

We'll find the 2nd hexadecimal digit of π (which is `4` in `3.243F6A...`) using the same steps the code implements. For `n = 2` we multiply by `16^(n-1) = 16`.

1. Write the multiplied BBP sum (showing the four series):

    16\pi = \n
    \sum_{k=0}^{\infty} \left( \frac{4\cdot 16^{1-k}}{8k+1} - \frac{2\cdot 16^{1-k}}{8k+4} - \frac{1\cdot 16^{1-k}}{8k+5} - \frac{1\cdot 16^{1-k}}{8k+6} \right)

2. Split into Head (k=0..1) and Tail (k>=2).

    - Head terms (k = 0 and 1): these produce positive powers 16^{1-k} = 16^1 and 16^0. Compute each term's fractional part. Use modular exponentiation to reduce numerator powers modulo the denominator and then divide to get the fractional contribution.

    - Tail terms (k >= 2): these have negative powers (16^{-1}, 16^{-2}, ...). Numerically they are small; summing a few of them in double precision gives a good approximation.

3. Sum fractional Head + Tail to get fractional part of 16π, then multiply fractional part by 16 and take the integer part to get the digit.

Concretely, carrying out these steps numerically yields a fractional part whose first hex digit is `4`, matching the known expansion.

### Tail precision note

The README previously suggested using `f64` for the tail. That's fine for extracting single hex digits for moderate `n` because tail terms drop exponentially. If you plan to extract very large `n` or convert many hex digits into decimal with high precision, prefer arbitrary-precision arithmetic (the project already uses `bigdecimal` for the hex→decimal conversion).

### Where to look in the code

- The BBP head/tail logic and modular exponentiation are in `src/bbp/mod.rs` (functions: `get_digit`, `series_sum`, `mod_pow`).
- The hexadecimal-to-decimal conversion uses the `bigdecimal` crate and lives in `src/decimal/mod.rs` (`hex_pi_to_decimal`).

### Final Step: Getting the Digit

Let's say our final fractional part is `0.43F6A...`. To isolate the first digit (`4`), we just multiply by 16:
`0.43F6A... * 16 = 4.3F6A...`

The integer part of this result, `4`, is the *n*th hexadecimal digit of π we were looking for!

That's the complete algorithm. It cleverly avoids infinite numbers and huge calculations by splitting the problem and using modular arithmetic, allowing us to pinpoint a single digit of π from the vastness of its infinite sequence.

## Code Implementation

The project is organized into three main modules:

### `src/main.rs`
This is the main entry point of the application. It:
1.  Calls `bbp::get_digits()` to calculate a sequence of hexadecimal digits of π.
2.  Prints the hexadecimal representation.
3.  Uses `decimal::hex_pi_to_decimal()` to convert the hexadecimal string into a high-precision decimal string.
4.  Prints the decimal representation.
5.  Demonstrates the calculation of individual digits (e.g., the 10th and 50th).
6.  Provides a verification section to compare the result against a known value of π.

### `src/bbp/mod.rs`
This module contains the core logic for the BBP formula.
-   `get_digit(n)`: Calculates the *n*th hexadecimal digit of π (1-based).
-   `series_sum(n, j)`: Computes one of the four series sums from the BBP formula for a given digit `n` and denominator offset `j`.
-   `mod_pow(base, exp, modulus)`: An efficient implementation of modular exponentiation to handle large numbers without overflow.

### `src/decimal/mod.rs`
This module is responsible for converting the calculated hexadecimal digits into a standard decimal representation.
-   `hex_pi_to_decimal(hex_digits, precision)`: Takes a string of hexadecimal digits and converts them to a decimal string. It uses the `bigdecimal` crate to perform arbitrary-precision arithmetic, ensuring that the conversion is accurate and not subject to the limitations of standard floating-point types.

## Usage

### Prerequisites
You need to have the Rust toolchain installed. You can install it from [rustup.rs](https://rustup.rs/).

### Running the Project
To run the application, navigate to the project directory and use Cargo:
```bash
cargo run
```

The program will output:
1.  The first 500 hexadecimal digits of π.
2.  The decimal conversion of those digits.
3.  The 10th and 50th hexadecimal digits calculated individually.
4.  A verification comparing the result to the known first 20 decimal digits of π.

### Building the Project
To build an optimized executable, run:
```bash
cargo build --release
```
The binary will be located at `target/release/bbp`.

## Dependencies
-   `bigdecimal`: Used for high-precision decimal arithmetic to ensure accurate conversion from hexadecimal to decimal.
