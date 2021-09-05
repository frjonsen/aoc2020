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
    let all_rules: Vec<_> = input
        .rules
        .iter()
        .map(|r| [&r.interval_one, &r.interval_two])
        .flatten()
        .collect();

    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|v| !all_rules.iter().any(|p| p.contains(v)))
        .sum()
}

#[aoc(day16, part2)]
fn day16_part2(input: &Input) -> u64 {
    let all_rules: Vec<_> = input
        .rules
        .iter()
        .map(|r| [&r.interval_one, &r.interval_two])
        .flatten()
        .collect();

    let valid_tickets: Vec<&Vec<u32>> = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|v| all_rules.iter().any(|r| r.contains(v)))
        })
        .collect();

    let slot_count = valid_tickets.first().unwrap().len();

    let mut valid_for_slot = vec![input.rules.iter().collect::<Vec<&Rule>>(); slot_count];

    for ticket in &valid_tickets {
        for i in 0..slot_count {
            let still_valid = valid_for_slot
                .get(i)
                .unwrap()
                .iter()
                .filter(|r| {
                    r.interval_one.contains(&ticket[i]) || r.interval_two.contains(&ticket[i])
                })
                .copied()
                .collect::<Vec<_>>();
            if still_valid.len() == 1 {
                let name = &still_valid.first().unwrap().name;
                filter_determined(&mut valid_for_slot, name);
            }
            valid_for_slot[i] = still_valid;
        }
    }
    for ticket in &valid_tickets {
        for i in 0..slot_count {
            let still_valid = valid_for_slot
                .get(i)
                .unwrap()
                .iter()
                .filter(|r| {
                    r.interval_one.contains(&ticket[i]) || r.interval_two.contains(&ticket[i])
                })
                .copied()
                .collect::<Vec<_>>();
            if still_valid.len() == 1 {
                let name = &still_valid.first().unwrap().name;
                filter_determined(&mut valid_for_slot, name);
            }
            valid_for_slot[i] = still_valid;
        }
    }

    let departure_slots: Vec<usize> = valid_for_slot
        .iter()
        .map(|v| v.first())
        .map(Option::unwrap)
        .copied()
        .enumerate()
        .filter(|r| r.1.name.starts_with("departure"))
        .map(|r| r.0)
        .collect();

    departure_slots
        .iter()
        .map(|r| input.personal_ticket.get(*r))
        .map(Option::unwrap)
        .map(|r| *r as u64)
        .product()
}

fn filter_determined(rules: &mut Vec<Vec<&Rule>>, name: &str) {
    for slot_rules in rules.iter_mut() {
        if slot_rules.len() == 1 {
            continue;
        }
        slot_rules.retain(|r| r.name != name);
    }
}

#[cfg(test)]
mod tests {
    use super::{day16_part1, input_generator};

    #[test]
    fn test_given_day16() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";

        let generated = input_generator(&input);
        let res = day16_part1(&generated);
        assert_eq!(res, 71);
    }
}
