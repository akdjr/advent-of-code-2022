use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut total_priority: u32 = 0;

    for line in lines {
        if let Ok(lp) = line {
            if lp == "" {
                continue;
            }

            let mid = lp.chars().count() / 2;
            let left = &lp[0..mid];
            let right = &lp[mid..];

            let mut characters: HashMap<char, bool> = HashMap::new();
            let mut duplicates: HashMap<char, bool> = HashMap::new();

            let first: Vec<char> = left.chars().collect();
            let second: Vec<char> = right.chars().collect();

            let mut shared: Vec<char> = vec![];

            for ch in &first {
                characters.entry(*ch).or_insert(true);
            }

            for ch in &second {
                if characters.contains_key(ch) {
                    if !duplicates.contains_key(ch) {
                        shared.push(*ch);
                        duplicates.entry(*ch).or_insert(true);
                    }
                }
            }

            for ch in shared {
                let ascii = ch as u8;
                total_priority += match ascii {
                    97..=122 => ascii - 96,
                    65..=90 => ascii - 38,
                    _ => 0
                } as u32;
            }
        }
    }

    println!("Sum of priorities: {}", total_priority);

    Ok(())
}
