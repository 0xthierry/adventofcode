use std::collections::HashMap;
use std::fs;
use std::io::{self, Error};

fn get_similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut right_hashmap: HashMap<u32, u32> = HashMap::new();

    for r in right {
        *right_hashmap.entry(*r).or_insert(0) += 1;
    }

    left.iter().map(|l| right_hashmap.get(&l).unwrap_or(&0) * l).sum()
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
    let (left, right) = parse_input(&content)?;

    println!("Similarity Score: {}", get_similarity_score(&left, &right));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_similarity_score() {
        let mut left = vec![10, 1, 2, 7, 3, 1];
        let mut right = vec![5,3,9,18, 19,3];
    
        assert_eq!(get_similarity_score(&mut left, &mut right), 6);
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
        assert_eq!(get_similarity_score(&mut left, &mut right), 0);
    }

    #[test]
    fn test_parse_invalid_input() {
        let content = "10   five\n2   3\n";
        let result = parse_input(content);
        assert!(result.is_err());
    }

}