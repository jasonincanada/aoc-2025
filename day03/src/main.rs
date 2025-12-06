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

fn part2(input: &Input) -> u64 {
    input.banks
         .iter()
         .map(|bank| find_highest_joltage_k(bank, 12))
         .sum()
}

// the naive approach, hard-coded for only 2 battery selections, useful for part 1 only
fn find_highest_joltage(bank: &Vec<u8>) -> u8 {
    let mut highest = 0;

    for (i, battery) in bank.iter().enumerate() {

        // skip this whole row if the leading digit is already lower than the highest
        if battery * 10 < highest {
            continue
        }

        for b in bank.iter().skip(i+1) {
            if battery * 10 + b > highest {
                highest = battery * 10 + b
            }
        }
    }

    highest 
}

// for part 2 we need a more general approach to compute for k battery selections
fn find_highest_joltage_k(bank: &Vec<u8>, k: usize) -> u64 {

    let mut joltage   = 0 as u64;
    let mut left_edge = 0 as usize;
    let     n         = bank.len();

    for i in 0..k {
        // narrow our search to this slice
        let slice = &bank[left_edge ..= n-k+i];

        // get the index of the maximum element, taking the left-most of all matches if there are more than one
        let index = find_max_l(slice);

        joltage *= 10;
        joltage += slice[index] as u64;

        left_edge += index + 1;
    }

    joltage
}

// find the index of the largest element in this slice, working right to left,
// taking the left-most if there are multiple copies of the maximum
fn find_max_l(slice: &[u8]) -> usize {

    if slice.is_empty() {
        core::panic!("expected a non-empty slice")
    }

    // start with the far right element and consider that the highest we've seen so far
    let mut max_index = slice.len() - 1;
    let mut max_value = slice[max_index];

    for (i, &val) in slice.iter().enumerate().rev().skip(1) {
        if val >= max_value {
            max_index = i;
            max_value = val;
        }
    }

    max_index
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
        assert_eq!(168575096286051, part2(&input))
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

    #[test]
    fn test_find_highest_joltage_k() {
        let input = parse_input("sample.txt");

        assert_eq!(find_highest_joltage_k(&input.banks[0], 12), 987654321111);
        assert_eq!(find_highest_joltage_k(&input.banks[1], 12), 811111111119);
        assert_eq!(find_highest_joltage_k(&input.banks[2], 12), 434234234278);
        assert_eq!(find_highest_joltage_k(&input.banks[3], 12), 888911112111);
    }

    // ChatGPT 5.1 below here

    #[test]
    fn finds_max_in_single_element_slice() {
        let slice = [42u8];
        assert_eq!(find_max_l(&slice), 0);
    }

    #[test]
    fn finds_max_in_strictly_increasing_slice() {
        let slice = [1u8, 2, 3, 4, 5];
        assert_eq!(find_max_l(&slice), 4);
    }

    #[test]
    fn finds_max_in_strictly_decreasing_slice() {
        let slice = [9u8, 7, 5, 3, 1];
        assert_eq!(find_max_l(&slice), 0);
    }

    #[test]
    fn finds_leftmost_max_when_there_are_duplicates() {
        // max value 9 appears at indices 1 and 3; function should return 1
        let slice = [2u8, 9, 3, 9, 1];
        assert_eq!(find_max_l(&slice), 1);
    }

    #[test]
    fn all_elements_equal_returns_index_zero() {
        let slice = [7u8, 7, 7, 7];
        assert_eq!(find_max_l(&slice), 0);
    }

    #[test]
    fn max_at_the_end_is_found() {
        let slice = [0u8, 1, 2, 3, 255];
        assert_eq!(find_max_l(&slice), 4);
    }

    #[test]
    #[should_panic(expected = "expected a non-empty slice")]
    fn panics_on_empty_slice() {
        let slice: &[u8] = &[];
        let _ = find_max_l(slice);
    }
}
