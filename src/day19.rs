use regex::Regex;
use std::collections::HashMap;
#[aoc_generator(day19)]
fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (
        parts[0].lines().map(str::trim).map(str::to_owned).collect(),
        parts[1].lines().map(str::trim).map(str::to_owned).collect(),
    )
}

enum Rule {
    DirectMatch(String),
    Pattern(Regex),
}

fn update_rules(existing_rules: &[Rule], to_append: &[Rule]) -> Vec<Rule> {
    println!(
        "Combinding {} new rules with {} existing",
        to_append.len(),
        existing_rules.len()
    );
    let mut new_rules = Vec::new();

    for appendage in to_append {
        for existing_rule in existing_rules {
            let new_rule = match (existing_rule, appendage) {
                (Rule::DirectMatch(existing), Rule::DirectMatch(new)) => {
                    Rule::DirectMatch(format!("{}{}", existing, new))
                }
                (Rule::DirectMatch(existing), Rule::Pattern(new)) => {
                    Rule::Pattern(Regex::new(&format!("{}{}", existing, new.as_str())).unwrap())
                }
                (Rule::Pattern(existing), Rule::DirectMatch(new)) => {
                    Rule::Pattern(Regex::new(&format!("{}{}", existing.as_str(), new)).unwrap())
                }
                (Rule::Pattern(existing), Rule::Pattern(new)) => Rule::Pattern(
                    Regex::new(&format!("{}{}", existing.as_str(), new.as_str())).unwrap(),
                ),
            };
            new_rules.push(new_rule);
        }
    }

    new_rules
}

fn parse_rule(
    rule: &str,
    parsed_rules: &mut HashMap<String, Vec<Rule>>,
    raw_rules: &HashMap<String, String>,
) -> Vec<Rule> {
    if rule.contains('|') {
        rule.split('|')
            .map(str::trim)
            .map(|r| parse_rule(r, parsed_rules, raw_rules))
            .flatten()
            .collect()
    } else {
        let mut rule_builders: Vec<Rule> = vec![Rule::DirectMatch(String::new())];
        for part in rule.split_whitespace() {
            if part.starts_with('"') {
                return vec![Rule::DirectMatch(part[1..2].to_owned())];
            }
            if let Some(already_parsed) = parsed_rules.get(part) {
                rule_builders = update_rules(&rule_builders, already_parsed);
            } else {
                let raw_rule = raw_rules.get(part).unwrap();
                let parse_result = parse_rule(raw_rule, parsed_rules, raw_rules);
                rule_builders = update_rules(&rule_builders, &parse_result);

                parsed_rules.insert(part.to_owned(), parse_result);
            }
        }

        rule_builders
    }
}

#[aoc(day19, part1)]
fn day19_part1(parts: &(Vec<String>, Vec<String>)) -> usize {
    let mut parsed_rules: HashMap<String, Vec<Rule>> = HashMap::new();

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
        .filter(|m| {
            zero_rule.iter().any(|r| match r {
                Rule::DirectMatch(r) => r == *m,
                Rule::Pattern(r) => r.is_match(m),
            })
        })
        .count()
}

