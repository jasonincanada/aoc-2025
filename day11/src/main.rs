// https://adventofcode.com/2025/day/11

fn main() {
    let sample = parse_input("sample.txt");
    let sample2= parse_input("sample2.txt");
    let input  = parse_input("input.txt");

    println!("Sample 1: {}", part1(&sample));
    println!("Sample 2: {}", part2(&sample2));
    println!("Part 1:   {}", part1(&input));
    println!("Part 2:   {}", part2(&input));
}

struct Input {
    devices: HashMap<String, Vec<String>>
}

fn part1(input: &Input) -> u64 {
    let mut memos: HashMap<String, u64> = HashMap::new();

    go( "you",
        &input.devices,
        &mut memos )
}

// this is similar to day 7 part 2 (Laboratories) but with n sub-nodes instead of exactly 2
fn go(focus  : &str,
      devices: &HashMap<String, Vec<String>>,
      memos  : &mut HashMap<String, u64>) -> u64
{
    if focus == "out" {
        return 1
    }

    if let Some(&path_count) = memos.get(focus) {
        return path_count
    }

    // recurse through all the outputs, summing the resulting path counts
    let path_count =
        devices.get(focus)
               .unwrap()
               .iter()
               .map(|device| go(device, devices, memos))
               .sum();

    memos.insert(focus.to_string(), path_count);

    path_count
}

#[derive(Clone, Copy)]
struct Memo {
    neither: u64,
    dac_only: u64,
    fft_only: u64,
    dac_and_fft: u64
}

fn part2(input: &Input) -> u64 {
    let mut memos: HashMap<String, Memo> = HashMap::new();

    let memo = go2( "svr",
                    &input.devices,
                    &mut memos );

    memo.dac_and_fft
}

fn go2(focus  : &str,
       devices: &HashMap<String, Vec<String>>,
       memos  : &mut HashMap<String, Memo>) -> Memo
{
    if focus == "out" {
        return Memo {
            neither    : 1,
            dac_only   : 0,
            fft_only   : 0,
            dac_and_fft: 0,
        }
    }

    if let Some(&memo) = memos.get(focus) {
        return memo.clone()
    }

    let mut memo = Memo {
        neither    : 0,
        dac_only   : 0,
        fft_only   : 0,
        dac_and_fft: 0,
    };

    // how we aggregate the recursive sub-memos depends on whether we're at one of the distinguished nodes
    match focus
    {
        "dac" => for device in devices.get(focus).unwrap()
                 {
                     let sub = go2(device, devices, memos);
                     memo.dac_only    += sub.neither;
                     memo.dac_and_fft += sub.dac_and_fft + sub.fft_only;
                 },

        "fft" => for device in devices.get(focus).unwrap()
                 {
                     let sub = go2(device, devices, memos);
                     memo.fft_only    += sub.neither;
                     memo.dac_and_fft += sub.dac_and_fft + sub.dac_only;
                 },

          _   => for device in devices.get(focus).unwrap()
                 {
                     let sub = go2(device, devices, memos);
                     memo.neither     += sub.neither;
                     memo.dac_only    += sub.dac_only;
                     memo.fft_only    += sub.fft_only;
                     memo.dac_and_fft += sub.dac_and_fft;
                 }
    }

    memos.insert(focus.to_string(), memo);

    memo
}

use std::collections::HashMap;


/* Parsing */

// ChatGPT 5.1
fn parse_input(path: &str) -> Input {
    let contents = std::fs::read_to_string(path)
        .expect("File is guaranteed to exist and be valid");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines() {

        // Split "key: v1 v2 v3"
        let (key, values_part) = line
            .split_once(':')
            .expect("Every line must contain ':'");

        let key = key.trim().to_string();

        let values: Vec<String> = values_part
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert(key, values);
    }

    Input { devices: map }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(5, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample2.txt");
        assert_eq!(2, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(640, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(367579641755680, part2(&input))
    }

}

/*
    $ time ./target/release/day11.exe
    Sample 1: 5
    Sample 2: 2
    Part 1:   640
    Part 2:   367579641755680

    real    0m0.021s
    user    0m0.000s
    sys     0m0.000s
*/
