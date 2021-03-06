#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate nalgebra;
extern crate regex;
#[macro_use]
extern crate simple_error;

pub mod inverse_captcha;
pub mod corrupted_checksum;
pub mod spiral_memory;
pub mod high_entropy_passphrases;
pub mod twisty_trampoline_maze;
pub mod memory_reallocation;
pub mod recursive_circus;
pub mod registers;
pub mod stream_processing;
pub mod knot_hash;
pub mod hex_ed;
pub mod digital_plumber;
pub mod packet_scanners;
