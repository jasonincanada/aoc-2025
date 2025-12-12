// https://adventofcode.com/2025/day/12

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    gift_shapes: Vec<GiftShape>,
    regions: Vec<Region>
}

struct GiftShape {
    tiles: Vec<Vec<char>>
}

struct Region {
    width: u32,
    height: u32,
    gift_counts: Vec<u32>
}

fn part1(input: &Input) -> usize {
    input.regions
         .iter()
         .filter(|region| can_arrange_gifts(&input.gift_shapes, region))
         .count()
}

// testing a theory based on a hint i read on r/adventofcode...
fn can_arrange_gifts(gift_shapes: &[GiftShape], region: &Region) -> bool {

    let total_octothorpes =
        region.gift_counts
              .iter()
              .enumerate()
              .map(|(gift, count)| gift_shapes[gift].count_octothorpes() * count)
              .sum::<u32>();

    let region_area = region.width * region.height;

    total_octothorpes <= region_area
}

impl GiftShape {
    fn count_octothorpes(&self) -> u32 {
        self.tiles
            .iter()
            .map(|row| row.iter()
                          .filter(|&&tile| tile == '#')
                          .count() as u32)
            .sum()
    }
}

fn part2(_input: &Input) -> u64 {
    0
}


/* Parsing */

fn parse_input(path: &str) -> Input {
    let contents = std::fs::read_to_string(path).unwrap();
    let contents = contents.replace("\r\n", "\n");
    let (top, bottom) = contents.rsplit_once("\n\n").unwrap();

    Input {
        gift_shapes: top.split("\n\n")
                        .map(parse_gift_shape)
                        .collect(),

        regions: bottom.lines()
                       .map(parse_region)
                       .collect()
    }
}

fn parse_gift_shape(ascii: &str) -> GiftShape {
    GiftShape {
        tiles: ascii.lines()
                    .skip(1)
                    .map(|line| line.chars().collect())
                    .collect()
    }
}

fn parse_region(ascii: &str) -> Region {
    let (left, right) = ascii.split_once(": ").unwrap();
    let (width, height) = left.split_once("x").unwrap();

    Region {
        width : width.parse().unwrap(),
        height: height.parse().unwrap(),

        gift_counts: right.split_whitespace()
                          .map(|digits| digits.parse().unwrap())
                          .collect()
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    // this test fails because our assumption that worked for our input isn't valid for the sample
    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(2, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(0, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(510, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(0, part2(&input))
    }

}
