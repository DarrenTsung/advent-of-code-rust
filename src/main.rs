extern crate nalgebra;

mod inverse_captcha;
mod corrupted_checksum;
mod spiral_memory;
mod high_entropy_passphrases;

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
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect(&format!("File not found: {}!", filename));

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect(&format!("File could not be read: {}!", filename));

    return file_contents;
}