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

    let finished_rule = format!("(?:{})", parsed_parts.join("|"));

    known_rules.insert(rule_number, finished_rule.clone());

    finished_rule
}

fn parse_raw_rules<'a>(as_input: impl Iterator<Item = &'a String>) -> RawRules {
    as_input
        .map(|l| {
            let rule_parts: Vec<_> = l.split(':').collect();
            (
                rule_parts[0].parse().unwrap(),
                rule_parts[1].trim().to_owned(),
            )
        })
        .collect()
}

#[aoc(day19, part1)]
fn day19_part1(parts: &(Vec<String>, Vec<String>)) -> usize {
    let raw_rules = parse_raw_rules(parts.0.iter());

    let mut known_rules = KnownRules::new();

    let zero_rule = get_rule(0, &mut known_rules, &raw_rules);

    let zero_rule_pattern = Regex::new(&format!("^{}$", zero_rule)).unwrap();

    parts
        .1
        .iter()
        .filter(|r| zero_rule_pattern.is_match(r))
        .count()
}

#[aoc(day19, part2)]
fn day19_part2(parts: &(Vec<String>, Vec<String>)) -> usize {
    let raw_rules = parse_raw_rules(parts.0.iter());

    let mut known_rules = KnownRules::new();

    let rule_42 = get_rule(42, &mut known_rules, &raw_rules);
    let rule_31 = get_rule(31, &mut known_rules, &raw_rules);

    let rule_31_pattern = Regex::new(&format!("({})", rule_31)).unwrap();
    let rule_42_pattern = Regex::new(&format!("({})", rule_42)).unwrap();

    let zero_rule_pattern = Regex::new(&format!(
        "^(?P<ft>(?:{}){{2,}})(?P<to>(?:{})+)$",
        rule_42, rule_31
    ))
    .unwrap();

    parts
        .1
        .iter()
        .map(|m| zero_rule_pattern.captures_iter(m).collect::<Vec<_>>())
        .filter(|m| {
            if let Some(capture) = m.first() {
                let to_matches = rule_31_pattern.find_iter(&capture["to"]).count();
                let ft_matches = rule_42_pattern.find_iter(&capture["ft"]).count();

                to_matches < ft_matches
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::{day19_part2, get_rule, input_generator, KnownRules, RawRules};

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

    #[test]
    fn test_day_19_given_part_2() {
        let input = r#"42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let generated = input_generator(input);
        let res = day19_part2(&generated);
        assert_eq!(res, 12);
    }
}
