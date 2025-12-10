// https://adventofcode.com/2025/day/7

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    manifold: Vec<Vec<char>>
}

fn part1(input: &Input) -> u64 {
    let s_column = get_s_column(&input.manifold);

    let mut beam_columns = vec![ s_column ];
    let mut split_count = 0;

    for row in 2 .. input.manifold.len() {
        let mut next_beams = vec![];

        for col in beam_columns {
            match input.manifold[row][col] {
                
                '.' => {
                    next_beams.push(col)
                },

                '^' => {
                    next_beams.push(col - 1);
                    next_beams.push(col + 1);
                    split_count += 1;
                },

                _ => panic!("unknown tile type")
            } 
        }

        next_beams.sort();
        next_beams.dedup();

        beam_columns = next_beams.into_iter().collect()
    }

    split_count
}


fn part2(input: &Input) -> u64 {
    // fmap (const None) manifold
    let mut memos: Vec<Vec<Option<u64>>> =
        input.manifold
             .iter()
             .map(|row| row.iter()
                           .map(|_| None)
                           .collect())
             .collect();
    
    go( 1,
        get_s_column(&input.manifold),
        &input.manifold,
        &mut memos)
}

fn go(row     : usize,
      col     : usize,
      manifold: &Vec<Vec<char>>,
      memos   : &mut Vec<Vec<Option<u64>>> ) -> u64
{
    if row == manifold.len() - 1 {
        return 1
    }

    // check right away if we've been to this tile before
    if let Some(memo) = memos[row][col] {
        return memo
    }

    let path_count = match manifold[row][col]
    {
        // we're at a splitter, so recurse left, right and add the results
        '^' => go(row+1, col-1, manifold, memos)
             + go(row+1, col+1, manifold, memos),

        // go down a row
        '.' => go(row+1, col, manifold, memos),

         _  => panic!("unknown tile type")
    };
    
    memos[row][col] = Some(path_count);

    path_count
}

fn get_s_column(manifold: &[Vec<char>]) -> usize {
    manifold[0]
        .iter()
        .enumerate()
        .find(|&(_, s)| *s == 'S')
        .unwrap()
        .0
}


/* Parsing */

use std::fs;

fn parse_input(path: &str) -> Input {

    let manifold = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    Input { manifold }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(21, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(40, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(1717, part1(&input))
    }

    // 3432 too low (1717*2-2)

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(231507396180012, part2(&input))
    }

}

/*
    $ time ./target/release/day07.exe
    Sample part 1: 21
    Sample part 2: 40
    Part 1:        1717
    Part 2:        231507396180012

    real    0m0.019s
    user    0m0.000s
    sys     0m0.000s
*/
