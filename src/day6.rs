use std::collections::HashSet;

use regex::Regex;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    let group_separator = Regex::new(r"\n\n+").unwrap();
    group_separator.split(input).map(|g| g.to_owned()).collect()
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|g| g.replace('\n', "").chars().collect::<HashSet<char>>().len() as u32)
        .sum()
}

#[aoc(day6, part2)]
pub fn day6_part2(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>()
        })
        .map(|p| {
            let first = p.first().unwrap().clone();
            p.into_iter()
                .skip(1)
                .fold(first, |set1, set2| &set1 & &set2)
                .len() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{day6_part1, day6_part2, input_generator};

    #[test]
    fn test_generator() {
        let given = concat!(
            "abc\n\n",
            "a\nb\nc\n\n",
            "ab\nac\n\n",
            "a\na\na\na\n\n",
            "b"
        );

        let res = input_generator(given);
        let groups = vec![
            "abc".to_owned(),
            "a\nb\nc".to_owned(),
            "ab\nac".to_owned(),
            "a\na\na\na".to_owned(),
            "b".to_owned(),
        ];

        assert_eq!(&res[..], &groups[..])
    }

    #[test]
    fn test_given_part1() {
        let groups = vec![
            "abc".to_owned(),
            "ab\nac".to_owned(),
            "ab\nac".to_owned(),
            "a\na\na\na\na".to_owned(),
            "b".to_owned(),
        ];
        let res = day6_part1(&groups);

        assert_eq!(res, 11)
    }

    #[test]
    fn test_given_part2() {
        let groups = vec![
            "abc".to_owned(),
            "a\nb\nc".to_owned(),
            "ab\nac".to_owned(),
            "a\na\na\na\na".to_owned(),
            "b".to_owned(),
        ];
        let res = day6_part2(&groups);

        assert_eq!(res, 6)
    }
}
