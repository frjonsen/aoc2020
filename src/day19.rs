use std::collections::HashMap;
#[aoc_generator(day19)]
fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (
        parts[0].lines().map(str::trim).map(str::to_owned).collect(),
        parts[1].lines().map(str::trim).map(str::to_owned).collect(),
    )
}

fn parse_rule(
    rule: &str,
    parsed_rules: &mut HashMap<String, Vec<String>>,
    raw_rules: &HashMap<String, String>,
) -> Vec<String> {
    if rule.contains('|') {
        rule.split('|')
            .map(str::trim)
            .map(|r| parse_rule(r, parsed_rules, raw_rules))
            .flatten()
            .collect()
    } else {
        let mut rule_builders: Vec<String> = vec![String::new()];
        for part in rule.split_whitespace() {
            if part.starts_with('"') {
                return vec![part[1..2].to_owned()];
            }
            if let Some(already_parsed) = parsed_rules.get(part) {
                rule_builders = already_parsed
                    .iter()
                    .map(|p| {
                        let f = p.clone();
                        rule_builders.iter().map(move |r| format!("{}{}", &r, f))
                    })
                    .flatten()
                    .collect();
            } else {
                let raw_rule = raw_rules.get(part).unwrap();
                let parse_result = parse_rule(raw_rule, parsed_rules, raw_rules);

                rule_builders = parse_result
                    .iter()
                    .map(|p| {
                        let f = p.clone();
                        rule_builders.iter().map(move |r| format!("{}{}", r, f))
                    })
                    .flatten()
                    .collect();

                parsed_rules.insert(part.to_owned(), parse_result);
            }
        }

        rule_builders
    }
}

#[aoc(day19, part1)]
fn day19_part1(parts: &(Vec<String>, Vec<String>)) -> usize {
    // let mut rules: Vec<String> = Vec::new();

    let mut parsed_rules: HashMap<String, Vec<String>> = HashMap::new();

    let raw_rules: HashMap<String, String> = parts
        .0
        .iter()
        .map(|l| {
            let rule_parts: Vec<_> = l.split(':').collect();
            (rule_parts[0].to_owned(), rule_parts[1].trim().to_owned())
        })
        .collect();

    for r in raw_rules.iter() {
        if parsed_rules.contains_key(r.0) {
            continue;
        }

        let rule = parse_rule(r.1, &mut parsed_rules, &raw_rules);
        parsed_rules.insert(r.0.clone(), rule);
    }

    let zero_rule = parsed_rules.get("0").unwrap();

    parts
        .1
        .iter()
        .filter(|m| zero_rule.iter().any(|r| &r == m))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{day19_part1, input_generator, parse_rule};
    use std::collections::HashMap;

    #[test]
    fn test_rule_parsing() {
        let mut raw_rules = HashMap::new();
        raw_rules.insert("1".to_owned(), "2 3 | 3 2".to_owned());
        raw_rules.insert("2".to_owned(), "4 4 | 5 5".to_owned());
        raw_rules.insert("3".to_owned(), "4 5 | 5 4".to_owned());
        raw_rules.insert("4".to_owned(), "\"a\"".to_owned());
        raw_rules.insert("5".to_owned(), "\"b\"".to_owned());

        let mut parsed_rules: HashMap<String, Vec<String>> = HashMap::new();

        let res = parse_rule("4 1 5", &mut parsed_rules, &raw_rules);

        assert_eq!(res.len(), 8);
    }

    #[test]
    fn test_day19_part1() {
        let input = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;

        let generated = input_generator(input);
        let res = day19_part1(&generated);
        assert_eq!(res, 2);
    }
}
