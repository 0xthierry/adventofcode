use std::{fs, io::{self, Error}};

fn parse_input(content: &str) -> Result<Vec<Vec<u32>>, Error> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<u32>().map_err(|_| Error::new(io::ErrorKind::InvalidData, "Invalid number")))
                .collect()
        })
        .collect()
}

#[derive(PartialEq)]
enum Direction {
    Up = 1,
    Down = -1,
    Neutral = 0
}

fn get_direction(a: &u32, b: &u32) -> Direction {
    if a > b {
        Direction::Up
    } else if a < b {
        Direction::Down
    } else {
        Direction::Neutral
    }
}

fn is_safe_report(line: &[u32]) -> bool {
    if line.len() <= 2 {
        return true
    }

    let mut direction = get_direction(&line[0], &line[1]);

    for window in line.windows(2) {
        let current_direction: Direction = get_direction(&window[0], &window[1]);

        if current_direction == Direction::Neutral
            || current_direction != direction
            || window[0].abs_diff(window[1]) > 3
        {
            return false;
        }

        direction = current_direction;
    }

    true
}

fn get_total_safe_reports(lines: &Vec<Vec<u32>>) -> u32 {
    let mut total_safe = 0;

    for line in lines {
        if is_safe_report(&line) {
            total_safe += 1
        } else {
            for i in 0..line.len() {
                if is_safe_report(&[&line[..i], &line[i + 1..]].concat()) {
                    total_safe += 1;
                    break
                }
            }
        }
    }

    total_safe
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    let nums: Vec<Vec<u32>> = parse_input(&content)?;

    let total_safe_reports = get_total_safe_reports(&nums);

    println!("Total Safe: {:?}", total_safe_reports);

    Ok(())
}
