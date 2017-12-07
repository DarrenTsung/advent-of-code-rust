// Problem Location: http://adventofcode.com/2017/day/7
use regex::Regex;
use std::collections::HashSet;
use std::option::Option;

pub fn solve(tower_input : &str) -> Option<&str> {
    let split_regex = Regex::new(r"[\s\d()->,]+").unwrap();

    let mut token_set : HashSet<&str> = HashSet::new();
    let mut child_set : HashSet<&str> = HashSet::new();

    for line in tower_input.lines() {
        for (index, token) in split_regex.split(line).enumerate() {
            token_set.insert(token);

            if index > 0 {
                child_set.insert(token);
            }
        }
    }

    token_set.into_iter().find(|t| !child_set.contains(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_basic_case() {
        assert_eq!(solve("pbga (66)\n\
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"), Some("tknk"));
    }
}