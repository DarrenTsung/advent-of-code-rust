// Problem Location: http://adventofcode.com/2017/day/3
use nalgebra::Vector2;
pub type IntVector2 = Vector2<i32>;

use std::cmp::*;

pub fn solve(data_index : u32) -> u32 {
    if data_index <= 1 {
        return 0;
    }

    let mut spiral_start_position = IntVector2 { x: 1, y: 0 };
    let mut spiral_start_index = 2;

    let mut side_count = 1;
    loop {
        let count_in_spiral = (side_count * 4) + 4;
        let new_spiral_start_index = spiral_start_index + count_in_spiral;

        if data_index < new_spiral_start_index {
            let mut indexes_off = data_index - spiral_start_index;
            let mut data_index_position = spiral_start_position;
            let side_stride_length = side_count + 1;

            // Stride in a spiral counter-clockwise
            const STRIDE_DIRECTIONS : [IntVector2; 4] = [IntVector2 { x: 0, y: 1 }, IntVector2 { x: -1, y: 0 }, IntVector2 { x: 0, y: -1 }, IntVector2 { x: 1, y: 0 }];
            for direction in STRIDE_DIRECTIONS.iter() {
                // stride_amount becomes 0 after we've walked to the index
                let stride_amount = min(max(indexes_off, 0), side_stride_length) as i32;
                data_index_position += direction * IntVector2 { x: stride_amount, y: stride_amount };
                indexes_off -= if side_stride_length > indexes_off { indexes_off } else { side_stride_length }
            }

            return manhatten_distance(data_index_position);
        }

        spiral_start_position += IntVector2 { x: 1, y: -1 };
        spiral_start_index = new_spiral_start_index;
        side_count += 2;
    }
}

fn manhatten_distance(position: IntVector2) -> u32 {
    (position.x.abs() + position.y.abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spiral looks like this:
    // 17  16  15  14  13
    // 18   5   4   3  12
    // 19   6   1   2  11
    // 20   7   8   9  10
    // 21  22  23---> ...
    #[test]
    fn handles_basic_case() {
        assert!(solve(1) == 0);
    }

    #[test]
    fn handles_12_case() {
        assert!(solve(12) == 3);
    }

    #[test]
    fn handles_23_case() {
        assert!(solve(23) == 2);
    }

    #[test]
    fn handles_1024_case() {
        assert!(solve(1024) == 31);
    }
}