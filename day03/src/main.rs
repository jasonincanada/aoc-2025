// https://adventofcode.com/2025/day/3

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    banks: Vec<Vec<u8>>
}

fn part1(input: &Input) -> u64 {
    input.banks
         .iter()
         .map(|bank| find_highest_joltage(bank) as u64)
         .sum()
}

fn part2(_input: &Input) -> u64 {
    0
}

fn find_highest_joltage(bank: &Vec<u8>) -> u8 {
    let mut highest = 0;

    for (i, battery) in bank.iter().enumerate() {

        // skip this whole row if the leading digit is already lower than the highest
        if battery * 10 < highest {
            continue
        }

        for j in i+1 .. bank.len() {
            if battery * 10 + bank[j] > highest {
                highest = battery * 10 + bank[j]
            }
        }
    }

    highest 
}


/* Parsing */

use std::fs;

fn parse_input(path: &str) -> Input {

    let banks = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| b - b'0')
                .collect::<Vec<u8>>()
        })
        .collect();

    Input { banks }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(357, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(3121910778619, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(17031, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(0, part2(&input))
    }

    #[test]
    fn test_find_highest_joltage() {
        let input = parse_input("sample.txt");

        /*
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        */

        assert_eq!(find_highest_joltage(&input.banks[0]), 98);
        assert_eq!(find_highest_joltage(&input.banks[1]), 89);
        assert_eq!(find_highest_joltage(&input.banks[2]), 78);
        assert_eq!(find_highest_joltage(&input.banks[3]), 92);
    }

}
