use std::collections::BTreeSet;
enum OpCode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn line_to_op_code(line: &str) -> OpCode {
    let parts = line.split(" ").collect::<Vec<_>>();
    let op_code = parts.get(0).expect("No op code in line");
    let val: i32 = parts
        .get(1)
        .map(|s| s.parse().unwrap())
        .expect("No value in line");

    match op_code {
        _ if *op_code == "nop" => OpCode::Nop(val),
        _ if *op_code == "acc" => OpCode::Acc(val),
        _ if *op_code == "jmp" => OpCode::Jmp(val),
        _ => panic!(format!("Unknown op code {}", op_code)),
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<OpCode> {
    input.lines().map(line_to_op_code).collect()
}

fn add_usize(a: usize, b: i32) -> usize {
    ((a as i32) + b) as usize
}

fn run_until_loop_or_exit(
    op_codes: &Vec<OpCode>,
    mut prev_visits: BTreeSet<usize>,
    have_branched: bool,
    mut p_counter: usize,
) -> Option<i32> {
    let mut acc = 0i32;

    let branch = |v, p: &BTreeSet<usize>, p_c| {
        run_until_loop_or_exit(op_codes, p.clone(), true, add_usize(p_c, v))
    };

    while prev_visits.insert(p_counter) {
        if let Some(current_code) = op_codes.get(p_counter) {
            match current_code {
                OpCode::Nop(f) => {
                    if !have_branched {
                        if let Some(v) = branch(*f, &prev_visits, p_counter) {
                            return Some(acc + v);
                        }
                    }
                    p_counter += 1;
                }
                OpCode::Acc(v) => {
                    p_counter += 1;
                    acc += *v;
                }
                OpCode::Jmp(v) => {
                    if !have_branched {
                        if let Some(v) = branch(1, &prev_visits, p_counter) {
                            return Some(acc + v);
                        }
                    }
                    p_counter = add_usize(p_counter, *v);
                }
            }
        } else {
            return Some(acc);
        }
    }

    None
}

#[aoc(day8, part2)]
fn day8_part2(op_codes: &Vec<OpCode>) -> i32 {
    run_until_loop_or_exit(op_codes, BTreeSet::new(), false, 0).unwrap()
}

#[aoc(day8, part1)]
fn day8_part1(op_codes: &Vec<OpCode>) -> i32 {
    let mut p_counter: usize = 0;
    let mut acc = 0i32;

    let mut visited: BTreeSet<usize> = BTreeSet::new();

    while visited.insert(p_counter) {
        let current_code = op_codes
            .get(p_counter)
            .expect(&format!("No op code at {}", &p_counter));

        match current_code {
            OpCode::Nop(_) => {
                p_counter += 1;
            }
            OpCode::Acc(v) => {
                p_counter += 1;
                acc += *v;
            }
            OpCode::Jmp(v) => {
                p_counter = (p_counter as i32 + *v) as usize;
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::{day8_part1, day8_part2, line_to_op_code};
    #[test]
    fn test_given() {
        let input: Vec<_> = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .into_iter()
        .map(line_to_op_code)
        .collect();

        let res = day8_part1(&input);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_given_part_2() {
        let input: Vec<_> = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .into_iter()
        .map(line_to_op_code)
        .collect();

        let res = day8_part2(&input);
        assert_eq!(res, 8);
    }
}
