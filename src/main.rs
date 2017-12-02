mod inverse_captcha;

use std::fs::File;
use std::io::Read;

fn main() {
    let inverse_captcha_input = read_file("data/inverse_captcha_input.txt");
    match inverse_captcha::solve(&inverse_captcha_input) {
        Some(solution) => println!("Day 1: {}", solution),
        None => println!("Day 1: Invalid input {}!", &inverse_captcha_input),
    }
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect(&format!("File not found: {}!", filename));

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect(&format!("File could not be read: {}!", filename));

    return file_contents;
}