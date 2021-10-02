use std::convert::TryInto;

const IMAGE_SIDE_LENGTH: usize = 10;
type RawImage = [[char; IMAGE_SIDE_LENGTH]; IMAGE_SIDE_LENGTH];

fn calculate_sides(image: &RawImage) -> (u32, u32, u32, u32) {
    fn convert(image: &RawImage, x: usize, y: usize) -> u32 {
        match image[y][x] {
            '.' => 0,
            '#' => 1,
            _ => panic!("Unexpected character in image"),
        }
    }

    let mut north = 0u32;
    let mut east = 0u32;
    let mut south = 0u32;
    let mut west = 0u32;
    for i in 0..IMAGE_SIDE_LENGTH {
        let north_converted = convert(image, i, 0);
        let east_converted = convert(image, IMAGE_SIDE_LENGTH - 1, i);
        let south_converted = convert(image, i, IMAGE_SIDE_LENGTH - 1);
        let west_converted = convert(image, 0, i);

        north += north_converted * 2u32.pow(i as u32);
        east += east_converted * 2u32.pow(i as u32);
        south += south_converted * 2u32.pow(i as u32);
        west += west_converted * 2u32.pow(i as u32);
    }

    (north, east, south, west)
}

struct Image {
    number: u32,
    raw_image: RawImage,
    north: u32,
    east: u32,
    south: u32,
    west: u32,
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Image> {
    let v = vec!['a', 'b', 'c', 'd'];
    let s: [char; 4] = v.as_slice().try_into().unwrap();

    let images = input.split("\n\n");

    let mut parsed_images = Vec::new();
    for image in images.take(1) {
        let mut image_lines = image.lines().map(str::trim);
        let number_line = image_lines.next().unwrap().split(" ").collect::<Vec<_>>()[1];
        let number: u32 = number_line[..number_line.len() - 1].parse().unwrap();

        let raw_image: RawImage = image_lines
            .map(|l| {
                let line_characters: [char; IMAGE_SIDE_LENGTH] =
                    l.chars().collect::<Vec<_>>().as_slice().try_into().unwrap();
                return line_characters;
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        let sides = calculate_sides(&raw_image);
        let i = Image {
            number,
            raw_image,
            north: sides.0,
            east: sides.1,
            south: sides.2,
            west: sides.3,
        };
        parsed_images.push(i);
    }

    parsed_images
}

#[aoc(day20, part1)]
fn day20_part1(images: &Vec<Image>) -> u64 {
    123
}

#[cfg(test)]
mod tests {
    use super::input_generator;

    #[test]
    fn test_parse_image() {
        let input = "Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###";

        let generated = input_generator(input);
        let first = &generated[0];
        assert_eq!(first.number, 2311);
        assert_eq!(first.north, 300);
        assert_eq!(first.east, 616);
        assert_eq!(first.south, 924);
        assert_eq!(first.west, 318);
    }
}
