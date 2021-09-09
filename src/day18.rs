use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_owned).collect()
}

fn find_matching_parenthesis_index_rev(expression: &str) -> usize {
    if !expression.ends_with(')') {
        panic!("Got expression not ending with ')'");
    }

    let mut parethesis_depth = 1;
    for (i, c) in expression.chars().rev().skip(1).enumerate() {
        parethesis_depth += match c {
            ')' => 1,
            '(' => -1,
            _ => 0,
        };
        if parethesis_depth == 0 {
            return expression.len() - 2 - i;
        }
    }
    panic!("Didn't find matching index");
}

fn solve_expression(expression: &str) -> u64 {
    lazy_static! {
        static ref REV_EXP_PATTERN: Regex =
            Regex::new(r"((?P<rest>.*)\s*(?P<op>[+*])\s*)?(?P<value>\d+)$").unwrap();
    }

    if expression.ends_with(')') {
        let parenthesis_start = find_matching_parenthesis_index_rev(expression);
        let last_part = solve_expression(&expression[parenthesis_start + 1..expression.len() - 1]);
        let rest = expression[..parenthesis_start].trim();
        match rest.chars().last() {
            Some('+') => solve_expression(rest[..rest.len() - 1].trim()) + last_part,
            Some('*') => solve_expression(rest[..rest.len() - 1].trim()) * last_part,
            Some(e) => panic!("Unknown operator {}", e),
            _ => last_part,
        }
    } else {
        let captures = REV_EXP_PATTERN.captures(expression).unwrap();
        let value = captures
            .name("value")
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .unwrap();
        let op = match captures.name("op") {
            Some(m) => m.as_str(),
            None => return value,
        };
        let rest = captures.name("rest").unwrap().as_str().trim();
        match op {
            "+" => solve_expression(rest) + value,
            "*" => solve_expression(rest) * value,
            _ => panic!("Unknown operator {}", op),
        }
    }
}

#[aoc(day18, part1)]
fn day18_part1(expressions: &[String]) -> u128 {
    expressions
        .iter()
        .map(|c| solve_expression(c) as u128)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{find_matching_parenthesis_index_rev, solve_expression};
    use test_case::test_case;

    #[test_case("3 + (2 * 3)" => 4)]
    #[test_case("(3 + 5) * (2 * 4 + 10)" => 10)]
    #[test_case("2 + ((3 * 5) + 2)" => 4)]
    #[test_case("(2 + 3)" => 0)]
    fn test_find_matching_parenthesis_rev(expression: &str) -> usize {
        find_matching_parenthesis_index_rev(expression)
    }

    #[test_case("5" => 5 ; "simple")]
    #[test_case("2 * 3" => 6)]
    #[test_case("4 * 5" => 20)]
    #[test_case("4 + (2 * 3)" => 10)]
    #[test_case("2 + 2 + 2" => 6)]
    #[test_case("2 * 3 + (4 * 5)" => 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)" => 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 13632)]
    fn test_day18_given_part_1(expression: &str) -> u64 {
        solve_expression(expression)
    }
}
