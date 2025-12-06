// https://adventofcode.com/2025/day/2

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    intervals: Vec<(u64,u64)>
}

fn part1(input: &Input) -> u64 {
    input.intervals
         .iter()
         .map(|(from, to)| sum_invalid_numbers_between(*from, *to))
         .sum()
}

fn part2(_input: &Input) -> u64 {
    0
}

// store just one half of the InvalidNumber, there's no point in storing the same number twice
#[derive(Debug, PartialEq)]
struct InvalidNumber(u64);

// if the number passed is not an invalid number, find the next one lower than it. if it's
// already an invalid number, return it
fn next_lower_invalid_number(n: u64) -> Option<InvalidNumber> {

    // there's no invalid number less than 11
    if n < 11 {
        return None
    }

    let num_digits = digits(n);

    // any odd-number of digits must be mapped down to the 9999 below it (ie: f(123) -> 99)
    if !num_digits.is_multiple_of(2) {
        let exp = (num_digits - 1) / 2;
        return Some(InvalidNumber(10_u64.pow(exp) - 1))
    }

    let (left, right) = split_number_in_half(n);

    // is this already an invalid number (left and right halves match)
    if left == right {
        return Some(InvalidNumber(left));
    }

    // if the right half is greater than the left, bring the right half down to match the left
    // f(12347777) = 12341234
    else if right > left {
        return Some(InvalidNumber(left))
    }

    // f(12341200) = 12331233
    else if right < left {
        return Some(InvalidNumber(left - 1))
    }

    None
}

fn next_higher_invalid_number(n: u64) -> InvalidNumber {

    let num_digits = digits(n);

    // only when there are an even number of digits can the number be a smaller number duplicated
    if !num_digits.is_multiple_of(2) {
        return InvalidNumber(10_u64.pow(num_digits / 2))
    }

    let (left, right) = split_number_in_half(n);

    // is this already an invalid number (left and right halves match)
    if left == right {
        return InvalidNumber(left)
    }

    // if the right half is greater than the left, bring the left half up to match the right
    // f(12347777) = 12351235
    if right > left {
        return InvalidNumber(left + 1)
    }

    // f(12341200) = 12341234
    InvalidNumber(left)
}

// count the number of invalid numbers between from and to, including the ends
fn count_invalid_numbers_between(from: u64, to: u64) -> u64 {
    
    let next_higher = next_higher_invalid_number(from);
    let next_lower  = next_lower_invalid_number(to).expect("to should have a next lower invalid number");

    if next_lower.0 < next_higher.0 {
        0
    } else {
        next_lower.0 - next_higher.0 + 1
    }
}

fn sum_invalid_numbers_between(from: u64, to: u64) -> u64 {
    let next_higher = next_higher_invalid_number(from);
    let next_lower  = next_lower_invalid_number(to).expect("to should have a next lower invalid number");

    if next_lower.0 < next_higher.0 {
        0
    } else {
        (next_higher.0 ..= next_lower.0)
            .map(|x| expand(InvalidNumber(x)))
            .sum()
    }
}

// map an InvalidNumber to its u64; eg. Invalid(123) -> 123123
fn expand(invalid: InvalidNumber) -> u64 {
      invalid.0
    + invalid.0 * 10_u64.pow(digits(invalid.0))
}


/* Helper Functions (ChatGPT 5.1 with mods by me) */

// count number of digits in a number
fn digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

/// Splits an integer into its left and right halves **if** it has an even
/// number of digits. panics for odd digit lengths.
/// 
/// Examples:
///   split_number(1234)   → Some((12, 34))
///   split_number(567890) → Some((567, 890))
///   split_number(999)    → panic
/// 
fn split_number_in_half(n: u64) -> (u64, u64) {
    
    // Count digits
    let mut digits = 0;
    let mut tmp = n;
    while tmp > 0 {
        digits += 1;
        tmp /= 10;
    }

    if digits % 2 != 0 {
        panic!("must have an even number of digits")
    }

    let half = digits / 2;
    let pow10 = 10_u64.pow(half);

    let left  = n / pow10;
    let right = n % pow10;

    (left, right)
}


/* Parsing */

use std::fs;

// mostly ChatGPT 5.1
fn parse_input(path: &str) -> Input {

    // Parse "a-b,c-d,..." into Vec<(i64, i64)>
    let intervals = fs::read_to_string(path)
        .unwrap()
        .trim() // remove trailing newlines/whitespace
        .split(',') // split on commas
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts
                .next()
                .expect("missing start")
                .trim()
                .parse::<u64>()
                .expect("invalid start number");
            let end = parts
                .next()
                .expect("missing end")
                .trim()
                .parse::<u64>()
                .expect("invalid end number");
            (start, end)
        })
        .collect();

    Input { intervals }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(1227775554, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(4174379265, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(19128774598, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(0, part2(&input))
    }

    #[test]
    fn test_next_lower_invalid_number() {
        assert_eq!(next_lower_invalid_number(10), None);
        assert_eq!(next_lower_invalid_number(11), Some(InvalidNumber(1)));
        assert_eq!(next_lower_invalid_number(12), Some(InvalidNumber(1)));
        assert_eq!(next_lower_invalid_number(12347777), Some(InvalidNumber(1234)));
        assert_eq!(next_lower_invalid_number(12341200), Some(InvalidNumber(1233)));
        assert_eq!(next_lower_invalid_number(123), Some(InvalidNumber(9)));
        assert_eq!(next_lower_invalid_number(12345), Some(InvalidNumber(99)));
    }

    #[test]
    fn test_next_higher_invalid_number() {
        assert_eq!(next_higher_invalid_number(0), InvalidNumber(1));            // 11
        assert_eq!(next_higher_invalid_number(1), InvalidNumber(1));            // 11
        assert_eq!(next_higher_invalid_number(10), InvalidNumber(1));           // 11
        assert_eq!(next_higher_invalid_number(123), InvalidNumber(10));         // 1010
        assert_eq!(next_higher_invalid_number(12345), InvalidNumber(100));      // 100100
        assert_eq!(next_higher_invalid_number(12347777), InvalidNumber(1235));  // 12351235
        assert_eq!(next_higher_invalid_number(12341200), InvalidNumber(1234));  // 12341234
    }
    
    #[test]
    fn test_count_invalid_numbers_between() {
        assert_eq!(count_invalid_numbers_between(11,22), 2);
        assert_eq!(count_invalid_numbers_between(95,115), 1);
        assert_eq!(count_invalid_numbers_between(1188511880,1188511890), 1);
        assert_eq!(count_invalid_numbers_between(998,1012), 1);
        assert_eq!(count_invalid_numbers_between(222220,222224), 1);
        assert_eq!(count_invalid_numbers_between(446443,446449), 1);
        assert_eq!(count_invalid_numbers_between(38593856,38593862), 1);
    }

    #[test]
    fn test_sum_invalid_numbers_between() {
        assert_eq!(sum_invalid_numbers_between(11,22), 33);
        assert_eq!(sum_invalid_numbers_between(95,115), 99);
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand(InvalidNumber(1)), 11);
        assert_eq!(expand(InvalidNumber(2)), 22);
        assert_eq!(expand(InvalidNumber(9)), 99);
        assert_eq!(expand(InvalidNumber(11885)), 1188511885);
        assert_eq!(expand(InvalidNumber(10)), 1010);
    }
}
