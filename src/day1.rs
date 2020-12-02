use std::{unimplemented, vec::Vec};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &Vec<u32>) -> u32 {
    for (i_x, x) in input.iter().enumerate() {
        for y in input.iter().skip(i_x) {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unimplemented!();
}

#[aoc(day1, part2)]
pub fn part1_vec(input: &Vec<u32>) -> u32 {
    for (i_x, x) in input.iter().enumerate() {
        for (i_y, y) in input.iter().skip(i_x).enumerate() {
            for z in input.iter().skip(i_y) {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unimplemented!();
}
