use std::fs;
use std::io::{self, Error};

fn get_total_distance(left: &mut[u32], right: &mut[u32]) -> u32 {
    left.sort();
    right.sort();

    left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

fn parse_input(content: &str) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in content.lines() {
        if let Some((l, r)) = line.split_once("   ") {
            left.push(l.trim().parse::<u32>().map_err(|_| Error::new(io::ErrorKind::InvalidData, "Invalid left number"))?);
            right.push(r.trim().parse::<u32>().map_err(|_| Error::new(io::ErrorKind::InvalidData, "Invalid right number"))?);
        }
    }

    Ok((left, right))
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    let (mut left, mut right) = parse_input(&content)?;

    println!("Total distance: {}", get_total_distance(&mut left, &mut right));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_distance() {
        let mut left = vec![10, 1, 2, 7, 3];
        let mut right = vec![5,3,9,18, 19];
    
        assert_eq!(get_total_distance(&mut left, &mut right), 31);
    }

    #[test]
    fn test_parse_input() {
        let content = "10   5\n2   3\n7   9\n";
        let (left, right) = parse_input(content).unwrap();
        assert_eq!(left, vec![10, 2, 7]);
        assert_eq!(right, vec![5, 3, 9]);
    }

    #[test]
    fn test_empty_input() {
        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];
        assert_eq!(get_total_distance(&mut left, &mut right), 0);
    }

    #[test]
    fn test_parse_invalid_input() {
        let content = "10   five\n2   3\n";
        let result = parse_input(content);
        assert!(result.is_err());
    }

}