// Problem Location: http://adventofcode.com/2017/day/2
use std::result::Result;

pub fn solve(spreadsheet: &str) -> Result<u32, &'static str> {
    let mut checksum = 0;

    for line in spreadsheet.lines() {
        let mut low = u32::max_value();
        let mut high = u32::min_value();

        for val_result in line.split_whitespace().map(|s| s.parse::<u32>()) {
            match val_result {
                Err(_e) => return Err("Failed to parse spreadsheet into u32's!"),
                Ok(val) => {
                    if val < low {
                        low = val;
                    }

                    if high < val {
                        high = val;
                    }
                }
            }
        }

        checksum += high - low;
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_basic_case() {
        assert!(solve("2 1 9\n3 2 4") == Ok(10));
    }

    #[test]
    fn handles_more_advanced_case() {
        assert!(solve("5 1 9 5\n 7 5 3\n 2 4 6 8") == Ok(18));
    }

    #[test]
    fn handles_invalid_input() {
        assert!(solve("5 1 9 5\n 7 --5 3\n 2 4 6 8").is_err());
    }
}