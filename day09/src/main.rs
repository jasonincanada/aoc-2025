// https://adventofcode.com/2025/day/9

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    red_tiles: Vec<(u32, u32)>
}

fn part1(input: &Input) -> usize {
    let mut largest_area = 0 as usize;

    for tile1 in &input.red_tiles {
    for tile2 in &input.red_tiles {
        let area = area_between(*tile1, *tile2);
        if area > largest_area {
            largest_area = area;
        }
    }}

    largest_area
}


fn part2(_input: &Input) -> u64 {
    0
}


// ChatGPT 5.1 mostly
fn area_between(tile1: (u32, u32), tile2: (u32, u32)) -> usize {
    let dx = tile1.0 as i64 - tile2.0 as i64;
    let dy = tile1.1 as i64 - tile2.1 as i64;

    (dx.abs() as usize + 1) * (dy.abs() as usize + 1)
}


/* Parsing */

// ChatGPT 5.1 mostly
fn parse_input(path: &str) -> Input {
    let red_tiles = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split(',')
                             .map(|s| s.parse::<u32>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    Input { red_tiles }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(50, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(24, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(4771532800, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(0, part2(&input))
    }

}
