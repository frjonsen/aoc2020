use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::vec::Vec;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>[a-z]):\s(?P<password>\w+)$").unwrap();
}

#[derive(Debug)]
pub struct Password {
    min_occurence: u32,
    max_occurence: u32,
    letter: u8,
    password: String,
}

pub fn parse_line(row: Captures) -> Password {
    let min = row
        .name("min")
        .map(|m| m.as_str().parse().unwrap())
        .expect("No min in capture");
    let max = row
        .name("max")
        .map(|m| m.as_str().parse().unwrap())
        .expect("No max in capture");
    let letter = row
        .name("letter")
        .map(|m| m.as_str().chars().next().unwrap() as u8)
        .expect("No letter in capture");
    let password = row
        .name("password")
        .map(|m| m.as_str().to_owned())
        .expect("No password in capture");

    Password {
        min_occurence: min,
        max_occurence: max,
        letter,
        password,
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .into_iter()
        .map(|r| RE.captures(r).unwrap())
        .map(|r| parse_line(r))
        .collect()
}

#[aoc(day2, part1)]
pub fn day2_part1(input: &Vec<Password>) -> usize {
    input
        .into_iter()
        .filter(|p| {
            let occurences = p.password.matches(p.letter as char).count() as u32;
            occurences >= p.min_occurence && occurences <= p.max_occurence
        })
        .count()
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &Vec<Password>) -> usize {
    input
        .into_iter()
        .filter(|p| {
            let first = if p
                .password
                .as_bytes()
                .get((p.min_occurence - 1) as usize)
                .unwrap()
                == &p.letter
            {
                1
            } else {
                0
            };
            let second = if p
                .password
                .as_bytes()
                .get((p.max_occurence - 1) as usize)
                .unwrap()
                == &p.letter
            {
                1
            } else {
                0
            };
            first + second == 1
        })
        .count()
}
