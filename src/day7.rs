use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Rule {
    bag_name: String,
    count: u32,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

pub fn parse_rule(line: &String, bag_rules: &mut HashMap<String, HashSet<String>>) {
    let bag_rule_pattern = Regex::new(r"([a-z]+ [a-z]+)").unwrap();
    let matches = bag_rule_pattern
        .find_iter(line)
        .map(|c| c.as_str().to_owned())
        .collect::<Vec<_>>();

    let mut iter = matches.into_iter();
    let container = iter.next().unwrap().to_owned();
    iter.filter(|f| f != "bags contain")
        .take_while(|c| c != "no other")
        .for_each(|c| {
            bag_rules
                .entry(c)
                .or_insert_with(|| HashSet::new())
                .insert(container.clone());
        });
}

pub fn parse_rule_part_2(line: &String) -> (String, HashSet<Rule>) {
    let bag_rule_pattern = Regex::new(r"(?P<count>\d+) (?P<name>\w+ \w+)").unwrap();
    let parts: Vec<_> = line.split("bags contain").collect();
    let container = (*parts.get(0).expect("Found no container in line"))
        .trim()
        .to_owned();
    let contained_in_parts = parts.get(1).expect("Found no contained in line").split(",");
    let mut rules = HashSet::new();
    for p in contained_in_parts {
        if let Some(m) = bag_rule_pattern.captures(p) {
            rules.insert(Rule {
                bag_name: m.name("name").unwrap().as_str().to_owned(),
                count: m.name("count").unwrap().as_str().parse().unwrap(),
            });
        }
    }

    (container, rules)
}

fn count_containers<'a>(
    contained: &String,
    rules: &'a HashMap<String, HashSet<String>>,
) -> HashSet<&'a String> {
    if let Some(contained_in) = rules.get(contained) {
        let mut known_containers: HashSet<&String> = contained_in.iter().map(|s| s).collect();

        for c in contained_in {
            known_containers = &known_containers | &count_containers(c, rules);
        }

        known_containers
    } else {
        HashSet::new()
    }
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &Vec<String>) -> usize {
    let mut bag_rules: HashMap<String, HashSet<String>> = HashMap::new();

    input.into_iter().for_each(|l| {
        parse_rule(l, &mut bag_rules);
    });

    let containers = count_containers(&"shiny gold".to_owned(), &bag_rules);

    containers.len()
}

pub fn count_contained(bag: &String, rules: &HashMap<String, HashSet<Rule>>) -> u32 {
    rules
        .get(bag)
        .unwrap()
        .iter()
        .map(|r| r.count + r.count * count_contained(&r.bag_name, rules))
        .sum()
}

#[aoc(day7, part2)]
pub fn day7_part2(input: &Vec<String>) -> u32 {
    let rules = input
        .into_iter()
        .map(parse_rule_part_2)
        .collect::<HashMap<String, HashSet<Rule>>>();

    count_contained(&"shiny gold".to_owned(), &rules)
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::{day7_part1, day7_part2, parse_rule, parse_rule_part_2, Rule};

    #[test]
    fn test_parse_rule_basic() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_owned();
        let mut bag_rules = HashMap::new();
        parse_rule(&input, &mut bag_rules);
        assert_eq!(bag_rules.len(), 2);
        let mut s = HashSet::new();
        s.insert("light red".to_owned());
        assert_eq!(bag_rules["bright white"], s);
        assert_eq!(bag_rules["muted yellow"], s);
    }

    #[test]
    fn test_parse_part_2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_owned();
        let res = parse_rule_part_2(&input);
        assert_eq!("light red".to_owned(), res.0);
        let mut expected: HashSet<Rule> = HashSet::new();
        expected.extend(
            vec![
                Rule {
                    bag_name: "bright white".to_owned(),
                    count: 1,
                },
                Rule {
                    bag_name: "muted yellow".to_owned(),
                    count: 2,
                },
            ]
            .into_iter(),
        );
        assert_eq!(expected, res.1);
    }

    #[test]
    fn test_given() {
        let input: Vec<_> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        let res = day7_part1(&input);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_given_part_2() {
        let input: Vec<_> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        let res = day7_part2(&input);
        assert_eq!(res, 32)
    }

    #[test]
    fn test_given_part_2_example_2() {
        let input: Vec<_> = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let res = day7_part2(&input);
        assert_eq!(res, 126);
    }

    #[test]
    fn test_part_2_small() {
        let input: Vec<_> = vec![
            "shiny gold bags container 2 dark red bags.",
            "dark red bags contain 2 dark blue bags.",
            "dark blue bags contain no other bags.",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let res = day7_part2(&input);
        assert_eq!(res, 6);
    }
}
