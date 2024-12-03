use std::{io::{self, Error}, fs};
use regex::Regex;


fn parse_input(content: &str) -> Result<Vec<(u32, u32)>, Error> {
    Ok(
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(content)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (
                a.parse::<u32>().unwrap(), 
                b.parse::<u32>().unwrap()
            )
        })
        .collect()
    )
}

fn sum_mult(items: &[(u32, u32)]) -> u32 {
    items.iter().map(|(a, b)| a * b).sum()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&content)?;

    println!("Sum: {:?}", sum_mult(&parsed_input));

    Ok(())
}
