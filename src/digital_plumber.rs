// Problem Location: http://adventofcode.com/2017/day/12
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::result::Result;

pub fn count_connected(graph_input : &str) -> Result<u32, Box<Error>> {
    let graph = parse_graph_input(graph_input)?;
    Ok(count_connected_parsed(graph))
}

fn parse_graph_input(graph_input : &str) -> Result<HashMap<u32, Vec<u32>>, Box<Error>> {
    let split_regex = Regex::new(r"[\s<\->,]+").unwrap();

    let mut graph = HashMap::new();
    for line in graph_input.lines() {
        let mut key = 0;
        let mut values = Vec::new();

        for (index, token) in split_regex.split(line).enumerate() {
            let token = token.parse::<u32>()?;
            if index == 0 {
                key = token;
            } else {
                values.push(token);
            }
        }

        graph.insert(key, values);
    }

    Ok(graph)
}

fn count_connected_parsed(graph : HashMap<u32, Vec<u32>>) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(0);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        let connected_elements : &Vec<u32> = match graph.get(&current) {
            Some(s) => s,
            None => {
                // leaf node
                continue;
            },
        };

        for elem in connected_elements.iter() {
            queue.push_back(*elem);
        }
    }

    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_but_1() {
        let mut graph = HashMap::new();
        graph.insert(0, vec!(2));
        graph.insert(1, vec!(1));
        graph.insert(2, vec!(0, 3, 4));
        graph.insert(3, vec!(2, 4));
        graph.insert(4, vec!(2, 3, 6));
        graph.insert(5, vec!(6));
        graph.insert(6, vec!(4, 5));
        assert_eq!(count_connected_parsed(graph), 6);
    }

    #[test]
    fn parses_graph_correctly() {
        let mut graph = HashMap::new();
        graph.insert(0, vec!(2));
        graph.insert(1, vec!(1));
        graph.insert(2, vec!(0, 3, 4));
        graph.insert(3, vec!(2, 4));
        graph.insert(4, vec!(2, 3, 6));
        graph.insert(5, vec!(6));
        graph.insert(6, vec!(4, 5));

        assert_eq!(parse_graph_input("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5").unwrap(), graph);
    }
}
