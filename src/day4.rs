use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    let pattern = Regex::new(r"(\n\s*\n)").unwrap();
    let pair_pattern = Regex::new(r"(?P<key>.*):(?P<value>.*)").unwrap();
    pattern
        .split(input)
        .map(|p| {
            p.split_whitespace()
                .map(|p| pair_pattern.captures(p).unwrap())
                .map(|pair| {
                    (
                        pair.name("key").unwrap().as_str().to_owned(),
                        pair.name("value").unwrap().as_str().to_owned(),
                    )
                })
                .collect::<HashMap<String, String>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
pub fn day4_part1(passports: &Vec<HashMap<String, String>>) -> usize {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports
        .into_iter()
        .filter(|p| fields.iter().all(|f| p.contains_key(*f)))
        .count()
}

#[aoc(day4, part2)]
pub fn day4_part2(passports: &Vec<HashMap<String, String>>) -> usize {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let int_check = |f: &str, min: u32, max: u32| {
        let n: u32 = f.parse().unwrap();
        n >= min && n <= max
    };

    let hgt_pattern = Regex::new(r"^(?P<hgt>\d+)(?P<unit>(cm|in))$").unwrap();
    let hair_color_pattern = Regex::new(r"^#\w{6}$").unwrap();

    let hgt_check = |f: &str| {
        if let Some(parts) = hgt_pattern.captures(f) {
            let height = parts.name("hgt").expect("No height number").as_str();
            let unit = parts.name("unit").expect("No height unit").as_str();
            match unit {
                "in" => int_check(height, 59, 76),
                "cm" => int_check(height, 150, 193),
                _ => false,
            }
        } else {
            false
        }
    };
    let eye_colors = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_pattern = Regex::new(r"^\d{9}$").unwrap();

    // Do match instead of using HashMap with closures
    passports
        .into_iter()
        .filter(|p| fields.iter().all(|f| p.contains_key(*f)))
        .filter(|p| {
            p.iter().all(|(k, v)| match k.as_ref() {
                "byr" => int_check(v, 1920, 2002),
                "iyr" => int_check(v, 2010, 2020),
                "eyr" => int_check(v, 2020, 2030),
                "hgt" => hgt_check(v),
                "hcl" => hair_color_pattern.is_match(v),
                "ecl" => eye_colors.iter().any(|f| f == v),
                "pid" => pid_pattern.is_match(v),
                "cid" => true,
                _ => panic!("Unexpected field"),
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::input_generator;

    #[test]
    pub fn test_generator_single() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";
        let parsed = input_generator(input);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed.get(0).unwrap().keys().len(), 8)
    }

    #[test]
    pub fn test_generator_multiple() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let parsed = input_generator(input);
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed.get(0).unwrap().keys().len(), 8);
        assert_eq!(parsed.get(1).unwrap().keys().len(), 7);
        assert_eq!(parsed.get(2).unwrap().keys().len(), 7);
        assert_eq!(parsed.get(3).unwrap().keys().len(), 6);

        let second = parsed.get(1).unwrap();
        let mut expected = HashMap::new();
        expected.insert("iyr", "2013");
        expected.insert("ecl", "amb");
        expected.insert("cid", "350");
        expected.insert("eyr", "2023");
        expected.insert("pid", "028048884");
        expected.insert("hcl", "#cfa07d");
        expected.insert("byr", "1929");
        expected
            .into_iter()
            .for_each(|(k, v)| assert_eq!(second[k], v))
    }
}
