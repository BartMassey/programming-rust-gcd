// Copyright © 2018 Jim Blandy, Jason Orendorff, Bart Massey
// This work is released under the "MIT License".
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Gcd example from Blandy & Orendorff, ch 1.
// Calculate the common GCD of a list of numbers
// presented as command-lne arguments.

use std::env;
use std::str::FromStr;

// Compute the GCD of two numbers.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    let n1 = 2 * 3 * 5 * 11 * 17;
    let n2 = 3 * 7 * 11 * 13 * 19;
    let d = 3 * 11;
    assert_eq!(gcd(n1, n2), d)
}

// Keep a running GCD of inputs and print as needed.
fn main() {
    let mut d = None;
    for arg in env::args().skip(1) {
        let arg = u64::from_str(&arg).expect("bad argument");
        d = match d {
            None => Some(arg),
            Some(d) => Some(gcd(d, arg)),
        }
    }
    if let Some(d) = d {
        println!("{}", d)
    }
}
