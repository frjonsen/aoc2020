use regex::Regex;
use std::collections::HashMap;

type KnownRules = HashMap<u64, String>;
type RawRules = HashMap<u64, String>;

#[aoc_generator(day19)]
fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (
        parts[0].lines().map(str::trim).map(str::to_owned).collect(),
        parts[1].lines().map(str::trim).map(str::to_owned).collect(),
    )
}

fn get_rule(rule_number: u64, known_rules: &mut KnownRules, raw_rules: &RawRules) -> String {
    if let Some(r) = known_rules.get(&rule_number) {
        return r.clone();
    }

    let rule_options = raw_rules
        .get(&rule_number)
        .expect("Rule was not found in raw_rules")
        .split('|');

    let mut parsed_parts: Vec<String> = Vec::new();

    for rule_option in rule_options {
        let rule_parts = rule_option.split_whitespace();

        let mut to_concat = Vec::new();
        for rule_part in rule_parts {
            if let Ok(num) = rule_part.parse() {
                let known_rule_part = get_rule(num, known_rules, raw_rules);
                to_concat.push(known_rule_part);
            } else {
                // Found one of the base rules
                known_rules.insert(rule_number, rule_part[1..2].to_owned());
                return rule_part[1..2].to_owned();
            }
        }

        parsed_parts.push(to_concat.join(""));
    }

    let finished_rule = format!("({})", parsed_parts.join("|"));

    known_rules.insert(rule_number, finished_rule.clone());

    finished_rule
}

#[aoc(day19, part1)]
fn day19_part1(parts: &(Vec<String>, Vec<String>)) -> usize {
    let raw_rules: RawRules = parts
        .0
        .iter()
        .map(|l| {
            let rule_parts: Vec<_> = l.split(':').collect();
            (
                rule_parts[0].parse().unwrap(),
                rule_parts[1].trim().to_owned(),
            )
        })
        .collect();

    let mut known_rules = KnownRules::new();

    let zero_rule = get_rule(0, &mut known_rules, &raw_rules);

    println!("{}", zero_rule);

    let zero_rule_pattern = Regex::new(&format!("^{}$", zero_rule)).unwrap();

    parts
        .1
        .iter()
        .filter(|r| zero_rule_pattern.is_match(r))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{get_rule, KnownRules, RawRules};
    use std::collections::HashMap;

    #[test]
    fn test_basic_concat_from_known() {
        let raw_rules: RawRules = vec![(1, "2 | 3".to_owned())].into_iter().collect();
        let mut known_rules: KnownRules = vec![(2, "a".to_owned()), (3, "b".to_owned())]
            .into_iter()
            .collect();

        let res = get_rule(1, &mut known_rules, &raw_rules);
        assert_eq!(res, "(a|b)");
    }

    #[test]
    fn test_basic_concat_from_raw() {
        let raw_rules: RawRules = vec![
            (1, "2 | 3".to_owned()),
            (2, "\"a\"".to_owned()),
            (3, "\"b\"".to_owned()),
        ]
        .into_iter()
        .collect();
        let res = get_rule(1, &mut KnownRules::new(), &raw_rules);
        assert_eq!(res, "(a|b)");
    }

    #[test]
    fn test_basic_concat_more() {
        let raw_rules: RawRules = vec![
            (1, "2 2 | 3 2".to_owned()),
            (2, "\"a\"".to_owned()),
            (3, "\"b\"".to_owned()),
        ]
        .into_iter()
        .collect();
        let res = get_rule(1, &mut KnownRules::new(), &raw_rules);
        assert_eq!(res, "(aa|ba)");
    }

    #[test]
    fn test_basic_concat_nested() {
        let raw_rules: RawRules = vec![
            (1, "2 4 | 3 2".to_owned()),
            (2, "\"a\"".to_owned()),
            (3, "\"b\"".to_owned()),
            (4, "2 | 3".to_owned()),
        ]
        .into_iter()
        .collect();
        let res = get_rule(1, &mut KnownRules::new(), &raw_rules);
        assert_eq!(res, "(a(a|b)|ba)");
    }
}
