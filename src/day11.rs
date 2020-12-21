use lazy_static::lazy_static;

lazy_static! {
    static ref DIRECTIONS: [Coordinate; 8] = [
        Coordinate { x: 1, y: 0 },
        Coordinate { x: -1, y: 0 },
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: -1 },
        Coordinate { x: 1, y: 1 },
        Coordinate { x: -1, y: 1 },
        Coordinate { x: 1, y: -1 },
        Coordinate { x: -1, y: -1 },
    ];
}
#[derive(Eq, PartialEq, Clone)]
enum Tile {
    Empty,
    Floor,
    Occupied,
}

#[derive(PartialEq, Eq, Debug)]
struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl From<(usize, usize)> for Coordinate {
    fn from(coord: (usize, usize)) -> Coordinate {
        Coordinate {
            x: coord.0 as i64,
            y: coord.1 as i64,
        }
    }
}

impl std::ops::Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, other: Self) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Map = Vec<Vec<Coordinate>>;
type TileMap = Vec<Vec<Tile>>;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> TileMap {
    input
        .lines()
        .map(&str::trim)
        .map(&str::chars)
        .map(|f| {
            f.into_iter()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    _ => panic!("Unrecognized tile"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[allow(dead_code)]
fn print_map(map: &TileMap) {
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
    println!("");
}

#[aoc(day11, part1)]
fn day11_part1(input: &TileMap) -> usize {
    let mut map = input.clone();

    loop {
        let res = simulate_move(&map, 4, get_seats_to_check);
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
#[aoc(day11, part2)]
fn day11_part2(input: &TileMap) -> usize {
    let mut map = input.clone();

    loop {
        let res = simulate_move(&map, 5, seat_calc_part_2);
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

fn get_seats_to_check(_: &[Vec<Tile>], coord: Coordinate) -> Map {
    DIRECTIONS.iter().map(|c| vec![c + &coord]).collect()
}

fn calculcate_visible_seats(
    current: &Coordinate,
    direction: &Coordinate,
    limit_y: usize,
    limit_x: usize,
) -> Vec<Coordinate> {
    let first: Coordinate = current + direction;
    let mut coordinates = Vec::new();

    if first.x < 0 && first.x >= limit_x as i64 && first.y < 0 && first.y >= limit_y as i64 {
        return vec![];
    }
    coordinates.push(first);

    loop {
        let next = coordinates.last().unwrap() + &direction;
        if next.y < 0 || next.y >= limit_y as i64 || next.x < 0 || next.x >= limit_x as i64 {
            return coordinates;
        }
        coordinates.push(next);
    }
}

fn count_occupied_adjacent(input: &[Vec<Tile>], to_check: Map) -> usize {
    to_check
        .into_iter()
        .filter_map(|c| {
            c.iter().find_map(|f| {
                input
                    .get(f.y as usize)
                    .and_then(|r| r.get(f.x as usize))
                    .filter(|c| **c != Tile::Floor)
            })
        })
        .filter(|c| **c == Tile::Occupied)
        .count()
}
fn seat_calc_part_2(map: &[Vec<Tile>], current: Coordinate) -> Map {
    DIRECTIONS
        .iter()
        .map(|c| calculcate_visible_seats(&current, c, map.len(), map[0].len()))
        .collect()
}

fn simulate_move(
    input: &TileMap,
    tolerance: usize,
    seat_calc: impl Fn(&[Vec<Tile>], Coordinate) -> Map,
) -> (TileMap, bool) {
    let mut result: TileMap = Vec::with_capacity(input.len());
    let mut changes = false;
    for y in input.iter().enumerate() {
        let mut new_row: Vec<Tile> = Vec::with_capacity(input[0].len());
        for x in y.1.iter().enumerate() {
            if *x.1 == Tile::Floor {
                new_row.push(Tile::Floor);
            } else if *x.1 == Tile::Empty {
                let to_check = seat_calc(input, (x.0, y.0).into());
                if count_occupied_adjacent(input, to_check) == 0 {
                    new_row.push(Tile::Occupied);
                    changes = true;
                } else {
                    new_row.push(Tile::Empty);
                }
            } else {
                let to_check = seat_calc(input, (x.0, y.0).into());
                if count_occupied_adjacent(input, to_check) >= tolerance {
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
    use super::{calculcate_visible_seats, day11_part1, day11_part2, input_generator, Coordinate};
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
    fn test_given_part_1_f() {
        let map = input_generator(INPUT1);
        let res = day11_part1(&map);
        assert_eq!(res, 37)
    }
    #[test]
    fn test_given_part_2() {
        let map = input_generator(INPUT1);
        let res = day11_part2(&map);
        assert_eq!(res, 26)
    }

    #[test]
    fn test_calc_visible_seats_1() {
        let current = Coordinate { x: 3, y: 2 };
        let res = calculcate_visible_seats(&current, &Coordinate { x: -1, y: -1 }, 5, 4);
        assert_eq!(
            vec![Coordinate { x: 2, y: 1 }, Coordinate { x: 1, y: 0 }],
            res
        );
    }

    #[test]
    fn test_calc_visible_seats_2() {
        let current = Coordinate { x: 3, y: 2 };
        let res = calculcate_visible_seats(&current, &Coordinate { x: 1, y: -1 }, 5, 4);
        assert_eq!(vec![Coordinate { x: 4, y: 1 }], res);
    }
}
