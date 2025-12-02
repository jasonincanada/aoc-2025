// https://adventofcode.com/2025/day/1

fn main() {
    let sample = parse_rotations("sample.txt");
    let input  = parse_rotations("input.txt");

    println!("Sample part 1: {}", part1(&sample)); // 3
    println!("Sample part 2: {}", part2(&sample)); // 6
    println!("Part 1:        {}", part1(&input));  // 1081
    println!("Part 2:        {}", part2(&input));  // 6689
}

struct Input {
    rotations: Vec<i32>
}

fn part1(input: &Input) -> i32 {
    let mut dial = 50;
    let mut count_zeros = 0;

    for rotation in &input.rotations {
        dial = (dial + rotation) % 100;
        if dial == 0 {
            count_zeros += 1;
        }
    }

    count_zeros
}

fn part2(input: &Input) -> i32 {
    let mut dial = 50 as i32;
    let mut count_zeros = 0;

    for rotation in &input.rotations {

        let dial_before = dial;
        let hundreds_before = dial_before.div_euclid(100);

        dial = dial + rotation;

        let hundreds_now  = dial.div_euclid(100);
        let hundreds_diff = (hundreds_now - hundreds_before).abs();

        if rotation > &0 {
            count_zeros += hundreds_diff;
        }

        if rotation < &0 {
            count_zeros += hundreds_diff;

            if dial_before % 100 == 0 {
                count_zeros -= 1;
            }

            if dial % 100 == 0 {
                count_zeros += 1;
            }
        }
    }

    count_zeros
}

// 7937 too high
// 6777 too high
// 4711 too low


/* Parsing */

use std::fs;

// ChatGPT 5.1
fn parse_rotations(filename: &str) -> Input {
    let rotations = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);        // ("R", "10") or ("L", "3")
            let value: i32 = num.parse().unwrap();    // parse digits only

            match dir {
                "R" => value,
                "L" => -value,
                _ => unreachable!(),                  // input always valid
            }
        })
        .collect();

    Input {
        rotations
    }
}
