// https://adventofcode.com/2025/day/10

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample 1: {}", part1(&sample));
    println!("Sample 2: {}", part2(&sample));
    println!("Part 1:   {}", part1(&input));
    println!("Part 2:   {}", part2(&input));
}

struct Input {
    machines: Vec<Machine>
}

struct Machine {
    indicator_lights: String,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

fn part1(input: &Input) -> usize {
    let mut total_fewest_buttons = 0;
    
    let fewest_buttons =
        input.machines.iter()
                      .map(|machine| machine.buttons.len())
                      .min()
                      .unwrap();

    let most_buttons =
        input.machines.iter()
                      .map(|machine| machine.buttons.len())
                      .max()
                      .unwrap();

    for button_count in fewest_buttons ..= most_buttons
    {
        // as a pre-processing step, get all the bitstrings with {button_count} number of bits.
        // critically this is sorted by the number of 1s in the bitstring, so we may not have to
        // try all the possible button configurations; we can stop at the first match and it'll
        // be a minimum number of buttons to press to activate this machine's indicator lights
        let bitstrings: Vec<Vec<u8>> = get_bitstrings(button_count);

        let machines_with_this_many_buttons =
            input.machines.iter()
                          .filter(|m| m.buttons.len() == button_count);
                        
        for machine in machines_with_this_many_buttons {
            let indicator_target: u32 = indicator_lights_to_target(&machine.indicator_lights);

            for bitstring in &bitstrings {
                let lights_on: u32 = get_lights_after_pressing_buttons(bitstring, &machine.buttons);

                if lights_on == indicator_target {
                    total_fewest_buttons += bitstring.len();
                    break
                }
            }
        }
    }

    total_fewest_buttons
}

fn part2(_input: &Input) -> u64 {
    0
}


fn get_lights_after_pressing_buttons(button_indexes: &[u8], buttons: &Vec<Vec<u32>>) -> u32 {
    let mut lights_on = 0 as u32;

    for idx in button_indexes {
        for b in &buttons[*idx as usize] {
            lights_on ^= 1 << b;
        }
    }

    lights_on
}


/* ChatGPT 5.1 */

// Write this Rust function that takes a number like 3 and returns a vector of vecs of locations of
// the bits that are turned on for that number. The whole list should be returned sorted by the number
// of on bits (length of the inner Vec<u8>): fn get_bitstrings(n: usize) -> Vec<Vec<u8>> { }
fn get_bitstrings(n: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let total = 1usize << n;

    for mask in 0..total {
        let mut bits = Vec::new();

        for bit in 0..n {
            if (mask >> bit) & 1 == 1 {
                bits.push(bit as u8);
            }
        }

        result.push(bits);
    }

    // Sort by number of 1 bits
    result.sort_by_key(|v| v.len());

    result
}

// Write this Rust function that takes a string of . or # and constructs the corresponding integer
// with bits # set to on: fn indicator_lights_to_target(indicator_lights: &str) -> u32 { }
fn indicator_lights_to_target(indicator_lights: &str) -> u32 {
    let mut value = 0u32;

    for c in indicator_lights.chars().rev() {
        value <<= 1; // shift existing bits left

        if c == '#' {
            value |= 1; // set lowest bit
        }
    }

    value
}


/* Parsing (mostly ChatGPT 5.1) */

fn parse_input(path: &str) -> Input {
    let machines = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();

    Input { machines }
}

fn parse_line(line: &str) -> Machine {
    // [indicator] rest...
    let start = line.find('[').unwrap();
    let end = line[start..].find(']').unwrap() + start;

    // between [ and ]
    let indicator_lights = line[start + 1..end].to_string();

    // after the closing ]
    let rest = line[end + 1..].trim();

    let mut buttons: Vec<Vec<u32>> = Vec::new();
    let mut joltages: Vec<u32> = Vec::new();

    for token in rest.split_whitespace() {
        if token.starts_with('(') {
            buttons.push(parse_numbers(token, '(', ')'));
        } else if token.starts_with('{') {
            joltages = parse_numbers(token, '{', '}');
        }
    }

    Machine {
        indicator_lights,
        buttons,
        joltages,
    }
}

fn parse_numbers(token: &str, open: char, close: char) -> Vec<u32> {
    let inner = token
        .trim()
        .trim_start_matches(open)
        .trim_end_matches(close);

    if inner.is_empty() {
        return Vec::new();
    }

    inner
        .split(',')
        .map(|s| s.parse::<u32>().unwrap()) // input always valid
        .collect()
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(7, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(33, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(578, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(0, part2(&input))
    }

}

/*
    $ time ./target/release/day10.exe
    Sample 1: 7
    Sample 2: 0
    Part 1:   578
    Part 2:   0

    real    0m0.023s
    user    0m0.000s
    sys     0m0.000s
*/
