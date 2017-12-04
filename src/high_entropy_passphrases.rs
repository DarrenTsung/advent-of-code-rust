// Problem Location: http://adventofcode.com/2017/day/4
use std::collections::HashSet;

pub fn valid_count(passphrases : &str) -> u32 {
    let mut total = 0;

    for line in passphrases.lines() {
        let mut words = HashSet::new();

        let mut is_valid = true;
        for word in line.split_whitespace() {
            if words.contains(word) {
                is_valid = false;
                break;
            }

            words.insert(word);
        }

        if is_valid {
            total += 1;
        }
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_simple_case() {
        assert!(valid_count("aa bb cc") == 1);
    }

    #[test]
    fn handles_multiple_simple_cases() {
        assert!(valid_count("aa bb cc\nbb cc dd\na b c") == 3);
    }

    #[test]
    fn handles_invalid_passphrase() {
        assert!(valid_count("aa bb aa") == 0);
    }

    #[test]
    fn handles_multiple_including_invalid() {
        assert!(valid_count("aa bb\n aaa a aa a") == 1);
    }
}
