use std::fs;
use std::io::Error;

fn parse_input(content: &str) -> Result<Vec<Vec<char>>, Error> {
    Ok(content
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}



fn check_xmas_pattern(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    positions: &mut Vec<(usize, usize)>,
) {
    let n = grid.len();
    let m = grid[0].len();

    let forward_diagonal1 = i >= 1
        && i + 1 < n
        && j >= 1
        && j + 1 < m
        && grid[i - 1][j - 1] == 'M'
        && grid[i + 1][j + 1] == 'S';

    let backward_diagonal1 = i >= 1
        && i + 1 < n
        && j >= 1
        && j + 1 < m
        && grid[i - 1][j - 1] == 'S'
        && grid[i + 1][j + 1] == 'M';

    let forward_diagonal2 = i >= 1
        && i + 1 < n
        && j >= 1
        && j + 1 < m
        && grid[i - 1][j + 1] == 'M'
        && grid[i + 1][j - 1] == 'S';

    let backward_diagonal2 =  i >= 1
        && i + 1 < n
        && j >= 1
        && j + 1 < m
        && grid[i - 1][j + 1] == 'S'
        && grid[i + 1][j - 1] == 'M';

    println!("f1: {}, b1: {}, f2: {}, b2: {}", forward_diagonal1, backward_diagonal1, forward_diagonal2, backward_diagonal2);

    if (forward_diagonal1 || backward_diagonal1) && (forward_diagonal2 || backward_diagonal2) {
        positions.push((i, j));
    }
}

fn solve(grid: &Vec<Vec<char>>) -> usize {
    let mut positions: Vec<(usize, usize)> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' {
                check_xmas_pattern(grid, i, j, &mut positions);
            }
        }
    }

    positions.len()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    println!("File length: {}", content.len());
    let grid = parse_input(&content)?;
    println!("Grid dimensions: {}x{}", grid.len(), grid[0].len());

    println!("Solution: {}", solve(&grid));

    Ok(())
}
