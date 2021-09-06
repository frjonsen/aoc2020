#[derive(Eq, PartialEq, Debug)]
enum Entry {
    Unknown,
    Bus(u64),
}

struct Input {
    timestamp: u64,
    busses: Vec<Entry>,
}

struct BezoutIdentity {
    _s: i64,
    t: i64,
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

#[aoc(day13, part2)]
fn day13_part2(input: &Input) -> i64 {
    let busses: Vec<_> = input
        .busses
        .iter()
        .enumerate()
        .filter_map(|f| match f.1 {
            Entry::Bus(t) => Some((((*t as i64) - (f.0 as i64)).rem_euclid(*t as i64), *t)),
            _ => None,
        })
        .collect();

    let n: Vec<i64> = busses.iter().map(|b| b.1 as i64).collect();
    let a: Vec<i64> = busses.iter().map(|b| b.0).collect();
    chinese_remainder_theorem(&n, &a)
}

fn extended_euclidean(prod: i64, n: i64) -> BezoutIdentity {
    let mut t0 = 0;
    let mut t1 = 1;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut r2 = prod;
    let mut r1 = n;

    while r1 != 0 {
        let q1 = r2 / r1;
        let r2_temp = r2;
        r2 = r1;
        r1 = r2_temp - q1 * r1;

        let t_temp = t0 - q1 * t1;
        t0 = t1;
        t1 = t_temp;

        let s_temp = s0 - q1 * s1;
        s0 = s1;
        s1 = s_temp;
    }

    BezoutIdentity { _s: s0, t: t0 }
}

fn chinese_remainder_theorem(n: &[i64], a: &[i64]) -> i64 {
    let prod: i64 = n.iter().product();
    n.iter()
        .zip(a)
        .map(|(n, a)| {
            let p = prod / *n;
            let b = extended_euclidean(*n, p);
            let res = p * b.t * a;
            res.rem_euclid(prod)
        })
        .sum::<i64>()
        .rem_euclid(prod)
}

#[cfg(test)]
mod tests {
    use super::{chinese_remainder_theorem, day13_part1, day13_part2, input_generator};

    #[test]
    fn test_given_part_1() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let generated = input_generator(input);
        let res = day13_part1(&generated);
        assert_eq!(res, 295);
    }

    #[test]
    fn test_given_part_2_sample_1() {
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

    #[test]
    fn test_correct() {
        let expected = 702970661767766;
        let input = "1000495
        19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,521,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,523,x,x,x,x,x,37,x,x,x,x,x,x,13";
        let generated = input_generator(input);
        let res = day13_part2(&generated);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_theorem() {
        let res = chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 2]);
        assert_eq!(res, 23);
    }

    #[test]
    fn test_theorem_2() {
        let res = chinese_remainder_theorem(&[5, 7, 12], &[0, 6, 10]);
        assert_eq!(res, 370);
    }

    #[test]
    fn test_theorem_3() {
        let res = chinese_remainder_theorem(&[13, 5, 7], &[2, 3, 2]);
        assert_eq!(res, 93);
    }

    #[test]
    fn test_theorem_4() {
        let res = chinese_remainder_theorem(&[13, 5, 29], &[2, 5, 7]);
        assert_eq!(res, 210);
    }
    #[test]
    fn test_theorem_5() {
        let res = chinese_remainder_theorem(&[13, 5, 29], &[24, 5, 7]);
        assert_eq!(res, 1805);
    }
}
