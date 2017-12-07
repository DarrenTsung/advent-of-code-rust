extern crate nalgebra;
extern crate regex;

mod inverse_captcha;
mod corrupted_checksum;
mod spiral_memory;
mod high_entropy_passphrases;
mod twisty_trampoline_maze;
mod memory_reallocation;
mod recursive_circus;

use std::fs::File;
use std::io::Read;

fn main() {
    let inverse_captcha_input = read_file("data/inverse_captcha_input.txt");
    match inverse_captcha::solve(&inverse_captcha_input) {
        Ok(solution) => println!("Day 1: {}", solution),
        Err(e) => println!("Day 1: Error {}!", e),
    }

    let corrupted_checksum_input = read_file("data/corrupted_checksum_input.txt");
    match corrupted_checksum::solve(&corrupted_checksum_input) {
        Ok(solution) => println!("Day 2: {}", solution),
        Err(e) => println!("Day 2: Error {}!", e),
    }

    println!("Day 3: {}", spiral_memory::solve(368078));

    let high_entropy_passphrases_input = read_file("data/high_entropy_passphrases_input.txt");
    println!("Day 4: {}", high_entropy_passphrases::valid_count(&high_entropy_passphrases_input));

    let twisty_trampoline_maze_input = read_file("data/twisty_trampoline_maze_input.txt");
    match twisty_trampoline_maze::solve(&twisty_trampoline_maze_input) {
        Ok(solution) => println!("Day 5: {}", solution),
        Err(e) => println!("Day 5: Error {}!", e),
    }

    let memory_reallocation_input = read_file("data/memory_reallocation_input.txt");
    match memory_reallocation::solve(&memory_reallocation_input) {
        Ok(solution) => println!("Day 6: {}", solution),
        Err(e) => println!("Day 6: Error {}!", e),
    }

    let recursive_circus_input = read_file("data/recursive_circus_input.txt");
    println!("Day 7: {}", recursive_circus::solve(&recursive_circus_input).unwrap_or("No Solution"));
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect(&format!("File not found: {}!", filename));

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect(&format!("File could not be read: {}!", filename));

    return file_contents;
}