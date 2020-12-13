use std::collections::BTreeSet;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .flatten()
        .collect()
}

fn calc_valid_numbers(input: &Vec<u64>, start: usize, preamble_length: usize) -> BTreeSet<u64> {
    let mut valid_numbers: BTreeSet<u64> = BTreeSet::new();
    for current in start..(preamble_length + start) {
        let mut it = input.iter().skip(current).take(preamble_length);
        while let Some(c) = it.next() {
            for i in it.clone() {
                valid_numbers.insert(c + *i);
            }
        }
    }

    valid_numbers
}

fn find_invalid_number(input: &Vec<u64>, preamble_length: usize) -> u64 {
    input
        .iter()
        .skip(preamble_length)
        .enumerate()
        .filter(|i| !calc_valid_numbers(input, i.0, preamble_length).contains(i.1))
        .next()
        .map(|s| *s.1)
        .unwrap()
}

#[aoc(day9, part1, iterators)]
fn day9_part1(input: &Vec<u64>) -> u64 {
    find_invalid_number(input, 25)
}

fn find_encryption_weakness(input: &Vec<u64>, preamble_length: usize) -> u64 {
    let target = find_invalid_number(input, preamble_length);

    let mut start = 0usize;
    let mut range = start + 1;

    loop {
        let range_iter = input.iter().skip(start).take(range);
        let sum: u64 = range_iter.clone().sum();
        if sum < target {
            range += 1;
        } else if sum > target {
            start += 1;
            range = 1;
        } else {
            let smallest = range_iter.clone().min().unwrap();
            let largest = range_iter.max().unwrap();
            return smallest + largest;
        }
    }
}

#[aoc(day9, part2)]
fn day9_part2(input: &Vec<u64>) -> u64 {
    find_encryption_weakness(input, 25)
}

#[cfg(test)]
mod tests {
    use super::{
        calc_valid_numbers, find_encryption_weakness, find_invalid_number, input_generator,
    };

    #[test]
    fn test_calc_valid_numbers() {
        let input = vec![1, 2, 3, 4, 5];
        let res = calc_valid_numbers(&input, 0, 5);
        assert!(res.contains(&3));
        assert!(!res.contains(&2));
        assert!(!res.contains(&1));
        assert!(!res.contains(&10));
        assert!(res.contains(&9));
    }

    #[test]
    fn test_given_part_1() {
        let numbers = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let input = input_generator(numbers);
        let res = find_invalid_number(&input, 5);
        assert_eq!(res, 127)
    }

    #[test]
    fn test_given_part_2() {
        let numbers = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let input: Vec<u64> = input_generator(numbers);
        let res = find_encryption_weakness(&input, 5);
        assert_eq!(res, 62);
    }
}
