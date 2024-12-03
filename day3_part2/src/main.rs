use std::{fs, io::Error};
use regex::Regex;


fn parse_input(content: &str) -> Result<Vec<(u32, u32)>, Error> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();

    let mut results: Vec<(u32, u32)> = Vec::new();
    let mut enabled = true;

    for caps in re.captures_iter(&content) {
        match caps.get(0).map(|m| m.as_str()) {
            Some("don't()") => enabled = false,
            Some("do()") => enabled = true,
            _ => {
                if enabled {
                    if let (Some(a), Some(b)) = (caps.get(2), caps.get(3)) {
                        if let (Ok(a), Ok(b)) = (a.as_str().parse(), b.as_str().parse()) {
                            results.push((a, b))
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}

fn sum_mult(items: &[(u32, u32)]) -> u32 {
    items.iter().fold(0, |acc, (a, b)| acc + a * b)
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&content)?;

    println!("Sum: {:?}", sum_mult(&parsed_input));

    Ok(())
}
