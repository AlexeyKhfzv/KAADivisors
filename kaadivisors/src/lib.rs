//! # Divisors
//!
//! Simple CLI program for finding divisors of huge numbers
//!
//! **Run** - `kaadivs` or `kaadivs <your_number>` (example: kaadivs 123)

/// This function returns the number of times the divisor occurs in the number (the power)
///
/// # Panics
/// * Panics when the divisor < 2, message: "The divisor must be in range [2; 2^128), got {divisor}"
/// * Panics when the number = 0, message: "The number must be in range [1; 2^128), got {number}"
///
/// # Example
/// ```
/// let mut number = 8;
/// assert_eq!(3, kaadivisors::get_power(&mut number, 2));
/// ```
pub fn get_power(number: &mut u128, divisor: u128) -> u16 {
    if divisor < 2 {
        panic!("The divisor must be in range [2; 2^128), got {divisor}");
    }
    if *number == 0 {
        panic!("The number must be in range [1; 2^128), got {number}");
    }

    let mut power: u16 = 0;
    loop {
        if *number % divisor == 0_u128 {
            *number /= divisor;
            power += 1;
        } else {
            break;
        }
    }

    power
}

/// This function returns all divisors of the number with their power
///
/// # Example
/// ```
/// assert_eq!(vec![(2, 2), (3, 1)], kaadivisors::get_divisors(12));
/// ```
pub fn get_divisors(mut number: u128) -> Vec<(u128, u16)> {
    if number < 2 {
        return vec![];
    }
    if number < 4 {
        return vec![(number, 1)];
    }

    let mut result: Vec<(u128, u16)> = vec![];

    if number % 2 == 0 {
        result.push((2, get_power(&mut number, 2)));
    }
    if number % 3 == 0 {
        result.push((3, get_power(&mut number, 3)));
    }

    let mut divisor = 5;
    while divisor * divisor < number + 1 {
        if number % divisor == 0 {
            result.push((divisor, get_power(&mut number, divisor)));
        }
        if number % (divisor + 2) == 0 {
            result.push((divisor + 2, get_power(&mut number, divisor + 2)));
        }

        divisor += 6;
    }

    if number != 1 {
        result.push((number, 1));
    }

    result
}
