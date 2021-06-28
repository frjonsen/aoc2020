#[derive(Eq, PartialEq, Debug)]
enum Entry {
    Unknown,
    Bus(u64),
}

struct Input {
    timestamp: u64,
    busses: Vec<Entry>,
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().map(&str::trim).collect();
    let timestamp = lines.get(0).unwrap();
    let busses = lines
        .get(1)
        .unwrap()
        .split(',')
        .map(|b| match b {
            "x" => Entry::Unknown,
            _ => Entry::Bus(b.parse().unwrap()),
        })
        .collect();
    Input {
        timestamp: timestamp.parse().unwrap(),
        busses,
    }
}

#[aoc(day13, part1)]
fn day13_part1(input: &Input) -> u64 {
    let busses = input.busses.iter().filter_map(|f| match f {
        Entry::Bus(t) => Some(t),
        _ => None,
    });

    let next_departure = busses
        .into_iter()
        .map(|b| (b, b * (input.timestamp / b) + b))
        .min_by_key(|b| b.1)
        .unwrap();

    (next_departure.1 - input.timestamp) * next_departure.0
}

//#[aoc(day13, part2)]
fn day13_part2(input: &Input) -> u64 {
    let busses: Vec<_> = input
        .busses
        .iter()
        .enumerate()
        .filter_map(|f| match f.1 {
            Entry::Bus(t) => Some((f.0 as u64, *t)),
            _ => None,
        })
        .collect();

    let first = busses.first().unwrap();
    let busses: Vec<_> = busses.iter().skip(1).collect();

    for min in (first.1..).step_by(first.1 as usize) {
        if busses.iter().all(|q| (min + q.0) % q.1 == 0) {
            return min;
        }
    }

    panic!("Didn't find it")
}

#[cfg(test)]
mod tests {
    use super::{day13_part1, day13_part2, input_generator};

    #[test]
    fn test_given_part_1() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let generated = input_generator(input);
        let res = day13_part1(&generated);
        assert_eq!(res, 295);
    }

    #[test]
    fn test_given_part_2() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 1068781);
    }

    #[test]
    fn test_given_part_2_sample_2() {
        let input = "939
        17,x,13,19";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 3417);
    }

    #[test]
    fn test_given_part_2_sample_3() {
        let input = "939
        67,7,59,61";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 754018);
    }
    #[test]
    fn test_given_part_2_sample_4() {
        let input = "939
        67,x,7,59,61";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 779210);
    }
    #[test]
    fn test_given_part_2_sample_5() {
        let input = "939
        67,7,x,59,61";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 1261476);
    }
    #[test]
    fn test_given_part_2_sample_6() {
        let input = "939
        1789,37,47,1889";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, 1202161486);
    }
}
