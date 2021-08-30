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
    s: i64,
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

fn extended_euclidean(N: i64, n: i64) -> BezoutIdentity {
    let mut t0 = 0;
    let mut t1 = 1;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut r2 = N;
    let mut r1 = n;

    while r1 != 0 {
        let q1 = r2 / r1;
        let r2_temp = r2;
        r2 = r1;
        r1 = r2_temp - q1 * r1;

        let t_temp = t0 - q1 * t1;
        t0 = t1;
        t1 = t_temp % N;

        let s_temp = s0 - q1 * s1;
        s0 = s1;
        s1 = s_temp % N;
    }

    BezoutIdentity { s: s0, t: t0 }
}

fn chinese_remainder_theorem(n: &[i64], a: &[i64]) -> i64 {
    let N: i64 = n.iter().product();
    let x: i64 = n
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let b = extended_euclidean(*n, N / n);
            a[i] * b.t * N / n
        })
        .sum();
    x % N
}

#[cfg(test)]
mod tests {
    use super::{
        chinese_remainder_theorem, day13_part1, day13_part2, extended_euclidean, input_generator,
    };

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

    #[test]
    fn test_theorem() {
        let res = chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 2]);
        assert_eq!(res, 23);
    }
}
