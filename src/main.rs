extern crate nalgebra;
extern crate regex;
#[macro_use]
extern crate simple_error;

mod inverse_captcha;
mod corrupted_checksum;
mod spiral_memory;
mod high_entropy_passphrases;
mod twisty_trampoline_maze;
mod memory_reallocation;
mod recursive_circus;
mod registers;
mod stream_processing;

use std::fs::File;
use std::io::Read;
use std::fmt::Display;

fn main() {
    let inverse_captcha_input = read_file("data/inverse_captcha_input.txt");
    unwrap_day_result(1, inverse_captcha::solve(&inverse_captcha_input));

    let corrupted_checksum_input = read_file("data/corrupted_checksum_input.txt");
    unwrap_day_result(2, corrupted_checksum::solve(&corrupted_checksum_input));

    println!("Day 3: {}", spiral_memory::solve(368078));

    let high_entropy_passphrases_input = read_file("data/high_entropy_passphrases_input.txt");
    println!("Day 4: {}", high_entropy_passphrases::valid_count(&high_entropy_passphrases_input));

    let twisty_trampoline_maze_input = read_file("data/twisty_trampoline_maze_input.txt");
    unwrap_day_result(5, twisty_trampoline_maze::solve(&twisty_trampoline_maze_input));

    let memory_reallocation_input = read_file("data/memory_reallocation_input.txt");
    unwrap_day_result(6, memory_reallocation::solve(&memory_reallocation_input));

    let recursive_circus_input = read_file("data/recursive_circus_input.txt");
    println!("Day 7: {}", recursive_circus::solve(&recursive_circus_input).unwrap_or("No Solution"));

    let registers_input = read_file("data/registers_input.txt");
    unwrap_day_result(8, registers::run_instructions(&registers_input));

    let stream_processing_input = read_file("data/stream_processing_input.txt");
    println!("Day 9: {}", stream_processing::count_group_score(&stream_processing_input));
}

fn unwrap_day_result<T, TError>(day : u32, result : Result<T, TError>)
    where T : Display,
          TError : Display
{
    match result {
        Ok(solution) => println!("Day {}: {}", day, solution),
        Err(e) => println!("Day {}: Error {}!", day, e),
    }
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect(&format!("File not found: {}!", filename));

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect(&format!("File could not be read: {}!", filename));

    return file_contents;
}