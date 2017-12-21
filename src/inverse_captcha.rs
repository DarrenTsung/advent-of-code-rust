// Problem Location: http://adventofcode.com/2017/day/1
use std::result::Result;

pub fn solve(s : &str) -> Result<u32, &'static str> {
    if s.len() <= 0 {
        return Ok(0);
    }

    let mut sum : u32 = 0;

    // NOTE (darren): is this the best way to get the last digit?
    let last_char = s.chars().rev().next().unwrap(); // unwrap ok because s.len() > 0
    let mut previous_digit = helper_to_digit(last_char)?;

    for c in s.chars() {
        let digit = helper_to_digit(c)?;
        if digit == previous_digit {
            sum += digit;
        }
        previous_digit = digit;
    }

    Ok(sum)
}

fn helper_to_digit(c: char) -> Result<u32, &'static str> {
    const RADIX: u32 = 10;

    match c.to_digit(RADIX) {
        Some(digit) => Ok(digit),
        None => Err("Failed to parse digit (not 0-9) inside input!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_sum() {
        assert!(solve("1122") == Ok(3));
    }

    #[test]
    fn non_consecutive_doesnt_sum() {
        assert!(solve("1234") == Ok(0));
    }

    #[test]
    fn invalid_input_returns_err() {
        assert!(solve("11-22").is_err());
    }

    #[test]
    fn wraps_around_string() {
        assert!(solve("91212129") == Ok(9));
    }
}