#[aoc(day19, part2)]
fn day19_part2(parts: &(Vec<String>, Vec<String>)) -> usize {
    let mut parsed_rules: HashMap<String, Vec<Rule>> = HashMap::new();

    let raw_rules: HashMap<String, String> = parts
        .0
        .iter()
        .map(|l| {
            let rule_parts: Vec<_> = l.split(':').collect();
            (rule_parts[0].to_owned(), rule_parts[1].trim().to_owned())
        })
        .collect();

    let rule_42 = parse_rule("42", &mut parsed_rules, &raw_rules);
    let rule_31 = parse_rule("31", &mut parsed_rules, &raw_rules);
    for r in rule_42.iter() {
        let p = match r {
            Rule::DirectMatch(m) => m,
            Rule::Pattern(m) => m.as_str(),
        };
        println!("{}", p);
    }
    let rule_8: Vec<Rule> = rule_42
        .iter()
        .map(|r| match r {
            Rule::DirectMatch(s) => Rule::DirectMatch(format!("{}({})*", s, s)),
            _ => panic!("Rule 42 was wrong type"),
        })
        .collect();

    let rule_11 = rule_42
        .iter()
        .map(|ft| {
            rule_31
                .iter()
                .map(|to| match (ft, to) {
                    (Rule::DirectMatch(fts), Rule::DirectMatch(tos)) => {
                        Rule::DirectMatch(format!("{}({}{})*{}", fts, fts, tos, tos))
                    }
                    _ => panic!("Wrong types"),
                })
                .collect::<Vec<Rule>>()
        })
        .flatten()
        .collect();

    parsed_rules.insert("31".to_owned(), rule_31);
    parsed_rules.insert("42".to_string(), rule_42);
    parsed_rules.insert("8".to_owned(), rule_8);
    parsed_rules.insert("11".to_owned(), rule_11);

    println!("Starting parsing of rules");
    for r in raw_rules.iter() {
        if parsed_rules.contains_key(r.0) {
            continue;
        }

        let rule = parse_rule(r.1, &mut parsed_rules, &raw_rules);
        parsed_rules.insert(r.0.clone(), rule);
    }
    println!("Done parsing rules");

    let zero_rule = parsed_rules.get("0").unwrap();
    let zero_rule_strings = zero_rule
        .iter()
        .map(|r| match r {
            Rule::DirectMatch(s) => s,
            _ => panic!("No"),
        })
        .collect::<Vec<_>>();

    println!("Zero rule has {} variations", zero_rule.len());

    /*
    println!("Converting to patterns");
    let patterns: Vec<Regex> = zero_rule
        .iter()
        .map(|r| match r {
            Rule::DirectMatch(m) => Regex::new(&format!("^{}$", m)).unwrap(),
            _ => panic!("No"),
        })
        .collect();
        */

    let mut matches = 0;
    let mut checked = 0;
    println!("Checking messages");
    for message in parts.1.iter() {
        checked += 1;
        println!("Checked {} messages", checked);
        if zero_rule_strings
            .iter()
            .any(|r| Regex::new(r).unwrap().is_match(message))
        {
            matches += 1;
        }
    }
    matches

    /*
    parts
        .1
        .iter()
        .filter(|m| patterns.iter().any(|r| r.is_match(m)))
        .count()
        */
}

#[cfg(test)]
mod tests {
    use super::{input_generator, parse_rule, update_rules, Rule};
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn test_update_rule() {
        let rule_builders = vec![Rule::DirectMatch("a".to_owned())];
        let to_append = vec![Rule::DirectMatch("b".to_owned())];
        let res = update_rules(&rule_builders, &to_append);
        assert_eq!(res.len(), 1);
        if let Rule::DirectMatch(p) = &res[0] {
            assert_eq!(p, "ab");
        } else {
            panic!("Wrong type");
        }
    }

    #[test]
    fn test_update_rule_append_regex() {
        let rule_builders = vec![Rule::DirectMatch("a".to_owned())];
        let to_append = vec![Rule::Pattern(Regex::new("b").unwrap())];
        let res = update_rules(&rule_builders, &to_append);
        assert_eq!(res.len(), 1);
        if let Rule::Pattern(p) = &res[0] {
            assert_eq!(p.as_str(), "ab");
        } else {
            panic!("Wrong type");
        }
    }

    #[test]
    fn test_update_rule_append_to_regex() {
        let rule_builders = vec![Rule::Pattern(Regex::new("a").unwrap())];
        let to_append = vec![Rule::DirectMatch("ab".to_owned())];
        let res = update_rules(&rule_builders, &to_append);
        assert_eq!(res.len(), 1);
        if let Rule::Pattern(p) = &res[0] {
            assert_eq!(p.as_str(), "aab");
        } else {
            panic!("Wrong type");
        }
    }

    //    #[test]
    //fn test_rule_parsing() {
    //let mut raw_rules = HashMap::new();
    //raw_rules.insert("1".to_owned(), "2 3 | 3 2".to_owned());
    //raw_rules.insert("2".to_owned(), "4 4 | 5 5".to_owned());
    //raw_rules.insert("3".to_owned(), "4 5 | 5 4".to_owned());
    //raw_rules.insert("4".to_owned(), "\"a\"".to_owned());
    //raw_rules.insert("5".to_owned(), "\"b\"".to_owned());

    //let mut parsed_rules: HashMap<String, Vec<String>> = HashMap::new();

    //let res = parse_rule("4 1 5", &mut parsed_rules, &raw_rules);

    //assert_eq!(res.len(), 8);
    //    }

    //#[test]
    //fn test_day19_part1() {
    //let input = r#"0: 4 1 5
    //1: 2 3 | 3 2
    //2: 4 4 | 5 5
    //3: 4 5 | 5 4
    //4: "a"
    //5: "b"

    //ababbb
    //bababa
    //abbbab
    //aaabbb
    //aaaabbb"#;

    //let generated = input_generator(input);
    //let res = day19_part1(&generated);
    //assert_eq!(res, 2);
    //}
}
