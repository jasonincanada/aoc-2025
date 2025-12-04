// https://adventofcode.com/2025/day/4

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    grid: Vec<Vec<char>>
}

fn part1(input: &Input) -> usize {
    accessible_rolls(&input.grid)
        .len()
}

fn part2(input: &Input) -> usize {
    
    let mut grid = input.grid.clone();
    let mut count_rolls = 0;

    loop {
        let rolls = accessible_rolls(&grid);
        if rolls.is_empty() {
            break
        }
        count_rolls += rolls.len();
        remove_rolls(rolls, &mut grid);
    }

    count_rolls
}

// find the coordinates of the rolls that have fewer than 4 rolls around them
fn accessible_rolls(grid: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    
    let mut coords = vec![];

    for (r, row) in grid.iter().enumerate() {
    for (c, tile) in row.iter().enumerate() {

        if *tile != '@' {
            continue
        }

        let count = neighbours(r, c, &grid)
            .into_iter()
            .filter(|(row, col)| grid[*row][*col] == '@')
            .count();

        // if there are fewer than 4 rolls in our neighbourhood, remember this coordinate
        if count < 4 {
            coords.push((r,c));
        }
    }}

    coords
}

fn remove_rolls(coords: Vec<(usize, usize)>,
                grid  : &mut Vec<Vec<char>>)
{
    for coord in coords.into_iter() {
        grid[coord.0][coord.1] = '.'
    }
}

// get the coordinates of cells neighbouring (row, col)
fn neighbours(row: usize, col: usize, grid: &Vec<Vec<char>>) -> Vec<(usize,usize)> {

    let height = grid.len() as isize;
    let width  = grid[0].len() as isize;
    let row    = row as isize;
    let col    = col as isize;

    vec![ (-1,-1), (-1,0), (-1,1),
          ( 0,-1),         ( 0,1),        // (row, col) offsets of our neighbours
          ( 1,-1), ( 1,0), ( 1,1) ]

        .into_iter()
        .map(|(dr, dc)| (row + dr, col + dc))
        .filter(|(row, col)| *row >= 0 && *row < height     // stay on the grid
                          && *col >= 0 && *col < width)
        .map(|(row, col)| (row as usize, col as usize))
        .collect()
}


/* Parsing */

use std::fs;

// ChatGPT 5.1
fn parse_input(filename: &str) -> Input {
    // Read the whole file into a string
    let contents = fs::read_to_string(filename).unwrap();

    // For each line, collect its characters into a Vec<char>
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Input { grid }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(13, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(43, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(1416, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(9086, part2(&input))
    }

}
