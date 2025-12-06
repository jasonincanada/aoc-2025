// https://adventofcode.com/2025/day/6

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

fn part1(input: &Input) -> u64 { calculate(input).0 }
fn part2(input: &Input) -> u64 { calculate(input).1 }

fn calculate(input: &Input) -> (u64, u64)
{
    let blocks: Vec<Block> = parse_grid_into_blocks(&input.grid);

    let mut horiz = 0;
    let mut vert  = 0;

    for block in blocks.iter() {
        if block.symbol == '*' {
            horiz += block.horizontal_numbers.iter().product::<u64>();
            vert  += block.vertical_numbers  .iter().product::<u64>();
        } else {
            horiz += block.horizontal_numbers.iter().sum::<u64>();
            vert  += block.vertical_numbers  .iter().sum::<u64>();
        }
    }

    (horiz, vert)
}

// one symbol with both its horizontal numbers and vertical numbers
struct Block {
    symbol: char,
    horizontal_numbers: Vec<u64>,
    vertical_numbers  : Vec<u64>
}

// strategy:
//
//  locate symbols
//  for each symbol:
//      get horizontal numbers
//      get vertical numbers (number of vertical numbers determined by longest horizontal number)
//
fn parse_grid_into_blocks(grid: &Vec<Vec<char>>) -> Vec<Block>
{
    // find the locations of the symbols + and * on the last row
    let symbols = grid.last()
                      .unwrap()
                      .iter()
                      .enumerate()
                      .filter(|&(_, symbol)| *symbol == '+' ||
                                             *symbol == '*');
    let mut blocks = vec![];

    for (col, symbol) in symbols
    {
        let mut horizontal_numbers = vec![];
        let mut vertical_numbers   = vec![];

        for row in 0 .. grid.len() - 1 {
            horizontal_numbers.push(
                get_horizontal_number(row, col, grid)
            );
        }

        // here we can benefit a bit from parsing the horizontal/vertical number together: we know
        // how many columns of numbers to get because it's the size of the largest horizontal number
        let column_count = horizontal_numbers.iter()
                                             .map(|&number| num_digits(number))
                                             .max()
                                             .unwrap();

        for column in col .. col+column_count {
            vertical_numbers.push(
                get_vertical_number(column, grid)
            );
        }

        blocks.push(Block {
            symbol: *symbol,
            horizontal_numbers,
            vertical_numbers
        });
    }
                                             
    blocks
}

// parse the horizontal number starting at a row/col on the grid (potentially starting with spaces)
fn get_horizontal_number(row: usize, column: usize, grid: &Vec<Vec<char>>) -> u64
{
    let digit_string =
        grid[row][column..]
            .iter()
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();

    digit_string.parse().unwrap()
}

// parse the vertical number from the digits in a given column
fn get_vertical_number(column: usize, grid: &Vec<Vec<char>>) -> u64
{
    let digit_string =
        (0 .. grid.len()-1)                         // the range of row offsets, except for the last row
            .map(|row| grid[row][column])           // get the char at this row (the column is fixed)
            .filter(|char| !char.is_whitespace())   // ignore blanks
            .collect::<String>();                   // collect individual chars into a string

    digit_string.parse().unwrap()
}

// count number of digits in a number
fn num_digits(n: u64) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as usize + 1
    }
}


/* Parsing */

// collect the chars into a 2D grid, parsing into blocks happens later
fn parse_input(path: &str) -> Input {
    let grid = std::fs::read_to_string(path)
        .expect("file should exist")
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
        assert_eq!(4277556, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(3263827, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(6503327062445, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(9640641878593, part2(&input))
    }

}
