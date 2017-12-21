// Problem Location: http://adventofcode.com/2017/day/12
use std::collections::HashMap;
use std::num::ParseIntError;
use std::result::Result;

pub fn calculate_severity(security_input : &str) -> Result<u32, ParseIntError> {
    let security = parse_security_input(security_input)?;
    Ok(calculate_severity_parsed(&security))
}

fn parse_security_input(security_input : &str) -> Result<HashMap<u32, u32>, ParseIntError> {
    let mut security = HashMap::new();

    for line in security_input.lines() {
        let tokens = line.split(": ").map(|t| t.parse::<u32>()).collect::<Result<Vec<_>, _>>()?;
        security.insert(tokens[0], tokens[1]);
    }

    Ok(security)
}

fn calculate_severity_parsed(security : &HashMap<u32, u32>) -> u32 {
    let mut severity = 0;

    for (guard_depth, patrol_range) in security {
        let guard_position = guard_position_at_time(*patrol_range, *guard_depth);
        if guard_position == 0 {
            severity += guard_depth * patrol_range;
        }
    }

    severity
}

fn guard_position_at_time(patrol_range : u32, time : u32) -> u32 {
    let repeat_range = (patrol_range * 2) - 2;
    if repeat_range == 0 {
        return 0;
    }

    let mod_time = time % repeat_range;

    if mod_time < patrol_range {
        mod_time
    } else {
        ((patrol_range - 1) * 2) - mod_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example() {
        let given_input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(calculate_severity(given_input), Ok(24));
    }

    #[test]
    fn catches_on_0_but_no_severity() {
        let mut security = HashMap::new();
        security.insert(0, 2);

        assert_eq!(calculate_severity_parsed(&security), 0);
    }

    #[test]
    fn catch_on_1() {
        let mut security = HashMap::new();
        security.insert(1, 1);

        assert_eq!(calculate_severity_parsed(&security), 1);
    }

    #[test]
    fn no_catch_on_1() {
        let mut security = HashMap::new();
        security.insert(1, 2);

        assert_eq!(calculate_severity_parsed(&security), 0);
    }

    #[test]
    fn guard_position_t0() {
        assert_eq!(guard_position_at_time(3, 0), 0);
    }

    #[test]
    fn guard_position_t1() {
        assert_eq!(guard_position_at_time(3, 1), 1);
    }

    #[test]
    fn guard_position_t2() {
        assert_eq!(guard_position_at_time(3, 2), 2);
    }

    #[test]
    fn guard_position_t3() {
        assert_eq!(guard_position_at_time(3, 3), 1);
    }

    #[test]
    fn guard_position_t4() {
        assert_eq!(guard_position_at_time(3, 4), 0);
    }
}
