use std::collections::BTreeMap;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .flatten()
        .collect();
    numbers.sort_unstable();
    numbers.insert(0, 0);
    numbers
}

#[aoc(day10, part1, window)]
fn day10_part1(input: &Vec<i32>) -> u32 {
    let mut frequency = vec![0, 0, 1];
    input
        .windows(2)
        .map(|s| s[1] - s[0])
        .for_each(|d| frequency[(d - 1) as usize] += 1);

    frequency[0] * frequency[2]
}

#[aoc(day10, part2)]
fn day10_part2(input: &Vec<i32>) -> u64 {
    let mut paths: BTreeMap<i32, u64> = BTreeMap::new();
    paths.insert(0, 1);

    for i in input.iter().enumerate() {
        let c = i.0;
        let range = ((c as i64 - 3).max(0) as usize)..c;
        let past_three: u64 = input[range]
            .iter()
            .filter(|p| i.1 - *p <= 3)
            .map(|k| paths.get(k).unwrap())
            .sum();

        paths.insert(*i.1, past_three.max(1));
    }

    *paths.get(input.last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{day10_part1, day10_part2, input_generator};

    const INPUT1: &'static str = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    const INPUT2: &'static str = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

    #[test]
    fn test_given_part_1_1() {
        let numbers = input_generator(INPUT1);
        let res = day10_part1(&numbers);
        assert_eq!(35, res);
    }

    #[test]
    fn test_given_part_1_2() {
        let numbers = input_generator(INPUT2);
        let res = day10_part1(&numbers);
        assert_eq!(220, res);
    }
    #[test]
    fn test_given_part_2_1() {
        let numbers = input_generator(INPUT1);
        let res = day10_part2(&numbers);
        assert_eq!(8, res);
    }

    #[test]
    fn test_given_part_2_2() {
        let numbers = input_generator(INPUT2);
        let res = day10_part2(&numbers);
        assert_eq!(19208, res);
    }
}
