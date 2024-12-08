use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::Error;

fn parse_input(content: &str) -> Result<(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>), Error> {
    let mut rules = HashMap::new();
    let mut updates = vec![];

    let sections: Vec<&str> = content.split("\n\n").collect();
    match &sections[..] {
        [page_ordering_rules_content, page_updates_content] => {

        for line in page_ordering_rules_content.lines() {
            let pair: Vec<&str> = line.split("|").collect();
            let pair_a = &pair[0].parse::<u32>().unwrap();
            let pair_b = &pair[1].parse::<u32>().unwrap();

            if !rules.contains_key(pair_a) {
                rules.insert(*pair_a, HashSet::new());
            }

            rules.get_mut(pair_a).unwrap().insert(*pair_b);
        }

        for line in page_updates_content.lines() {
            let nums_content: Vec<&str> = line.split(",").collect();
            let mut aux = vec![];

            for num_content in nums_content {
                aux.push(num_content.parse::<u32>().unwrap())
            }

            updates.push(aux);
        }
        }
        _ => {
            println!("The content does not have exactly two sections separated by a blank line.");
        }
    }

    Ok((rules, updates))
}

fn solve(rules: &HashMap<u32, HashSet<u32>>, updates: &mut Vec<Vec<u32>>) -> u32 {
    let mut total = 0;

    for update in updates {
        let mut ok = true;

        for (num_idx, num) in update.iter().enumerate() {
            if let Some(set) = rules.get(num) {
                for set_n in set {
                    if let Some(set_idx) = update.iter().position(|x| x == set_n) {
                        if num_idx > set_idx {
                            ok = false;
                            break;
                        }
                    }
                }
            }

            if !ok {
                break;
            }
        }

        if !ok {
            update.sort_by(|a, b| {
                if rules.get(a).unwrap().contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid_idx: usize = update.len() / 2;
            total += update[mid_idx];
        }
    }

    total
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("input.txt")?;
    println!("File length: {}", content.len());
    let (rules, mut updates) = parse_input(&content)?;
    println!("Solution: {}", solve(&rules, &mut updates));
    Ok(())
}
