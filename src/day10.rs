#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = input
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
fn day10_part1(input: &Vec<u32>) -> u32 {
    let mut frequency = vec![0, 0, 1];
    input
        .windows(2)
        .map(|s| s[1] - s[0])
        .for_each(|d| frequency[(d - 1) as usize] += 1);

    frequency[0] * frequency[2]
}

#[cfg(test)]
mod tests {
    use super::{day10_part1, input_generator};

    #[test]
    fn test_given_part_1() {
        let input = "28
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
        let numbers = input_generator(input);
        let res = day10_part1(&numbers);
        assert_eq!(220, res);
    }
}
