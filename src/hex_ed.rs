// Problem Location: http://adventofcode.com/2017/day/11
use std::ops::Add;
use std::option::Option;

pub fn path_reduction(path : &str) -> Result<u32, ParseHexDirectionError> {
    let path : Vec<HexDirection> = path.split(',')
        .map(|s| parse_hex_direction(s))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(reduce_path_coordinate(path))
}

// Switch to sciyoshi's solution after doing path reduction didn't pan out
// https://github.com/sciyoshi/advent-of-rust-2017/blob/master/src/day11/mod.rs
#[derive(Debug, Copy, Clone)]
struct Pt(i32, i32);

impl Add for Pt {
	type Output = Self;

	fn add(self, other: Pt) -> Pt {
		Pt(self.0 + other.0, self.1 + other.1)
	}
}

fn dist(pt: Pt) -> u32 {
	((pt.0.abs() + pt.1.abs() + (pt.0 + pt.1).abs()) / 2) as u32
}

fn reduce_path_coordinate(path : Vec<HexDirection>) -> u32 {
    if path.len() <= 0 {
        return 0;
    }

    let end = path.into_iter().fold(Pt(0, 0), |pos, dir| {
        pos + match dir {
            HexDirection::N => Pt(0, 1),
            HexDirection::NE => Pt(1, 0),
            HexDirection::SE => Pt(1, -1),
            HexDirection::S => Pt(0, -1),
            HexDirection::SW => Pt(-1, 0),
            HexDirection::NW => Pt(-1, 1),
        }
    });

    dist(end)
}

/*
fn reduce_path_combined(path : Vec<HexDirection>) -> u32 {
    let mut reduced_path : Vec<HexDirection> = Vec::new();

    for direction in path.iter() {
        let direction = *direction;
        let mut combined = false;

        for other_direction in reduced_path.iter_mut() {
            match combine(&direction, other_direction) {
                Some(combined_direction) => {
                    *other_direction = combined_direction;
                    combined = true;
                    break;
                },
                None => (),
            }
        }

        if !combined {
            reduced_path.push(direction);
        }
    }

    reduced_path.into_iter().filter(|d| *d != HexDirection::None).count() as u32
}

fn combine(direction : &HexDirection, other_direction : &HexDirection) -> Option<HexDirection> {
    match combine_one_way(direction, other_direction) {
        Some(combined_direction) => return Some(combined_direction),
        None => (),
    }

    combine_one_way(other_direction, direction)
}

fn combine_one_way(direction : &HexDirection, other_direction : &HexDirection) -> Option<HexDirection> {
    match (*direction, *other_direction) {
        (HexDirection::NW, HexDirection::SE) => Some(HexDirection::None),
        (HexDirection::NE, HexDirection::SW) => Some(HexDirection::None),
        (HexDirection::S, HexDirection::N) => Some(HexDirection::None),

        (HexDirection::SW, HexDirection::SE) => Some(HexDirection::S),
        (HexDirection::NW, HexDirection::NE) => Some(HexDirection::N),

        (HexDirection::NE, HexDirection::S) => Some(HexDirection::SE),
        (HexDirection::NW, HexDirection::S) => Some(HexDirection::SW),

        (HexDirection::SE, HexDirection::N) => Some(HexDirection::NE),
        (HexDirection::SW, HexDirection::N) => Some(HexDirection::NW),

        _ => None,
    }
}
*/

use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseHexDirectionError;

impl fmt::Display for ParseHexDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse item into HexDirection!")
    }
}

impl error::Error for ParseHexDirectionError {
    fn description(&self) -> &str {
        "Unable to parse item into HexDirection!"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum HexDirection {
    // None,
    NW,
    N,
    NE,
    SW,
    S,
    SE,
}

fn parse_hex_direction(s: &str) -> Result<HexDirection, ParseHexDirectionError> {
    match s {
        "NW" | "nw" => Ok(HexDirection::NW),
        "N" | "n" => Ok(HexDirection::N),
        "NE" | "ne" => Ok(HexDirection::NE),
        "SW" | "sw" => Ok(HexDirection::SW),
        "S" | "s" => Ok(HexDirection::S),
        "SE" | "se" => Ok(HexDirection::SE),
        _ => Err(ParseHexDirectionError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hexes look like:
    //  \ n  /
    // nw +--+ ne
    //   /    \
    // -+      +-
    //   \    /
    // sw +--+ se
    //   / s  \
    #[test]
    fn one_direction_path() {
        assert_eq!(path_reduction("ne,ne,ne"), Ok(3));
    }

    #[test]
    fn backtracking_path() {
        assert_eq!(path_reduction("ne,ne,sw,sw"), Ok(0));
    }

    #[test]
    fn sw_se_becomes_s() {
        assert_eq!(path_reduction("sw,se,n"), Ok(0));
    }

    #[test]
    fn sideways_and_back() {
        assert_eq!(path_reduction("ne,se,sw,nw"), Ok(0));
    }

    #[test]
    fn n_se_sw() {
        assert_eq!(path_reduction("n,se,sw"), Ok(0));
    }

    #[test]
    fn ne_se_n_sw_sw() {
        assert_eq!(path_reduction("ne,se,n,sw,sw"), Ok(0));
    }

    #[test]
    fn combined_tests() {
        assert_eq!(path_reduction("ne,se,n,sw,sw,n,se,sw,ne,se,sw,nw,sw,se,n"), Ok(0));
    }

    #[test]
    fn semi_backtracking_path_1() {
        assert_eq!(path_reduction("ne,ne,s,s"), Ok(2));
    }

    #[test]
    fn semi_backtracking_path_1_1() {
        assert_eq!(path_reduction("ne,ne,s,s,nw,nw"), Ok(0));
    }

    #[test]
    fn semi_backtracking_path_2() {
        assert_eq!(path_reduction("se,sw,se,sw,sw"), Ok(3));
    }
}
