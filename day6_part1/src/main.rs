use std::fs;
use std::collections::HashSet;
use std::io::Error;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(content: &str) -> Result<Vec<Vec<char>>, Error> {
    Ok(content
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn solve(grid: &Vec<Vec<char>>) -> usize {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let n = grid.len();
    let m = grid[0].len();

    let mut i = 0;
    let mut j = 0;
    let mut current_dir = None;

    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
    ];

    'outer: for x in 0..n {
        for y in 0..m {
            match grid[x][y] {
                '^' => {
                    i = x;
                    j = y;
                    current_dir = Some(Direction::Up);
                    break 'outer;
                },
                '>' => {
                    i = x;
                    j = y;
                    current_dir = Some(Direction::Right);
                    break 'outer;
                },
                'v' => {
                    i = x;
                    j = y;
                    current_dir = Some(Direction::Down);
                    break 'outer;
                },
                '<' => {
                    i = x;
                    j = y;
                    current_dir = Some(Direction::Left);
                    break 'outer;
                },
                _ => {},
            }
        }
    }

    println!("init dir: {:?}", current_dir);
    println!("init dir: {} {}", i, j);
    
    if current_dir.is_none() {
        current_dir = Some(Direction::Right);
    }

    let mut current_dir_index = match current_dir.unwrap() {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    println!("current_dir_idx: {}", current_dir_index);

    while i < n && j < m {
        positions.insert((i, j));
        let (di, dj) = directions[current_dir_index];

        let new_i = i as isize + di;
        let new_j = j as isize + dj;

        if new_i < 0 || new_i >= n as isize || new_j < 0 || new_j >= m as isize {
            break;
        }

        let next_i = new_i as usize;
        let next_j = new_j as usize;

        if grid[next_i][next_j] == '#' {
            current_dir_index = (current_dir_index + 1) % directions.len();
        } else {
            i = next_i;
            j = next_j;
        }
    }

    println!("{:#?}", positions);

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
