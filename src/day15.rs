use std::collections::HashMap;
#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn find_nth_number(input: &[u64], n: usize) -> u64 {
    let mut numbers: HashMap<usize, usize> = HashMap::new();
    let mut spoken = 0;

    for (index, value) in input.iter().enumerate() {
        let last_occurence = numbers.get(&(*value as usize));
        if let Some(i) = last_occurence {
            spoken = index - i;
        } else {
            spoken = 0;
        }
        numbers.insert(*value as usize, index + 1);
    }

    for index in (input.len() + 1)..n {
        let last_occurence = numbers.get(&spoken);
        let t = spoken;
        if let Some(i) = last_occurence {
            spoken = index - i;
        } else {
            spoken = 0;
        }
        numbers.insert(t, index);
    }

    spoken as u64
}

fn find_nth_array(input: &[u64], n: usize) -> u64 {
    let mut numbers = Vec::<usize>::with_capacity(n);
    let mut spoken = 0;
    numbers.resize(n, 0);

    for (index, value) in input.iter().enumerate() {
        let last_occurence = numbers[*value as usize];
        if last_occurence != 0 {
            spoken = index - last_occurence;
        } else {
            spoken = 0;
        }
        numbers[*value as usize] = index + 1;
    }

    for index in (input.len() + 1)..n {
        let last_occurence = numbers[spoken];
        let t = spoken;
        if last_occurence != 0 {
            spoken = index - last_occurence;
        } else {
            spoken = 0;
        }

        numbers[t] = index;
    }

    spoken as u64
}

#[aoc(day15, part1)]
fn day15_part1(input: &[u64]) -> u64 {
    find_nth_array(input, 2020)
}
#[aoc(day15, part2)]
fn day15_part2(input: &[u64]) -> u64 {
    find_nth_array(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::{find_nth_array, find_nth_number};

    #[test]
    fn test_given_day_15() {
        let res = find_nth_array(&[0, 3, 6], 10);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_1_ex_1() {
        let res = find_nth_number(&[1, 3, 2], 2020);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_part_1_correct() {
        let res = find_nth_number(&[0, 1, 4, 13, 15, 12, 16], 2020);
        assert_eq!(res, 1665);
    }
}
