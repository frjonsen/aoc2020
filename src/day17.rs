use itertools::Itertools;
use std::convert::TryInto;
#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(str::trim)
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unexpected character {} in input", c),
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}

#[derive(PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

fn calculate_neighbors(coord: &Coordinate, edge: usize) -> Vec<Coordinate> {
    let mut neighbors = vec![];

    let modifiers = &[-1, 1];
    let e = edge.try_into().unwrap();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                if coord.x + x < 0 || coord.x + x >= e {
                    continue;
                }
                if coord.y + y < 0 || coord.y + y >= e {
                    continue;
                }
                if coord.z + z < 0 || coord.z + z >= e {
                    continue;
                }
                neighbors.push(Coordinate {
                    x: coord.x + x,
                    y: coord.y + y,
                    z: coord.z + z,
                });
            }
        }
    }

    neighbors
}

fn print_cube_slice(cube: &[Vec<Vec<bool>>], max: usize, z: usize) {
    for x in 0..max {
        for y in 0..max {
            let c = match cube[x][y][z] {
                false => '.',
                true => '#',
            };
            print!("{}", c);
        }
        println!();
    }
}

#[aoc(day17, part1)]
fn day17_part1(initial_state: &[Vec<bool>]) -> usize {
    let max: usize = (initial_state.len() + 6) * 2;

    let mut cube = vec![vec![vec![false; max]; max]; max];
    for x in 0..initial_state.len() {
        for y in 0..initial_state[x].len() {
            cube[x + max / 2][y + max / 2][max / 2] = initial_state[x][y];
        }
    }

    for _ in 0..6 {
        let mut new_cube = vec![vec![vec![false; max]; max]; max];

        for x in 0..max {
            for y in 0..max {
                for z in 0..max {
                    let current_neighbors = calculate_neighbors(
                        &Coordinate {
                            x: x as i64,
                            y: y as i64,
                            z: z as i64,
                        },
                        max,
                    );
                    let neighbor_active = current_neighbors
                        .into_iter()
                        .map(|c| cube[c.x as usize][c.y as usize][c.z as usize])
                        .filter(|c| *c)
                        .count();
                    if cube[x][y][z] {
                        new_cube[x][y][z] = neighbor_active == 2 || neighbor_active == 3;
                    } else {
                        new_cube[x][y][z] = neighbor_active == 3;
                    }
                }
            }
        }
        cube = new_cube;
    }

    print_cube_slice(&cube, max, max / 2);

    let mut sum = 0;
    for x in 0..max {
        for y in 0..max {
            for z in 0..max {
                if cube[x][y][z] {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{calculate_neighbors, day17_part1, input_generator, Coordinate};

    #[test]
    fn test_day17_part_1_given() {
        let input = ".#.
        ..#
        ###";
        let generated = input_generator(input);
        let res = day17_part1(&generated);
        assert_eq!(res, 112);
    }

    #[test]
    fn test_calculate_neighbors() {
        let coordinate = Coordinate { x: 1, y: 1, z: 1 };

        let res = calculate_neighbors(&coordinate, 5);
        let mut expected = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    expected.push(Coordinate {
                        x: 1 + x,
                        y: 1 + y,
                        z: 1 + z,
                    });
                }
            }
        }
        assert_eq!(res.len(), 26);
        assert!(expected.into_iter().all(|ref e| res.contains(e)));
    }
}
