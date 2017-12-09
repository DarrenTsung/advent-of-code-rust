// Problem Location: http://adventofcode.com/2017/day/8
extern crate simple_error;

use std::result::Result;
use std::error::Error;
use std::collections::HashMap;

pub fn run_instructions(instructions : &str) -> Result<i32, Box<Error>> {
    let mut registry : HashMap<&str, i32> = HashMap::new();

    for line in instructions.lines() {
        let tokens : Vec<&str> = line.split_whitespace().collect();

        // expect line to come in the format of:
        // b inc 5 if a > 1
        if tokens.len() != 7 {
            bail!("Invalid instruction line format!");
        }

        let condition_register = tokens[4];
        let condition_comparator = tokens[5];
        let condition_value = tokens[6].parse::<i32>()?;

        let condition : Box<Fn(i32) -> bool> = match condition_comparator {
            ">" => Box::new(|x| x > condition_value),
            ">=" => Box::new(|x| x >= condition_value),
            "<" => Box::new(|x| x < condition_value),
            "<=" => Box::new(|x| x <= condition_value),
            "==" => Box::new(|x| x == condition_value),
            "!=" => Box::new(|x| x != condition_value),
            _ => bail!("Invalid condition_comparator in instruction!"),
        };

        let condition_register_value = *registry.entry(condition_register).or_insert(0);
        // if doesn't match condition
        if !condition(condition_register_value) {
            continue;
        }

        let register = tokens[0];
        let instruction : Box<Fn(i32, i32) -> i32> = match tokens[1] {
            "inc" => Box::new(|x, y| x + y),
            "dec" => Box::new(|x, y| x - y),
            _ => bail!("Invalid instruction name inside instruction!"),
        };
        let instruction_value = tokens[2].parse::<i32>()?;
        let register_entry = registry.entry(register).or_insert(0);
        *register_entry = instruction(*register_entry, instruction_value);
    }

    match registry.values().max() {
        Some(v) => Ok(*v),
        None => Ok(0),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_basic_instructions() {
        assert_eq!(run_instructions("b inc 5 if a > 1\n\
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10").unwrap(), 1);
    }

    #[test]
    fn handle_invalid_instructions() {
        assert!(run_instructions("b inc 5").is_err());
    }

    #[test]
    fn handle_empty_instructions() {
        assert_eq!(run_instructions("").unwrap(), 0);
    }

}