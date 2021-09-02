use regex::Regex;
use std::collections::HashMap;
#[derive(Debug)]
enum Operation {
    SetMask { mask: String },
    SetMem { adress: usize, value: u64 },
}

#[derive(Debug)]
struct Mask {
    set: u64,
    unset: u64,
}

fn to_mask(mask: &str) -> Mask {
    let keep: u64 = 0b111111111111111111111111111111111111;
    let set: String = mask.replace("X", "0");
    let unset: String = mask.replace("X", "1");

    let set = u64::from_str_radix(&set, 2).unwrap();
    let unset = u64::from_str_radix(&unset, 2).unwrap();

    Mask {
        set,
        unset: unset & keep,
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Operation> {
    let p = Regex::new(r"(\[(?P<adress>\d+)\])? = (?P<value>.*)").unwrap();

    input
        .lines()
        .map(&str::trim)
        .map(|l| p.captures(l).unwrap())
        .map(|c| {
            let value = c.name("value").unwrap().as_str();
            if let Some(adress) = c.name("adress") {
                Operation::SetMem {
                    adress: adress.as_str().parse().unwrap(),
                    value: value.parse().unwrap(),
                }
            } else {
                Operation::SetMask {
                    mask: value.to_owned(),
                }
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn day14_part1(input: &[Operation]) -> u64 {
    let mut current_mask = Mask { set: 0, unset: 0 };
    let mut stack: HashMap<usize, u64> = HashMap::new();
    for op in input {
        match op {
            Operation::SetMask { mask } => current_mask = to_mask(&mask),
            Operation::SetMem { adress, value } => {
                stack.insert(*adress, (value | current_mask.set) & current_mask.unset);
            }
        }
    }

    stack.values().sum()
}

fn replace_x(indices: &[usize], adress: &str) -> Vec<String> {
    if indices.is_empty() {
        return vec![adress.to_owned()];
    }
    let index = indices[0];

    let mut one_replacement = adress.to_owned();
    let mut zero_replacement = one_replacement.clone();
    one_replacement.replace_range(index..(index + 1), "1");
    zero_replacement.replace_range(index..(index + 1), "0");
    let one_branch = replace_x(&indices[1..], &one_replacement);
    let zero_branch = replace_x(&indices[1..], &zero_replacement);

    one_branch
        .into_iter()
        .chain(zero_branch.into_iter())
        .collect()
}

fn set_at_adress(stack: &mut HashMap<usize, u64>, mask: &str, adress: usize, value: u64) {
    let adress = adress | usize::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
    let adress = format!("{:036b}", adress);
    let indices: Vec<usize> = mask.match_indices('X').map(|c| c.0).collect();

    let adresses = replace_x(&indices, &adress)
        .into_iter()
        .map(|a| usize::from_str_radix(&a, 2).unwrap());

    for adress in adresses {
        stack.insert(adress, value);
    }
}

#[aoc(day14, part2)]
fn day14_part2(input: &[Operation]) -> u64 {
    let mut current_mask = String::new();

    let mut stack: HashMap<usize, u64> = HashMap::new();
    for op in input {
        match op {
            Operation::SetMask { mask } => current_mask = mask.clone(),
            Operation::SetMem { adress, value } => {
                set_at_adress(&mut stack, &current_mask, *adress, *value)
            }
        }
    }

    stack.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{day14_part1, day14_part2, input_generator, to_mask};

    #[test]
    fn test_parse_mask() {
        let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask = to_mask(&input);
        assert_eq!(mask.set, 0b1000000);
        assert_eq!(mask.unset, 0b111111111111111111111111111111111101);
    }

    #[test]
    fn test_given_part_1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0";

        let generated = input_generator(input);
        let res = day14_part1(&generated);
        assert_eq!(res, 165);
    }

    #[test]
    fn test_given_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1";
        let generated = input_generator(input);
        let res = day14_part2(&generated);
        assert_eq!(res, 208);
    }
}
