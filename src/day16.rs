use itertools::Itertools;
use regex::{Match, Regex};
use std::ops::RangeInclusive;

struct Rule {
    name: String,
    interval_one: RangeInclusive<u32>,
    interval_two: RangeInclusive<u32>,
}

struct Input {
    rules: Vec<Rule>,
    personal_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn parse_range(m: Match) -> RangeInclusive<u32> {
    m.as_str()
        .split('-')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect_tuple()
        .map(|t: (u32, u32)| (t.0)..=(t.1))
        .unwrap()
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    let sections: Vec<_> = input.split("\n\n").collect();
    let rule_regex = Regex::new(r"(?P<name>.*): (?P<one>\d+-\d+) or (?P<two>\d+-\d+)").unwrap();

    let rules: Vec<_> = sections[0]
        .lines()
        .map(str::trim)
        .map(|l| rule_regex.captures(l))
        .map(Option::unwrap)
        .map(|c| {
            let name = c.name("name").unwrap().as_str().to_owned();
            let interval_one = c.name("one").map(parse_range).unwrap();
            let interval_two = c.name("two").map(parse_range).unwrap();
            Rule {
                name,
                interval_one,
                interval_two,
            }
        })
        .collect();
    let personal_ticket: Vec<_> = sections[1]
        .lines()
        .map(str::trim)
        .nth(1)
        .unwrap()
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    let nearby_tickets: Vec<Vec<u32>> = sections[2]
        .lines()
        .map(str::trim)
        .skip(1)
        .map(|t| {
            t.split(',')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<u32>>()
        })
        .collect();
    Input {
        rules,
        personal_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn day16_part1(input: &Input) -> u32 {
    5
}
