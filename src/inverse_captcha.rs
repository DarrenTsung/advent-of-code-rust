// Problem Location: http://adventofcode.com/2017/day/1
use std::option::Option;

pub fn solve(s : &str) -> Option<u32> {
    const RADIX: u32 = 10;

    if s.len() <= 0 {
        return Some(0);
    }

    let mut sum = 0;
    let mut previous_digit = s.chars().rev().next()?.to_digit(RADIX)?; // s.len() > 0 so last character exists
    for c in s.chars() {
        match c.to_digit(RADIX) {
            Some(digit) => {
                if digit == previous_digit {
                    sum += digit;
                }
                previous_digit = digit;
            },
            None => {
                return None;
            },
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_sum() {
        assert!(solve("1122").unwrap() == 3);
    }

    #[test]
    fn non_consecutive_doesnt_sum() {
        assert!(solve("1234").unwrap() == 0);
    }

    #[test]
    fn invalid_input_returns_none() {
        assert!(solve("11-22").is_none());
    }

    #[test]
    fn wraps_around_string() {
        assert!(solve("91212129").unwrap() == 9);
    }
}
