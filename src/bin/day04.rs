use std::io::Read;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn nof_neighbors(grid: &[Vec<u8>], row: usize, col: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut result = 0;

    for y in row.saturating_sub(1)..height.min(row+2) {
        for x in col.saturating_sub(1)..width.min(col+2) {
            if (y != row || x != col) && grid[y][x] == b'@' {
                result += 1;
            }
        }
    }

    result
}

fn part1(grid: &[Vec<u8>]) -> usize {
    let mut result = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'@' && nof_neighbors(grid, row, col) < 4 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");

    let grid = parse_input(&input);

    println!("Part 1: {}", part1(&grid));
}

#[test]
fn test_part1() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let grid = parse_input(&input);

    assert_eq!(nof_neighbors(&grid, 0, 2), 3);
    assert_eq!(part1(&grid), 13);
}