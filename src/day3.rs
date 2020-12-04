#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().into_iter().collect())
        .collect()
}

#[aoc(day3, part1, mod_op)]
pub fn day3_part1_mod_op(input: &Vec<Vec<char>>) -> u64 {
    get_trees_on_slope(input, 3, 1)
}

#[aoc(day3, part1, its)]
pub fn day3_part_1_its(input: &Vec<Vec<char>>) -> u64 {
    let mut trees = 0;
    let mut current_index = 0usize;

    for line in input {
        if *line.iter().cycle().nth(current_index).unwrap() == '#' {
            trees += 1;
        }
        current_index += 3;
    }

    trees
}

fn get_trees_on_slope(input: &Vec<Vec<char>>, slide: usize, line_skip: usize) -> u64 {
    let width = input.get(0).unwrap().len();
    let mut trees = 0;

    let mut current_index = 0usize;

    for line in input.iter().step_by(line_skip) {
        if *line.get(current_index).unwrap() == '#' {
            trees += 1;
        }
        current_index += slide;
        current_index %= width;
    }
    println!("{:?}", trees);
    trees
}

#[aoc(day3, part2)]
pub fn day3_part2(input: &Vec<Vec<char>>) -> u64 {
    let slides = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slides
        .into_iter()
        .map(|(s, l)| get_trees_on_slope(input, s, l))
        .product()
}

#[cfg(test)]
mod tests {
    use super::get_trees_on_slope as func;

    #[test]
    fn test_basic() {
        let map = vec![vec!['.', '#', '.', '.'], vec!['.', '.', '.', '#']];

        let res = func(&map, 3, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_wrapping() {
        let mut map = vec![];
        map.push(vec!['.', '.', '.', '.']);
        map.push(vec!['.', '.', '.', '#']);
        map.push(vec!['.', '.', '#', '.']);

        let res = func(&map, 3, 1);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_line_skip() {
        let mut map = vec![];
        map.push(vec!['.', '.', '.', '.']);
        map.push(vec!['.', '.', '.', '.']);
        map.push(vec!['.', '.', '#', '.']);
        map.push(vec!['.', '.', '.', '.']);
        map.push(vec!['#', '.', '.', '.']);

        let res = func(&map, 2, 2);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_given_example() {
        let map = vec![
            vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
        ];

        let slides = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let res: u64 = slides.into_iter().map(|(s, l)| func(&map, s, l)).product();
        assert_eq!(res, 336);
    }
}
