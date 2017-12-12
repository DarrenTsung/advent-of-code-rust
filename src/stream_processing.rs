// Problem Location: http://adventofcode.com/2017/day/9
pub fn count_group_score(input : &str) -> u32 {
    let mut score = 0;
    let mut depth = 0;

    let mut in_garbage = false;

    let mut chars_iter = input.chars();
    while let Some(c) = chars_iter.next() {
        if c == '!' {
            // skip next character
            chars_iter.next();
            continue;
        }

        if in_garbage && c == '>' {
            in_garbage = false;
        }

        if !in_garbage && c == '<' {
            in_garbage = true;
        }

        if in_garbage {
            continue;
        }

        match c {
            '{' => {
                depth += 1;
            },
            '}' => {
                score += depth;
                depth -= 1;
            }
            _ => (),
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_basic_case() {
        assert_eq!(count_group_score("{}"), 1);
    }

    #[test]
    fn handles_garbage() {
        assert_eq!(count_group_score("{<}>"), 0);
    }

    #[test]
    fn handles_cancelled_garbage() {
        assert_eq!(count_group_score("{!<}>"), 1);
    }

    #[test]
    fn handles_nested_groups() {
        assert_eq!(count_group_score("{{}}"), 3); // 1 + 2
    }

    #[test]
    fn handles_nested_groups_2() {
        assert_eq!(count_group_score("{{},{}}"), 5); // 1 + 2 + 2
    }

    #[test]
    fn handles_nested_groups_3() {
        assert_eq!(count_group_score("{{<ab>},{<ab>}}"), 5); // 1 + 2 + 2
    }

    #[test]
    fn handles_complex_1() {
        assert_eq!(count_group_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }

    #[test]
    fn handles_complex_2() {
        assert_eq!(count_group_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }

    #[test]
    fn handles_complex_3() {
        assert_eq!(count_group_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}