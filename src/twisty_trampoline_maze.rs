// Problem Location: http://adventofcode.com/2017/day/5
pub fn solve(maze_structure : &str) -> u32 {
    let mut maze_tokens : Vec<i32> = maze_structure.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut step_count = 0;
    let mut index : i32 = 0;

    let maze_length = maze_tokens.len();

    loop {
        // NOTE (darren): is conversion to usize correct here?
        // it seems good because it's only valid due to context
        let next_index = index + maze_tokens[index as usize];
        if next_index < 0 || next_index >= maze_length as i32 {
            return step_count + 1;
        }

        maze_tokens[index as usize] += 1;
        index = next_index;
        step_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_basic_case() {
        assert_eq!(solve("0\n3\n0\n1\n-3"), 5);
    }

    #[test]
    fn handles_exit_through_negatives() {
        assert_eq!(solve("0\n-5\n1\n2\n3"), 3);
    }
}