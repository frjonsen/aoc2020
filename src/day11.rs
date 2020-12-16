#[derive(Eq, PartialEq, Clone)]
enum Tile {
    Empty,
    Floor,
    Occupied,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(&str::trim)
        .map(&str::chars)
        .map(|f| {
            f.into_iter()
                .map(|c| match c {
                    '#' => Tile::Occupied,
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    _ => panic!("Unrecognized tile"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn print_map(map: &Vec<Vec<Tile>>) {
    map.iter().for_each(|f| {
        let s: String = f
            .iter()
            .map(|c| match c {
                Tile::Occupied => '#',
                Tile::Empty => 'L',
                Tile::Floor => '.',
            })
            .collect();
        println!("{}", s);
    });
}

#[aoc(day11, part1)]
fn day11_part1(input: &Vec<Vec<Tile>>) -> usize {
    let mut map = input.clone();

    loop {
        let res = simulate_move(&map);
        map = res.0;
        if !res.1 {
            break;
        }
    }
    map.into_iter()
        .flatten()
        .filter(|t| *t == Tile::Occupied)
        .count()
}

fn count_occupied_adjacent(input: &[Vec<Tile>], x: usize, y: usize) -> usize {
    let checks = -1i32..=1;
    checks
        .clone()
        .map(|c| checks.clone().map(move |f| (c, f)))
        .flatten()
        .filter(|f| *f != (0, 0))
        .map(|c| {
            input
                .get((y as i32 + c.0) as usize)
                .and_then(|r| r.get((x as i32 + c.1) as usize))
        })
        .filter(Option::is_some)
        .flatten()
        .filter(|c| **c == Tile::Occupied)
        .count()
}

fn simulate_move(input: &Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, bool) {
    let mut result: Vec<Vec<Tile>> = Vec::with_capacity(input.len());
    let mut changes = false;
    for y in input.iter().enumerate() {
        let mut new_row: Vec<Tile> = Vec::with_capacity(input[0].len());
        for x in y.1.iter().enumerate() {
            if *x.1 == Tile::Floor {
                new_row.push(Tile::Floor);
            } else if *x.1 == Tile::Empty {
                if count_occupied_adjacent(input, x.0, y.0) == 0 {
                    new_row.push(Tile::Occupied);
                    changes = true;
                } else {
                    new_row.push(Tile::Empty);
                }
            } else {
                if count_occupied_adjacent(input, x.0, y.0) >= 4 {
                    new_row.push(Tile::Empty);
                    changes = true;
                } else {
                    new_row.push(Tile::Occupied);
                }
            }
        }
        result.push(new_row);
    }

    (result, changes)
}

#[cfg(test)]
mod tests {
    use super::{count_occupied_adjacent, day11_part1, input_generator, Tile};
    const INPUT1: &'static str = "#.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##";

    #[test]
    fn test_occupied_count() {
        let map = vec![
            vec![Tile::Floor, Tile::Occupied, Tile::Floor],
            vec![Tile::Occupied, Tile::Occupied, Tile::Occupied],
            vec![Tile::Floor, Tile::Empty, Tile::Occupied],
        ];

        let res = count_occupied_adjacent(&map, 1, 1);
        assert_eq!(res, 4);
        let res = count_occupied_adjacent(&map, 0, 1);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_given_part_1() {
        let map = input_generator(INPUT1);
        let res = day11_part1(&map);
        assert_eq!(res, 37)
    }
}
