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
