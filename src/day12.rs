enum Instruction {
    North(i64),
    West(i64),
    East(i64),
    South(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

struct Coordinate {
    x: i64,
    y: i64,
}

impl Ship {
    pub fn travel(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(_)
            | Instruction::East(_)
            | Instruction::West(_)
            | Instruction::South(_) => self.travel_by_direction(instruction),
            Instruction::Left(_) | Instruction::Right(_) => self.rotate(instruction),
            Instruction::Forward(d) => self.travel_by_facing(d),
        }
    }

    fn travel_by_direction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(d) => self.position.y += d,
            Instruction::South(d) => self.position.y -= d,
            Instruction::West(d) => self.position.x -= d,
            Instruction::East(d) => self.position.x += d,
            _ => panic!("Not a direction instruction"),
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        let angle = match instruction {
            Instruction::Right(d) => *d,
            Instruction::Left(d) => 360 - *d,
            _ => panic!("Not a rotation instruction"),
        } as u16;
        self.facing += angle;
        self.facing %= 360;
    }

    fn travel_by_facing(&mut self, distance: &i64) {
        let instruction = match self.facing {
            0 => Instruction::North(*distance),
            90 => Instruction::East(*distance),
            180 => Instruction::South(*distance),
            270 => Instruction::West(*distance),
            _ => panic!(format!("{} is not an even angle", self.facing)),
        };
        self.travel_by_direction(&instruction);
    }
}

struct Ship {
    pub facing: u16,
    pub position: Coordinate,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            facing: 90,
            position: Coordinate { x: 0, y: 0 },
        }
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .into_iter()
        .map(&str::trim)
        .map(|l| {
            let f: i64 = l[1..]
                .parse()
                .expect(&format!("Failed to parse instruction: {}", l));
            match &l[0..1] {
                "N" => Instruction::North(f),
                "W" => Instruction::West(f),
                "E" => Instruction::East(f),
                "S" => Instruction::South(f),
                "F" => Instruction::Forward(f),
                "R" => Instruction::Right(f),
                "L" => Instruction::Left(f),
                _ => panic!(format!("Unexpected instruction {}", &l)),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn day12_part1(input: &Vec<Instruction>) -> u64 {
    let mut ship = Ship::new();
    for i in input {
        ship.travel(i);
    }

    ship.position.x.abs() as u64 + ship.position.y.abs() as u64
}

#[cfg(test)]
mod tests {
    use super::{day12_part1, input_generator, Instruction, Ship};

    #[test]
    fn test_rotate_right() {
        let mut ship = Ship::new();
        ship.rotate(&Instruction::Right(90));
        assert_eq!(ship.facing, 180);
    }
    #[test]
    fn test_rotate_left() {
        let mut ship = Ship::new();
        ship.rotate(&Instruction::Left(180));
        assert_eq!(ship.facing, 270);
    }

    #[test]
    fn test_given_part_1() {
        let input = "F10
        N3
        F7
        R90
        F11";
        let instructions = input_generator(input);
        let res = day12_part1(&instructions);
        assert_eq!(res, 25);
    }
}
