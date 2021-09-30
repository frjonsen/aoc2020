use itertools::Itertools;
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
    parts: Vec<i64>,
}

fn calculate_neighbors(coord: &Coordinate, edge: usize, neighbors: &[Vec<i64>]) -> Vec<Coordinate> {
    let mut to_check = Vec::with_capacity(coord.parts.len().pow(coord.parts.len() as u32));

    let e = edge as i64;

    for parts in neighbors.iter() {
        if parts.iter().all(|p| *p == 0) {
            continue;
        }

        if coord
            .parts
            .iter()
            .enumerate()
            .any(|(i, v)| v + parts[i] < 0 || v + parts[i] >= e)
        {
            continue;
        }

        to_check.push(Coordinate {
            parts: parts
                .iter()
                .enumerate()
                .map(|(i, v)| coord.parts[i] + v)
                .collect(),
        });
    }

    to_check
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
    const DIM: usize = 3;

    let neighbor_checks: Vec<Vec<i64>> = (0..DIM)
        .map(|_| -1..=1)
        .multi_cartesian_product()
        .into_iter()
        .collect_vec();

    let cells = (0..DIM)
        .map(|_| 0..max)
        .multi_cartesian_product()
        .collect_vec();

    for _ in 0..6 {
        let mut new_cube = vec![vec![vec![false; max]; max]; max];

        for coord_parts in cells.iter() {
            let current_neighbors = calculate_neighbors(
                &Coordinate {
                    parts: coord_parts.iter().map(|c| *c as i64).collect(),
                },
                max,
                &neighbor_checks,
            );
            let neighbor_active = current_neighbors
                .into_iter()
                .map(|c| cube[c.parts[0] as usize][c.parts[1] as usize][c.parts[2] as usize])
                .filter(|c| *c)
                .count();
            if cube[coord_parts[0]][coord_parts[1]][coord_parts[2]] {
                new_cube[coord_parts[0]][coord_parts[1]][coord_parts[2]] =
                    neighbor_active == 2 || neighbor_active == 3;
            } else {
                new_cube[coord_parts[0]][coord_parts[1]][coord_parts[2]] = neighbor_active == 3;
            }
        }

        cube = new_cube;
    }

    (0..DIM)
        .map(|_| 0..max)
        .multi_cartesian_product()
        .filter(|c| cube[c[0]][c[1]][c[2]])
        .count()
}

#[aoc(day17, part2)]
fn day17_part2(initial_state: &[Vec<bool>]) -> usize {
    let max: usize = (initial_state.len() + 6) * 2;

    let mut cube = vec![vec![vec![vec![false; max]; max]; max]; max];
    for x in 0..initial_state.len() {
        for y in 0..initial_state[x].len() {
            cube[x + max / 2][y + max / 2][max / 2][max / 2] = initial_state[x][y];
        }
    }
    const DIM: usize = 4;

    let neighbor_checks: Vec<Vec<i64>> = (0..DIM)
        .map(|_| -1..=1)
        .multi_cartesian_product()
        .into_iter()
        .collect_vec();

    let cells = (0..DIM)
        .map(|_| 0..max)
        .multi_cartesian_product()
        .collect_vec();

    for _ in 0..6 {
        let mut new_cube = vec![vec![vec![vec![false; max]; max]; max]; max];

        for coord_parts in cells.iter() {
            let current_neighbors = calculate_neighbors(
                &Coordinate {
                    parts: coord_parts.iter().map(|c| *c as i64).collect(),
                },
                max,
                &neighbor_checks,
            );
            let neighbor_active = current_neighbors
                .into_iter()
                .map(|c| {
                    cube[c.parts[0] as usize][c.parts[1] as usize][c.parts[2] as usize]
                        [c.parts[3] as usize]
                })
                .filter(|c| *c)
                .count();
            if cube[coord_parts[0]][coord_parts[1]][coord_parts[2]][coord_parts[3]] {
                new_cube[coord_parts[0]][coord_parts[1]][coord_parts[2]][coord_parts[3]] =
                    neighbor_active == 2 || neighbor_active == 3;
            } else {
                new_cube[coord_parts[0]][coord_parts[1]][coord_parts[2]][coord_parts[3]] =
                    neighbor_active == 3;
            }
        }

        cube = new_cube;
    }

    (0..DIM)
        .map(|_| 0..max)
        .multi_cartesian_product()
        .filter(|c| cube[c[0]][c[1]][c[2]][c[3]])
        .count()
}
#[cfg(test)]
mod tests {
    use super::{day17_part1, input_generator};
    use itertools::Itertools;

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
    fn test_something() {
        let t = (0..3).map(|_| 0..18).multi_cartesian_product().count();
        assert_eq!(t, 18usize.pow(3));
    }
}
