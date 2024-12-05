use std::fs;
use std::io::Error;

fn parse_input(content: &str) -> Result<Vec<Vec<char>>, Error> {
    Ok(content
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn check(
    grid: &Vec<Vec<char>>,
    word: &str,
    mut i: i32,
    mut j: i32,
    i_sum: i32,
    j_sum: i32,
    positions: &mut Vec<((i32, i32), (i32, i32))>,
) {
    let n: i32 = grid.len().try_into().unwrap();
    let m: i32 = grid[0].len().try_into().unwrap();
    let word_chars: Vec<char> = word.chars().collect();

    let mut word_idx = 0;

    // Keep track of starting position
    let start_i = i;
    let start_j = j;

    while i < n && j < m && i >= 0 && j >= 0 && word_idx < word_chars.len() && word_chars[word_idx] == grid[i as usize][j as usize] {
        word_idx += 1;
        i += i_sum;
        j += j_sum;
    }

    if word_idx == word_chars.len() {
        positions.push((
            (start_i, start_j),
            ((i - i_sum), (j - j_sum)),
        ));
    }
}

fn solve(grid: &Vec<Vec<char>>) -> usize {
    let word = "XMAS";
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let n = grid.len();
    let m = grid[0].len();

    let mut positions: Vec<((i32, i32), (i32, i32))> = vec![];

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'X' {
                for (i_sum, j_sum) in directions {
                    check(grid, word, i.try_into().unwrap(), j.try_into().unwrap(), i_sum, j_sum, &mut positions);
                }
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
