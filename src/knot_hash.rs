// Problem Location: http://adventofcode.com/2017/day/10
use std::cmp;
use std::num::ParseIntError;

pub fn hash(input : &str) -> Result<u32, ParseIntError> {
    let lengths : Vec<u32> = input.split(|c| char::is_whitespace(c) || c == ',')
        .map(|s| s.parse::<u32>()).collect::<Result<Vec<_>, _>>()?;

    const LIST_SIZE : u32 = 256;

    Ok(hash_internal(LIST_SIZE, lengths))
}

fn hash_internal(list_size : u32, lengths : Vec<u32>) -> u32 {
    if list_size < 2 {
        panic!("List size cannot be less than 2!");
    }

    let mut list : Vec<u32> = (0..list_size).collect();
    let mut skip = 0;
    let mut current_position = 0;

    for length in lengths {
        let length = cmp::min(length, list_size);

        // swap everything between low && high
        let mut low = current_position;
        let mut high = current_position + length - 1;

        while low < high {
            let wrapped_high : usize = (high % list_size) as usize;
            let wrapped_low : usize = (low % list_size) as usize;

            let swap = list[wrapped_high];
            list[wrapped_high] = list[wrapped_low];
            list[wrapped_low] = swap;

            high -= 1;
            low += 1;
        }

        current_position = (current_position + length + skip) % list_size;
        skip += 1;
    }

    list[0] * list[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_example() {
        // final: 3 4 2 1 0
        assert_eq!(hash_internal(5, vec![3, 4, 1, 5]), 12);
    }

    #[test]
    fn handles_empty_lengths() {
        // final: 0 1 2 3 4
        assert_eq!(hash_internal(5, vec![]), 0);
    }

    #[test]
    fn handles_larger_length_than_size() {
        // final: 4 3 2 1 0
        assert_eq!(hash_internal(5, vec![8]), 12);
    }
}