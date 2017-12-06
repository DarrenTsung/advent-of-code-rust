// Problem Location: http://adventofcode.com/2017/day/6
use std::collections::HashSet;

pub fn solve(memory_bank : &str) -> u32 {
    let mut memory_bank : Vec<i32> = memory_bank.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect();
    let memory_bank_len = memory_bank.len();

    let mut seen_memory_banks : HashSet<Vec<i32>> = HashSet::new();
    let mut iterations = 0;

    loop {
        if seen_memory_banks.contains(&memory_bank) {
            return iterations;
        }
        seen_memory_banks.insert(memory_bank.clone());

        // NOTE (darren): I don't understand how to compute the max_index here
        // without borrowing memory_bank - note to ask about this later..
        let hack_memory_bank = memory_bank.clone();
        let (max_index, max_value) = hack_memory_bank.iter().enumerate()
            .max_by(|&(index_a, val_a), &(index_b, val_b)| {
                val_a.cmp(val_b).then_with(|| index_b.cmp(&index_a))
            }).unwrap();
        let mut max_value = *max_value;
        memory_bank[max_index] = 0;

        let mut current_index = max_index;

        while max_value > 0 {
            current_index = (current_index + 1) % memory_bank_len;

            memory_bank[current_index] += 1;
            max_value -= 1;
        }

        iterations += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_basic_case() {
        assert_eq!(solve("0 2 7 0"), 5);
    }

    #[test]
    fn handles_simple_case() {
        assert_eq!(solve("0 1"), 2);
    }

    #[test]
    fn handles_only_one_iteration() {
        assert_eq!(solve("1"), 1);
    }
}