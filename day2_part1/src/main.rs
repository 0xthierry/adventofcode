use std::{fs, io::{self, Error}};

fn parse_input(content: &str) -> Result<Vec<Vec<u32>>, Error> {
    let mut items: Vec<Vec<u32>> = vec![];

    for line in content.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let mut aux = vec![];

        for part in parts {
            aux.push(part.parse::<u32>().map_err(|_| Error::new(io::ErrorKind::InvalidData, "Invalid number"))?);
        }

        items.push(aux);
    }

    Ok(items)
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

fn get_total_safe_reports(lines: &Vec<Vec<u32>>) -> u32 {
    let mut total_safe = 0;

    for line in lines {
        let direction: Direction = get_direction(&line[0], &line[1]);
        let mut safe = true;

        for i in 0..line.len() - 1 {
            let current_direction = get_direction(&line[i], &line[i+1]);

            if current_direction == Direction::Neutral || current_direction != direction || line[i].abs_diff(line[i+1]) > 3 {
                safe = false;
                break
            }
        }

        if safe {
            total_safe += 1
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
