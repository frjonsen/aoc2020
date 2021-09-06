use std::collections::BTreeSet;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn search(
    partitioning: &[char],
    lowest: u32,
    highest: u32,
    bottom_char: char,
    upper_char: char,
) -> u32 {
    let mut min = lowest;
    let mut _max = highest;
    let mut sub = highest + 1;

    for c in partitioning {
        sub /= 2;
        match c {
            _ if *c == bottom_char => {
                _max -= sub;
            }
            _ if *c == upper_char => {
                min += sub;
            }
            _ => panic!("Unexpected character {}", c),
        }
    }
    min
}

pub fn calculate_seat_id(partitioning: &Vec<char>) -> u32 {
    let parts = partitioning.split_at(7);
    let row = search(parts.0, 0, 127, 'F', 'B') * 8;
    let column = search(parts.1, 0, 7, 'L', 'R');

    row + column
}

#[aoc(day5, part1)]
pub fn day5_part1(input: &Vec<Vec<char>>) -> u32 {
    input.into_iter().map(calculate_seat_id).max().unwrap()
}

#[aoc(day5, part2)]
pub fn day5_part2(input: &Vec<Vec<char>>) -> u32 {
    let known_seat_ids: BTreeSet<u32> = input.into_iter().map(calculate_seat_id).collect();
    let highest = *known_seat_ids.iter().max().unwrap();
    let min = *known_seat_ids.iter().min().unwrap();

    for id in min..highest {
        if !known_seat_ids.contains(&id) {
            return id;
        }
    }

    panic!("Not found");
}

#[cfg(test)]
mod tests {
    use super::{calculate_seat_id, day5_part1, search};

    #[test]
    fn test_given_row() {
        let input = vec!['F', 'B', 'F', 'B', 'B', 'F', 'F'];
        let res = search(&input, 0, 127, 'F', 'B');
        assert_eq!(res, 44);
    }

    #[test]
    fn test_given_column() {
        let input = vec!['R', 'L', 'R'];
        let res = search(&input, 0, 7, 'L', 'R');
        assert_eq!(res, 5);
    }

    #[test]
    fn test_calculate_seat_id() {
        let input = vec!['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'];
        let res = calculate_seat_id(&input);
        assert_eq!(res, 357);
    }

    #[test]
    fn test_day5_part1() {
        let input = vec![
            "BFFFBBFRRR".chars().collect(),
            "FFFBBBFRRR".chars().collect(),
            "BBFFBBFRLL".chars().collect(),
        ];
        let res = day5_part1(&input);
        assert_eq!(res, 820)
    }
}
